use anchor_lang::idl::ERASED_AUTHORITY;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;

declare_id!("pmetaypqG6SiB47xMigYVMAkuHDWeSDXcv3zzDrJJvA");

#[program]
pub mod metadata_program {

    use anchor_lang::solana_program::{bpf_loader, bpf_loader_deprecated};

    use super::*;

    #[error_code]
    pub enum MyError {
        #[msg("Only the program upgrade authority can initialize the associated program metadata account")]
        WrongAuthority,        
        #[msg("The program account is not executable")]
        NotExecutable,
        #[msg("The program account is not a program. Not owned by the BPF loader")]
        NotAProgram,
        #[msg("The program account should not be a program data account")]
        ShouldBeProgramAccount,
    }

    pub fn initialize(ctx: Context<Initialize>, _seed: String) -> Result<()> {      
        msg!("Signer {:?}!", ctx.accounts.signer.key);              
        msg!("Authority {:?}!", ctx.accounts.program_data.upgrade_authority_address);              

        if ctx.accounts.program_id.owner.key() != bpf_loader_upgradeable::ID 
        && ctx.accounts.program_id.owner.key() != bpf_loader::ID 
        && ctx.accounts.program_id.owner.key() != bpf_loader_deprecated::ID {
            return Err(MyError::NotAProgram.into());
        }
        
        if !ctx.accounts.program_id.executable {
            return Err(MyError::NotExecutable.into());
        }
    
        // Borrow the program's account data
        let mut program_borrowed_data: &[u8] = &ctx.accounts.program_id.try_borrow_data()?;

        // Deserialize the UpgradeableLoaderState from the program account data
        let upgradable_loader_state =
            UpgradeableLoaderState::try_deserialize_unchecked(&mut program_borrowed_data)?;

        match upgradable_loader_state {
            UpgradeableLoaderState::Uninitialized
            | UpgradeableLoaderState::Buffer {
                authority_address: _,
            }
            | UpgradeableLoaderState::ProgramData {
                slot: _,
                upgrade_authority_address: _,
            } => {
                return err!(MyError::ShouldBeProgramAccount);
            }
            UpgradeableLoaderState::Program {
                programdata_address: program_data_address,
            } => {
                // Print out the program data address
                msg!("Program Data Address: {:?}", program_data_address);

                // Ensure the program data address matches the expected value
                if program_data_address != ctx.accounts.program_data.key() {
                    return err!(MyError::WrongAuthority);
                }
            }
        }

        // When all is good create PDA and save authority for later upgrades.
        //ctx.accounts.pda.load_init()?.authority = *ctx.accounts.signer.key;
        Ok(())
    }

    pub fn initialize_with_signer_seed(ctx: Context<InitializeWithSignerSeed>, _seed: String) -> Result<()> {      
        msg!("Signer {:?}!", ctx.accounts.signer.key);              
        msg!("Authority {:?}!", ctx.accounts.program_data.upgrade_authority_address);              

        if ctx.accounts.program_id.owner.key() != bpf_loader_upgradeable::ID 
        && ctx.accounts.program_id.owner.key() != bpf_loader::ID 
        && ctx.accounts.program_id.owner.key() != bpf_loader_deprecated::ID {
            return Err(MyError::NotAProgram.into());
        }
        
        if !ctx.accounts.program_id.executable {
            return Err(MyError::NotExecutable.into());
        }
    
        // Borrow the program's account data
        let mut program_borrowed_data: &[u8] = &ctx.accounts.program_id.try_borrow_data()?;

        // Deserialize the UpgradeableLoaderState from the program account data
        let upgradable_loader_state =
            UpgradeableLoaderState::try_deserialize_unchecked(&mut program_borrowed_data)?;

        match upgradable_loader_state {
            UpgradeableLoaderState::Uninitialized
            | UpgradeableLoaderState::Buffer {
                authority_address: _,
            }
            | UpgradeableLoaderState::ProgramData {
                slot: _,
                upgrade_authority_address: _,
            } => {
                return err!(MyError::ShouldBeProgramAccount);
            }
            UpgradeableLoaderState::Program {
                programdata_address: program_data_address,
            } => {
                // Print out the program data address
                msg!("Program Data Address: {:?}", program_data_address);

                // Ensure the program data address matches the expected value
                if program_data_address != ctx.accounts.program_data.key() {
                    return err!(MyError::WrongAuthority);
                }
            }
        }

        // When all is good create PDA and save authority for later upgrades.
        ctx.accounts.pda.load_init()?.authority = *ctx.accounts.signer.key;
        Ok(())
    }

