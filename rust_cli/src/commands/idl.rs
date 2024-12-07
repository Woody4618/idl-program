use anyhow::{Result, anyhow};
use flate2::write::GzEncoder;
use flate2::Compression;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
    signer::Signer,
    compute_budget::ComputeBudgetInstruction,
};
use std::io::Write;
use std::io::Read;
use std::{str::FromStr, fs};
use crate::codama_sdk::instructions::{Resize, ResizeInstructionArgs};
use crate::config::get_user_config;
use crate::codama_sdk::{
    instructions::{
        initialize::{Initialize, InitializeInstructionArgs},
        create_buffer::CreateBufferBuilder,
        write_buffer::{WriteBuffer, WriteBufferInstructionArgs},
        set_buffer::{SetBuffer, SetBufferInstructionArgs},
        close_buffer::CloseBuffer,
    },
    programs::METADATA_PROGRAM_ID,
};
use reqwest::blocking::Client;
use flate2::read::GzDecoder;

const IDL_SEED: &str = "idl";
const METADATA_SEED: &str = "metadata";
const METADATA_OFFSET: usize = 44;
const CHUNK_SIZE: u16 = 900;
const MAX_RESIZE_STEP: u16 = 10240;


//Upload functions
pub fn upload_idl_by_file_path(
    file_path: &str,
    program_id: &str,
    keypair_path: Option<&str>,
    priority_fees_per_cu: u64,
    rpc_url: Option<&str>,
) -> Result<()> {
    upload_data_by_file_path(file_path, program_id, keypair_path, priority_fees_per_cu, IDL_SEED, rpc_url)
}

pub fn upload_metadata_by_file_path(
    metadata_path: &str,
    program_id: &str,
    keypair_path: Option<&str>,
    priority_fees_per_cu: u64,
    rpc_url: Option<&str>,
) -> Result<()> {
    upload_data_by_file_path(metadata_path, program_id, keypair_path, priority_fees_per_cu, METADATA_SEED, rpc_url)
}

pub fn upload_idl_by_url(
    url: &str,
    program_id: &str,
    keypair_path: Option<&str>,
    priority_fees_per_cu: u64,
    rpc_url: Option<&str>,
) -> Result<()> {
    let upload_data = fetch_data_from_url(url)?;
    upload_data_from_bytes(upload_data, program_id, keypair_path, priority_fees_per_cu, IDL_SEED, rpc_url)
}

pub fn upload_metadata_by_url(
    url: &str,
    program_id: &str,
    keypair_path: Option<&str>,
    priority_fees_per_cu: u64,
    rpc_url: Option<&str>,
) -> Result<()> {
    let upload_data = fetch_data_from_url(url)?;
    upload_data_from_bytes(upload_data, program_id, keypair_path, priority_fees_per_cu, METADATA_SEED, rpc_url)
}

fn upload_data_by_file_path(
    file_path: &str,
    program_id: &str,
    keypair_path: Option<&str>,
    priority_fees_per_cu: u64,
    seed: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    let upload_data = fs::read(file_path)
        .map_err(|e| anyhow!("Failed to read file: {}", e))?;
    upload_data_from_bytes(upload_data, program_id, keypair_path, priority_fees_per_cu, seed, rpc_url)
}

// New core function that handles the actual upload
fn upload_data_from_bytes(
    upload_data: Vec<u8>,
    program_id: &str,
    keypair_path: Option<&str>,
    priority_fees_per_cu: u64,
    seed: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    // Get signer and RPC client
    let (signer, rpc_client) = if let Some(path) = keypair_path {
        let keypair = solana_sdk::signature::read_keypair_file(path)
            .map_err(|e| anyhow!("Failed to read keypair file: {}", e))?;
        let rpc_client = get_user_config(rpc_url)
            .map(|(_, client)| client)
            .map_err(|e| anyhow!("Failed to get RPC client: {}", e))?;
        (keypair, rpc_client)
    } else {
        get_user_config(rpc_url)
            .map_err(|e| anyhow!("Failed to get user config: {}", e))?
    };

    // Parse program ID
    let program_pubkey = Pubkey::from_str(program_id)
        .map_err(|e| anyhow!("Invalid program ID: {}", e))?;

    println!("Signer: {}", signer.pubkey());
    println!("&program_pubkey: {}", &program_pubkey);

    // Get account address
    let metadata_address = get_metadata_address(seed, &program_pubkey);

    // Initialize account
    initialize(&program_pubkey, &signer, priority_fees_per_cu, seed, &rpc_client)?;

    // Create buffer
    let (compressed_data, buffer_keypair) = create_buffer(upload_data, &rpc_client, &signer, priority_fees_per_cu)?;

    // Write buffer
    write_buffer(compressed_data, &buffer_keypair, &signer, &rpc_client, priority_fees_per_cu)?;

    // Set and close buffer
    set_and_close_buffer(rpc_client, metadata_address, &buffer_keypair, priority_fees_per_cu, &signer, program_pubkey, seed)?;
    
    Ok(())
}

