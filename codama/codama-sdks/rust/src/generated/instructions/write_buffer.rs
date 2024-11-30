//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct WriteBuffer {
      
              
          pub buffer: solana_program::pubkey::Pubkey,
          
              
          pub signer: solana_program::pubkey::Pubkey,
          
              
          pub system_program: solana_program::pubkey::Pubkey,
      }

impl WriteBuffer {
  pub fn instruction(&self, args: WriteBufferInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: WriteBufferInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.buffer,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = WriteBufferInstructionData::new().try_to_vec().unwrap();
          let mut args = args.try_to_vec().unwrap();
      data.append(&mut args);
    
    solana_program::instruction::Instruction {
      program_id: crate::UPLOAD_IDL_ANCHOR_ID,
      accounts,
      data,
    }
  }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct WriteBufferInstructionData {
            discriminator: [u8; 8],
            }

impl WriteBufferInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [164, 194, 69, 154, 75, 169, 228, 85],
                                }
  }
}

impl Default for WriteBufferInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteBufferInstructionArgs {
                  pub idl_data: Vec<u8>,
      }


/// Instruction builder for `WriteBuffer`.
///
/// ### Accounts:
///
                ///   0. `[writable]` buffer
                      ///   1. `[writable, signer]` signer
                ///   2. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct WriteBufferBuilder {
            buffer: Option<solana_program::pubkey::Pubkey>,
                signer: Option<solana_program::pubkey::Pubkey>,
                system_program: Option<solana_program::pubkey::Pubkey>,
                        idl_data: Option<Vec<u8>>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl WriteBufferBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn buffer(&mut self, buffer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.buffer = Some(buffer);
                    self
    }
            #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.signer = Some(signer);
                    self
    }
            /// `[optional account, default to '11111111111111111111111111111111']`
#[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn idl_data(&mut self, idl_data: Vec<u8>) -> &mut Self {
        self.idl_data = Some(idl_data);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = WriteBuffer {
                              buffer: self.buffer.expect("buffer is not set"),
                                        signer: self.signer.expect("signer is not set"),
                                        system_program: self.system_program.unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
                      };
          let args = WriteBufferInstructionArgs {
                                                              idl_data: self.idl_data.clone().expect("idl_data is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `write_buffer` CPI accounts.
  pub struct WriteBufferCpiAccounts<'a, 'b> {
          
                    
              pub buffer: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub signer: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `write_buffer` CPI instruction.
pub struct WriteBufferCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub buffer: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub signer: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: WriteBufferInstructionArgs,
  }

impl<'a, 'b> WriteBufferCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: WriteBufferCpiAccounts<'a, 'b>,
              args: WriteBufferInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              buffer: accounts.buffer,
              signer: accounts.signer,
              system_program: accounts.system_program,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.buffer.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = WriteBufferInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::UPLOAD_IDL_ANCHOR_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.buffer.clone());
                        account_infos.push(self.signer.clone());
                        account_infos.push(self.system_program.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `WriteBuffer` via CPI.
///
/// ### Accounts:
///
                ///   0. `[writable]` buffer
                      ///   1. `[writable, signer]` signer
          ///   2. `[]` system_program
#[derive(Clone, Debug)]
pub struct WriteBufferCpiBuilder<'a, 'b> {
  instruction: Box<WriteBufferCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> WriteBufferCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(WriteBufferCpiBuilderInstruction {
      __program: program,
              buffer: None,
              signer: None,
              system_program: None,
                                            idl_data: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn buffer(&mut self, buffer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.buffer = Some(buffer);
                    self
    }
      #[inline(always)]
    pub fn signer(&mut self, signer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.signer = Some(signer);
                    self
    }
      #[inline(always)]
    pub fn system_program(&mut self, system_program: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.system_program = Some(system_program);
                    self
    }
                    #[inline(always)]
      pub fn idl_data(&mut self, idl_data: Vec<u8>) -> &mut Self {
        self.instruction.idl_data = Some(idl_data);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = WriteBufferInstructionArgs {
                                                              idl_data: self.instruction.idl_data.clone().expect("idl_data is not set"),
                                    };
        let instruction = WriteBufferCpi {
        __program: self.instruction.__program,
                  
          buffer: self.instruction.buffer.expect("buffer is not set"),
                  
          signer: self.instruction.signer.expect("signer is not set"),
                  
          system_program: self.instruction.system_program.expect("system_program is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct WriteBufferCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        idl_data: Option<Vec<u8>>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
