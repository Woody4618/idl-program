mod commands;
mod config;
mod cli;
mod codama_sdk;
use codama_sdk::programs::METADATA_PROGRAM_ID;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let matches = cli::get_clap_app().get_matches();

    match matches.subcommand() {
        Some(("idl", idl_matches)) => {
            match idl_matches.subcommand() {
                Some(("upload", upload_matches)) => {
                    let idl_path = upload_matches.get_one::<String>("idl-path").unwrap();
                    let program_id = upload_matches.get_one::<String>("program-id").unwrap();
                    let keypair_path = upload_matches.get_one::<String>("keypair");
                    let priority_fees_per_cu = upload_matches
                        .get_one::<String>("priority-fees-per-cu")
                        .unwrap_or(&"0".to_string())
                        .parse::<u64>()
                        .map_err(|_| anyhow!("Invalid priority fees value"))?;
                    let rpc_url = upload_matches.get_one::<String>("rpc-url");
                    
                    commands::idl::upload_idl_by_file_path(idl_path, program_id, keypair_path.map(|s| s.as_str()), priority_fees_per_cu, rpc_url.map(|s| s.as_str()))
                }
                Some(("upload-url", upload_matches)) => {
                    let url = upload_matches.get_one::<String>("url").unwrap();
                    let program_id = upload_matches.get_one::<String>("program-id").unwrap();
                    let keypair_path = upload_matches.get_one::<String>("keypair");
                    let priority_fees_per_cu = upload_matches
                        .get_one::<String>("priority-fees-per-cu")
                        .unwrap_or(&"0".to_string())
                        .parse::<u64>()
                        .map_err(|_| anyhow!("Invalid priority fees value"))?;
                    let rpc_url = upload_matches.get_one::<String>("rpc-url");
                    
                    commands::idl::upload_idl_by_url(url, program_id, keypair_path.map(|s| s.as_str()), priority_fees_per_cu, rpc_url.map(|s| s.as_str()))
                }
                Some(("download", download_matches)) => {
                    let program_id = download_matches.get_one::<String>("program-id").unwrap();
                    let output_path = download_matches
                        .get_one::<String>("output")
                        .map(|s| s.as_str())
                        .unwrap_or("idl.json");
                    let rpc_url = download_matches.get_one::<String>("rpc-url");
                    commands::idl::download_idl_to_file(program_id, output_path, rpc_url.map(|s| s.as_str()))
                }
                _ => {
                    println!("Unknown IDL command. Use --help to see available commands");
                    Ok(())
                }
            }
        }
        Some(("metadata", metadata_matches)) => {
            match metadata_matches.subcommand() {
                Some(("upload", upload_matches)) => {
                    let metadata_path = upload_matches.get_one::<String>("metadata-path").unwrap();
                    let program_id = upload_matches.get_one::<String>("program-id").unwrap();
                    let keypair_path = upload_matches.get_one::<String>("keypair");
                    let priority_fees_per_cu = upload_matches
                        .get_one::<String>("priority-fees-per-cu")
                        .unwrap_or(&"0".to_string())
                        .parse::<u64>()
                        .map_err(|_| anyhow!("Invalid priority fees value"))?;
                    let rpc_url = upload_matches.get_one::<String>("rpc-url");
                    
                    commands::idl::upload_metadata_by_file_path(metadata_path, program_id, keypair_path.map(|s| s.as_str()), priority_fees_per_cu, rpc_url.map(|s| s.as_str()))
                }
                Some(("upload-url", upload_matches)) => {
                    let url = upload_matches.get_one::<String>("url").unwrap();
                    let program_id = upload_matches.get_one::<String>("program-id").unwrap();
                    let keypair_path = upload_matches.get_one::<String>("keypair");
                    let priority_fees_per_cu = upload_matches
                        .get_one::<String>("priority-fees-per-cu")
                        .unwrap_or(&"0".to_string())
                        .parse::<u64>()
                        .map_err(|_| anyhow!("Invalid priority fees value"))?;
                    let rpc_url = upload_matches.get_one::<String>("rpc-url");
                    
                    commands::idl::upload_metadata_by_url(url, program_id, keypair_path.map(|s| s.as_str()), priority_fees_per_cu, rpc_url.map(|s| s.as_str()))
                }
                Some(("download", download_matches)) => {
                    let program_id = download_matches.get_one::<String>("program-id").unwrap();
                    let output_path = download_matches
                        .get_one::<String>("output")
                        .map(|s| s.as_str())
                        .unwrap_or("metadata.json");
                    let rpc_url = download_matches.get_one::<String>("rpc-url");
                    commands::idl::download_metadata_to_file(program_id, output_path, rpc_url.map(|s| s.as_str()))
                }
                _ => {
                    println!("Unknown metadata command. Use --help to see available commands");
                    Ok(())
                }
            }
        }
        _ => {
            println!("Please use --help to see available commands");
            Ok(())
        }
    }
}