//Sub-transactions
fn initialize(
    program_pubkey: &Pubkey,
    signer: &Keypair,
    priority_fees_per_cu: u64,
    seed: &str,
    rpc_client: &solana_client::rpc_client::RpcClient,
) -> Result<()> {

    let metadata_address = get_metadata_address(seed, program_pubkey);

    // Check if account already exists
    if let Ok(_) = rpc_client.get_account(&metadata_address) { 
        println!("Data account already exists");
        return Ok(());
    }

    println!("Initializing Data with seed: {}", seed);

    // Get program data address
    let (program_data_address, _) = Pubkey::find_program_address(
        &[program_pubkey.as_ref()],
        &solana_sdk::bpf_loader_upgradeable::ID,
    );

    // Create initialize instruction using the generated code
    let accounts = Initialize {
        idl: metadata_address,
        signer: signer.pubkey(),
        system_program: solana_sdk::system_program::ID,
        program_id: *program_pubkey,
        program_data: program_data_address,
    };

    let args = InitializeInstructionArgs {
        seed: seed.to_string(),
    };

    let ix = accounts.instruction(args);

    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(|e| anyhow!("Failed to get recent blockhash: {}", e))?;

    let transaction = simulate_and_create_transaction(
        &rpc_client,
        vec![ix],
        &[signer],
        priority_fees_per_cu,
        recent_blockhash,
    )?;

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, CommitmentConfig::confirmed())
        .map_err(|e| anyhow!("Failed to send transaction: {}", e))?;

    println!("Data initialized successfully!");
    println!("Signature: {}", signature);
    println!("Newly created metadata PDA address from seed {seed} : {metadata_address}");
    
    Ok(())
}

fn create_buffer(
    idl_data: Vec<u8>, 
    rpc_client: &solana_client::rpc_client::RpcClient, 
    signer: &Keypair, 
    priority_fees_per_cu: u64
) -> Result<(Vec<u8>, Keypair), anyhow::Error> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&idl_data)?;
    let compressed_data = encoder.finish()?;
    let data_len = compressed_data.len() + METADATA_OFFSET;
    let buffer_keypair = Keypair::new();
    let rent = rpc_client.get_minimum_balance_for_rent_exemption(data_len)
        .map_err(|e| anyhow!("Failed to get rent: {}", e))?;
    let create_account_ix = solana_sdk::system_instruction::create_account(
        &signer.pubkey(),
        &buffer_keypair.pubkey(),
        rent,
        data_len as u64,
        &METADATA_PROGRAM_ID,
    );
    let create_buffer = CreateBufferBuilder::new()
        .buffer(buffer_keypair.pubkey())
        .authority(signer.pubkey())
        .instruction();
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(|e| anyhow!("Failed to get recent blockhash: {}", e))?;

    

    let transaction = simulate_and_create_transaction(
            &rpc_client,
            vec![create_account_ix, create_buffer],
            &[signer, &buffer_keypair],
            priority_fees_per_cu,
            recent_blockhash,
    )?;

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(&transaction, CommitmentConfig::confirmed())
        .map_err(|e| anyhow!("Failed to send transaction: {}", e))?;
    println!("Buffer account created successfully: {}", buffer_keypair.pubkey());
    println!("Signature: {}", signature);
    Ok((compressed_data, buffer_keypair))
} 