    pub fn write_buffer(ctx: Context<WriteBuffer>, data: Vec<u8>) -> Result<()> {
        let account_info = ctx.accounts.buffer.as_ref();
        let mut account_data = account_info.try_borrow_mut_data()?;
        
        // Skip discriminator (8) + authority (32) + data_len (4)
        const DATA_OFFSET: usize = 8 + 32 + 4;
        
        // Write data
        let current_len = account_data[DATA_OFFSET-4..DATA_OFFSET]
            .try_into()
            .map(u32::from_le_bytes)
            .unwrap_or(0) as usize;
        
        let new_len = current_len + data.len();
        
        // Update data_len
        account_data[DATA_OFFSET-4..DATA_OFFSET]
            .copy_from_slice(&(new_len as u32).to_le_bytes());
        
        // Write new data
        account_data[DATA_OFFSET + current_len..DATA_OFFSET + new_len]
            .copy_from_slice(&data);
        
        Ok(())
    }

    pub fn create_buffer(ctx: Context<CreateBuffer>) -> Result<()> {
        //ctx.accounts.buffer.load_mut()?.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn close_buffer(_ctx: Context<CloseBuffer>) -> Result<()> {
        Ok(())
    }

    pub fn resize(_ctx: Context<Resize>, _len: u16, _seed: String) -> Result<()> {
        Ok(())
    }

    pub fn set_authority(ctx: Context<MetadataAccounts>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.pda.load_mut()?.authority = new_authority;
        Ok(())
    }

    pub fn set_buffer(ctx: Context<SetBuffer>, _seed: String) -> Result<()> {
        const DATA_OFFSET: usize = 8 + 32 + 4;  // Skip discriminator + authority + data_len
        
        // Get source data
        let buffer_info = ctx.accounts.buffer.as_ref();
        let buffer_data = buffer_info.try_borrow_data()?;
        let buffer_len = u32::from_le_bytes(buffer_data[DATA_OFFSET-4..DATA_OFFSET].try_into().unwrap());
        
        // Get destination data
        let pda_info = ctx.accounts.pda.as_ref();
        let mut pda_data = pda_info.try_borrow_mut_data()?;
        
        // Copy data length
        pda_data[DATA_OFFSET-4..DATA_OFFSET].copy_from_slice(&buffer_len.to_le_bytes());
        
        // Copy actual data
        pda_data[DATA_OFFSET..DATA_OFFSET + buffer_len as usize]
            .copy_from_slice(&buffer_data[DATA_OFFSET..DATA_OFFSET + buffer_len as usize]);
        
        Ok(())
    }

    // use std::cell::{Ref, RefMut};

    // pub trait MetadataUploadTrailingData<'info> {
    //     fn trailing_data(self) -> Ref<'info, [u8]>;
    //     fn trailing_data_mut(self) -> RefMut<'info, [u8]>;
    // }