fn write_buffer(
    compressed_data: Vec<u8>, 
    buffer_keypair: &Keypair, 
    signer: &Keypair, 
    rpc_client: &solana_client::rpc_client::RpcClient, 
    priority_fees_per_cu: u64
) -> Result<(), anyhow::Error> {
    let mut offset = 0;
    while offset < compressed_data.len() {
        let chunk_end = std::cmp::min(offset + CHUNK_SIZE as usize, compressed_data.len());
        let chunk = &compressed_data[offset..chunk_end];
    
        let write_buffer = WriteBuffer {
            buffer: buffer_keypair.pubkey(),
            signer: signer.pubkey(),
            system_program: solana_sdk::system_program::ID,
        };
    
        let write_args = WriteBufferInstructionArgs {
            idl_data: chunk.to_vec(),
        };
    
        let write_ix = write_buffer.instruction(write_args);
    
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .map_err(|e| anyhow!("Failed to get write buffer blockhash: {}", e))?;


        let write_transaction = simulate_and_create_transaction(
            &rpc_client,
            vec![write_ix],
            &[signer],
            priority_fees_per_cu,
            recent_blockhash,
        )?;
    
        let write_signature = rpc_client
            .send_and_confirm_transaction_with_spinner_and_commitment(
                &write_transaction,
                CommitmentConfig::confirmed()
            )
            .map_err(|e| anyhow!("Failed to send write buffer transaction: {}", e))?;
    
        println!("Wrote chunk {} to {}, signature: {}", offset, chunk_end, write_signature);
    
        offset = chunk_end;
    }
    println!("All buffer chunks written successfully!");

    Ok(())
}

fn set_and_close_buffer(
    rpc_client: solana_client::rpc_client::RpcClient, 
    idl_address: Pubkey, 
    buffer_keypair: &Keypair, 
    priority_fees_per_cu: u64, 
    signer: &Keypair, 
    program_pubkey: Pubkey,
    seed: &str,
) -> Result<()> {
    let idl_account_info = get_account_with_retry(&rpc_client, &idl_address, 10, 2)?;
    let buffer_account_info = get_account_with_retry(&rpc_client, &buffer_keypair.pubkey(), 10, 2)?;

    let idl_account_size = idl_account_info.data.len();
    let buffer_account_size = buffer_account_info.data.len();
    
    println!("Data account size: {}, Buffer account size: {}", idl_account_size, buffer_account_size);
    let mut instructions = vec![];
    if buffer_account_size < idl_account_size {
        // Shrink IDL account to buffer size
        let resize = Resize {
            idl: idl_address,
            signer: signer.pubkey(),
            system_program: solana_sdk::system_program::ID,
            program_id: program_pubkey,
        }.instruction(ResizeInstructionArgs {
            len: buffer_account_size as u16,
            seed: seed.to_string(),
        });
        instructions.push(resize);
    } else {
        let mut current_size = idl_account_size;
        let target_size = buffer_account_size;

        while current_size < target_size {
            let next_size = std::cmp::min(
                current_size + MAX_RESIZE_STEP as usize,
                target_size
            );

            let resize = Resize {
                idl: idl_address,
                signer: signer.pubkey(),
                system_program: solana_sdk::system_program::ID,
                program_id: program_pubkey,
            }.instruction(ResizeInstructionArgs {
                len: next_size as u16,
                seed: seed.to_string(),
            });
            instructions.push(resize);

            println!("Adding resize instruction to size {}", next_size);
            current_size = next_size;
        }
    }
    let set_buffer = SetBuffer {
        buffer: buffer_keypair.pubkey(),
        idl: idl_address,
        authority: signer.pubkey(),
        program_id: program_pubkey,
    }.instruction(SetBufferInstructionArgs {
        seed: seed.to_string(),
    });
    println!("Set buffer instruction");
    instructions.push(set_buffer);
    let close_buffer = CloseBuffer {
        buffer: buffer_keypair.pubkey(),
        authority: signer.pubkey(),
    }.instruction();
    instructions.push(close_buffer);
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .map_err(|e| anyhow!("Failed to get recent blockhash: {}", e))?;
    
    let transaction = simulate_and_create_transaction(
        &rpc_client,
        instructions,
        &[signer],
        priority_fees_per_cu,
        recent_blockhash,
    )?;

    let signature = rpc_client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &transaction,
            CommitmentConfig::confirmed()
        )
        .map_err(|e| anyhow!("Failed to send set buffer transaction: {}", e))?;
    println!("Buffer set and closed successfully! Signature: {}", signature);
    Ok(())
}


//Download functions
pub fn download_idl_to_file(
    program_id: &str,
    output_path: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    download_data_to_file(program_id, output_path, IDL_SEED, rpc_url)
}

pub fn download_metadata_to_file(
    program_id: &str,
    output_path: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    download_data_to_file(program_id, output_path, METADATA_SEED, rpc_url)
}