    // impl<'a, 'info: 'a> MetadataUploadTrailingData<'a> for &'a Account<'info, MetadataAccount> {
    //     fn trailing_data(self) -> Ref<'a, [u8]> {
    //         let info: &AccountInfo<'info> = self.as_ref();
    //         Ref::map(info.try_borrow_data().unwrap(), |d| &d[44..])
    //     }
    //     fn trailing_data_mut(self) -> RefMut<'a, [u8]> {
    //         let info: &AccountInfo<'info> = self.as_ref();
    //         RefMut::map(info.try_borrow_mut_data().unwrap(), |d| &mut d[44..])
    //     }
    // }
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [seed.as_ref(), program_id.key.as_ref()],
        bump,
        payer = signer,
        space = 8 + 32 + 4,
    )]
    pub pda: AccountLoader<'info, MetadataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the program id of the program you want to upload the metadata for. Checks are done in code.
    pub program_id: AccountInfo<'info>,
    // Make sure that the signer is actually the upgrade authority of the program.
    #[account(constraint = program_data.upgrade_authority_address == Some(signer.key()))]
    pub program_data: Account<'info, ProgramData>,
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct InitializeWithSignerSeed<'info> {
    #[account(
        init,
        seeds = [seed.as_ref(), program_id.key.as_ref(), signer.key.as_ref()],
        bump,
        payer = signer,
        space = 8 + 32 + 4,
    )]
    pub pda: AccountLoader<'info, MetadataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the program id of the program you want to upload the metadata for. Checks are done in code.
    pub program_id: AccountInfo<'info>,
    // When we add signer seed we do NOT check if the signer is the program authority
    pub program_data: Account<'info, ProgramData>,
}

#[derive(Accounts)]
pub struct MetadataAccounts<'info> {
    #[account(mut, has_one = authority)]
    pub pda: AccountLoader<'info, MetadataAccount>,
    #[account(constraint = authority.key != &ERASED_AUTHORITY)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
// Seed can be dynamic. For IDL use "idl" as seed. For metadata use "metadata" as seed.
#[instruction(len: u16, seed: String)]
pub struct Resize<'info> {
    #[account(
        mut,
        realloc = len as usize, 
        realloc::zero = true, 
        realloc::payer = authority,
        //has_one = authority
    )]
    pub pda: AccountLoader<'info, MetadataAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the program id of the program you want to upload the metadata for.
    pub program_id: AccountInfo<'info>,
}


// Accounts for creating an metadata buffer.
#[derive(Accounts)]
pub struct CreateBuffer<'info> {
    #[account(zero)]
    pub buffer: AccountLoader<'info, MetadataAccount>,
    #[account(constraint = authority.key != &ERASED_AUTHORITY)]
    pub authority: Signer<'info>,
}

// Close buffer to claim back SOL
#[derive(Accounts)]
pub struct CloseBuffer<'info> {
    #[account(mut, close = authority,
        // has_one = authority
        )]
    pub buffer: AccountLoader<'info, MetadataAccount>,
    #[account(constraint = authority.key != &ERASED_AUTHORITY)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct WriteBuffer<'info> {
    #[account(mut)]
    pub buffer: AccountLoader<'info, MetadataAccount>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Accounts for upgrading the canonical MetadataAccount with the buffer.
#[derive(Accounts)]
#[instruction(seed: String)]
pub struct SetBuffer<'info> {
    // The buffer with the new metadata.
    #[account(mut)]
    pub buffer: AccountLoader<'info, MetadataAccount>,
    // The pda account to be updated with the buffer's data.
    #[account(mut)]
    pub pda: AccountLoader<'info, MetadataAccount>,
    pub authority: Signer<'info>,
    /// CHECK: This is the program id of the program you want to upload the IDL for.
    pub program_id: AccountInfo<'info>,
}

// #[account]
// pub struct MetadataAccount {
//     authority: Pubkey,
//     data_len: u32,
//     // program id or seed needed? To make it easier to query the IDL by program id or seed.
//     // The rest is compressed metadata bytes.
// }

#[account(zero_copy(unsafe))]
#[repr(C)]
pub struct MetadataAccount {
    authority: Pubkey,
    data_len: u32,
    data: [u8; 40952],
}