pub fn download_data_to_file(
    program_id: &str,
    output_path: &str,
    seed: &str,
    rpc_url: Option<&str>,
) -> Result<()> {
    let (_, rpc_client) = get_user_config(rpc_url)?;
    
    // Parse program ID
    let program_pubkey = Pubkey::from_str(program_id)
        .map_err(|e| anyhow!("Invalid program ID: {}", e))?;

    // Get account address
    let metadata_address = get_metadata_address(seed, &program_pubkey);

    // Get account data
    let account = rpc_client.get_account(&metadata_address)
        .map_err(|e| anyhow!("Failed to get account data: {}", e))?;

    // Get data length from account (4 bytes at offset 40)
    let data_len_bytes = account.data[METADATA_OFFSET - 4..METADATA_OFFSET].try_into()
        .map_err(|_| anyhow!("Failed to read data length"))?;
    let data_length = u32::from_le_bytes(data_len_bytes);

    // Get compressed data
    let compressed_data = &account.data[METADATA_OFFSET..METADATA_OFFSET + data_length as usize];

    // Decompress data
    let mut decoder = GzDecoder::new(compressed_data);
    let mut download_data = Vec::new();
    decoder.read_to_end(&mut download_data)?;

    // Write to file
    fs::write(output_path, download_data)
        .map_err(|e| anyhow!("Failed to write to file: {}", e))?;

    println!("Successfully downloaded and saved data to {}", output_path);
    Ok(())
}


// Utility functions
fn get_account_with_retry(
    rpc_client: &solana_client::rpc_client::RpcClient,
    pubkey: &Pubkey,
    max_attempts: u32,
    retry_delay_secs: u64,
) -> Result<solana_sdk::account::Account> {
    let mut attempts = 0;
    loop {
        match rpc_client.get_account(pubkey) {
            Ok(account) => return Ok(account),
            Err(e) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(anyhow!("Failed to get account {} after {} attempts: {}", pubkey, max_attempts, e));
                }
                println!("Failed to get account {}, retrying in {} seconds... (attempt {}/{})", 
                    pubkey, retry_delay_secs, attempts, max_attempts);
                std::thread::sleep(std::time::Duration::from_secs(retry_delay_secs));
            }
        }
    }
}

fn get_metadata_address(seed: &str, program_pubkey: &Pubkey) -> Pubkey {
    let (metadata_address, _) = Pubkey::find_program_address(
        &[seed.as_bytes(), program_pubkey.as_ref()],
        &METADATA_PROGRAM_ID,
    );
    metadata_address
}

fn fetch_data_from_url(
    url: &str
) -> Result<Vec<u8>> {
    let client = Client::new();
    let response = client.get(url)
        .send()
        .map_err(|e| anyhow!("Failed to fetch from URL: {}", e))?;
    
    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch URL: HTTP {}", response.status()));
    }

    response.bytes()
        .map_err(|e| anyhow!("Failed to read response body: {}", e))
        .map(|b| b.to_vec())
}

// Add to utility functions section
fn simulate_and_create_transaction(
    rpc_client: &solana_client::rpc_client::RpcClient,
    instructions: Vec<solana_sdk::instruction::Instruction>,
    signers: &[&Keypair],
    priority_fees_per_cu: u64,
    recent_blockhash: solana_sdk::hash::Hash,
) -> Result<Transaction> {
    // Add compute budget instructions
    let mut simulation_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_price(priority_fees_per_cu),
        ComputeBudgetInstruction::set_compute_unit_limit(200000),
    ];
    simulation_instructions.extend(instructions.clone());

    // Create and simulate transaction
    let transaction_to_simulate = Transaction::new_signed_with_payer(
        &simulation_instructions,
        Some(&signers[0].pubkey()),
        signers,
        recent_blockhash,
    );

    let simulation = rpc_client.simulate_transaction(&transaction_to_simulate)
        .map_err(|e| anyhow!("Failed to simulate transaction: {}", e))?;

    let units_consumed: u32 = simulation.value.units_consumed.unwrap_or(0) as u32;
    println!("Compute units from simulation: {}", units_consumed);

    // Create final transaction with adjusted compute limit
    let mut final_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_price(priority_fees_per_cu),
        ComputeBudgetInstruction::set_compute_unit_limit(units_consumed + 200),
    ];
    final_instructions.extend(instructions);

    Ok(Transaction::new_signed_with_payer(
        &final_instructions,
        Some(&signers[0].pubkey()),
        signers,
        recent_blockhash,
    ))
}




    