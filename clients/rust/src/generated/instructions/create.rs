//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::CreateArgs;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Create {
    /// Unallocated metadata account with address as pda of ['metadata', program id, mint id]
    pub metadata: solana_program::pubkey::Pubkey,
    /// Unallocated edition account with address as pda of ['metadata', program id, mint, 'edition']
    pub master_edition: Option<solana_program::pubkey::Pubkey>,
    /// Mint of token asset
    pub mint: (solana_program::pubkey::Pubkey, bool),
    /// Mint authority
    pub authority: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// Update authority for the metadata account
    pub update_authority: (solana_program::pubkey::Pubkey, bool),
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
    /// Instructions sysvar account
    pub sysvar_instructions: solana_program::pubkey::Pubkey,
    /// SPL Token program
    pub spl_token_program: solana_program::pubkey::Pubkey,
}

impl Create {
    pub fn instruction(
        &self,
        args: CreateInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: CreateInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.metadata,
            false,
        ));
        if let Some(master_edition) = self.master_edition {
            accounts.push(solana_program::instruction::AccountMeta::new(
                master_edition,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.mint.0,
            self.mint.1,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.update_authority.0,
            self.update_authority.1,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.sysvar_instructions,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.spl_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = CreateInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct CreateInstructionData {
    discriminator: u8,
}

impl CreateInstructionData {
    fn new() -> Self {
        Self { discriminator: 42 }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateInstructionArgs {
    pub create_args: CreateArgs,
}

/// Instruction builder.
#[derive(Default)]
pub struct CreateBuilder {
    metadata: Option<solana_program::pubkey::Pubkey>,
    master_edition: Option<solana_program::pubkey::Pubkey>,
    mint: Option<(solana_program::pubkey::Pubkey, bool)>,
    authority: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    update_authority: Option<(solana_program::pubkey::Pubkey, bool)>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    sysvar_instructions: Option<solana_program::pubkey::Pubkey>,
    spl_token_program: Option<solana_program::pubkey::Pubkey>,
    create_args: Option<CreateArgs>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl CreateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Unallocated metadata account with address as pda of ['metadata', program id, mint id]
    #[inline(always)]
    pub fn metadata(&mut self, metadata: solana_program::pubkey::Pubkey) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    /// `[optional account]`
    /// Unallocated edition account with address as pda of ['metadata', program id, mint, 'edition']
    #[inline(always)]
    pub fn master_edition(
        &mut self,
        master_edition: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.master_edition = master_edition;
        self
    }
    /// Mint of token asset
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey, as_signer: bool) -> &mut Self {
        self.mint = Some((mint, as_signer));
        self
    }
    /// Mint authority
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// Update authority for the metadata account
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: solana_program::pubkey::Pubkey,
        as_signer: bool,
    ) -> &mut Self {
        self.update_authority = Some((update_authority, as_signer));
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'Sysvar1nstructions1111111111111111111111111']`
    /// Instructions sysvar account
    #[inline(always)]
    pub fn sysvar_instructions(
        &mut self,
        sysvar_instructions: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.sysvar_instructions = Some(sysvar_instructions);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL Token program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.spl_token_program = Some(spl_token_program);
        self
    }
    #[inline(always)]
    pub fn create_args(&mut self, create_args: CreateArgs) -> &mut Self {
        self.create_args = Some(create_args);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Create {
            metadata: self.metadata.expect("metadata is not set"),
            master_edition: self.master_edition,
            mint: self.mint.expect("mint is not set"),
            authority: self.authority.expect("authority is not set"),
            payer: self.payer.expect("payer is not set"),
            update_authority: self.update_authority.expect("update_authority is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            sysvar_instructions: self.sysvar_instructions.unwrap_or(solana_program::pubkey!(
                "Sysvar1nstructions1111111111111111111111111"
            )),
            spl_token_program: self.spl_token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = CreateInstructionArgs {
            create_args: self.create_args.clone().expect("create_args is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `create` CPI accounts.
pub struct CreateCpiAccounts<'a, 'b> {
    /// Unallocated metadata account with address as pda of ['metadata', program id, mint id]
    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Unallocated edition account with address as pda of ['metadata', program id, mint, 'edition']
    pub master_edition: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Mint of token asset
    pub mint: (&'b solana_program::account_info::AccountInfo<'a>, bool),
    /// Mint authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Update authority for the metadata account
    pub update_authority: (&'b solana_program::account_info::AccountInfo<'a>, bool),
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Instructions sysvar account
    pub sysvar_instructions: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token program
    pub spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `create` CPI instruction.
pub struct CreateCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Unallocated metadata account with address as pda of ['metadata', program id, mint id]
    pub metadata: &'b solana_program::account_info::AccountInfo<'a>,
    /// Unallocated edition account with address as pda of ['metadata', program id, mint, 'edition']
    pub master_edition: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Mint of token asset
    pub mint: (&'b solana_program::account_info::AccountInfo<'a>, bool),
    /// Mint authority
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// Update authority for the metadata account
    pub update_authority: (&'b solana_program::account_info::AccountInfo<'a>, bool),
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Instructions sysvar account
    pub sysvar_instructions: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL Token program
    pub spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: CreateInstructionArgs,
}

impl<'a, 'b> CreateCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: CreateCpiAccounts<'a, 'b>,
        args: CreateInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            metadata: accounts.metadata,
            master_edition: accounts.master_edition,
            mint: accounts.mint,
            authority: accounts.authority,
            payer: accounts.payer,
            update_authority: accounts.update_authority,
            system_program: accounts.system_program,
            sysvar_instructions: accounts.sysvar_instructions,
            spl_token_program: accounts.spl_token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.metadata.key,
            false,
        ));
        if let Some(master_edition) = self.master_edition {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *master_edition.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::MPL_TOKEN_METADATA_ID,
                false,
            ));
        }
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.mint.0.key,
            self.mint.1,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.update_authority.0.key,
            self.update_authority.1,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.sysvar_instructions.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.spl_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = CreateInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_TOKEN_METADATA_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.metadata.clone());
        if let Some(master_edition) = self.master_edition {
            account_infos.push(master_edition.clone());
        }
        account_infos.push(self.mint.0.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.update_authority.0.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.sysvar_instructions.clone());
        account_infos.push(self.spl_token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// `create` CPI instruction builder.
pub struct CreateCpiBuilder<'a, 'b> {
    instruction: Box<CreateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> CreateCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(CreateCpiBuilderInstruction {
            __program: program,
            metadata: None,
            master_edition: None,
            mint: None,
            authority: None,
            payer: None,
            update_authority: None,
            system_program: None,
            sysvar_instructions: None,
            spl_token_program: None,
            create_args: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Unallocated metadata account with address as pda of ['metadata', program id, mint id]
    #[inline(always)]
    pub fn metadata(
        &mut self,
        metadata: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.metadata = Some(metadata);
        self
    }
    /// `[optional account]`
    /// Unallocated edition account with address as pda of ['metadata', program id, mint, 'edition']
    #[inline(always)]
    pub fn master_edition(
        &mut self,
        master_edition: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.master_edition = master_edition;
        self
    }
    /// Mint of token asset
    #[inline(always)]
    pub fn mint(
        &mut self,
        mint: &'b solana_program::account_info::AccountInfo<'a>,
        as_signer: bool,
    ) -> &mut Self {
        self.instruction.mint = Some((mint, as_signer));
        self
    }
    /// Mint authority
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// Update authority for the metadata account
    #[inline(always)]
    pub fn update_authority(
        &mut self,
        update_authority: &'b solana_program::account_info::AccountInfo<'a>,
        as_signer: bool,
    ) -> &mut Self {
        self.instruction.update_authority = Some((update_authority, as_signer));
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// Instructions sysvar account
    #[inline(always)]
    pub fn sysvar_instructions(
        &mut self,
        sysvar_instructions: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.sysvar_instructions = Some(sysvar_instructions);
        self
    }
    /// SPL Token program
    #[inline(always)]
    pub fn spl_token_program(
        &mut self,
        spl_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.spl_token_program = Some(spl_token_program);
        self
    }
    #[inline(always)]
    pub fn create_args(&mut self, create_args: CreateArgs) -> &mut Self {
        self.instruction.create_args = Some(create_args);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = CreateInstructionArgs {
            create_args: self
                .instruction
                .create_args
                .clone()
                .expect("create_args is not set"),
        };
        let instruction = CreateCpi {
            __program: self.instruction.__program,

            metadata: self.instruction.metadata.expect("metadata is not set"),

            master_edition: self.instruction.master_edition,

            mint: self.instruction.mint.expect("mint is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            update_authority: self
                .instruction
                .update_authority
                .expect("update_authority is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            sysvar_instructions: self
                .instruction
                .sysvar_instructions
                .expect("sysvar_instructions is not set"),

            spl_token_program: self
                .instruction
                .spl_token_program
                .expect("spl_token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct CreateCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    metadata: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    master_edition: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<(&'b solana_program::account_info::AccountInfo<'a>, bool)>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    update_authority: Option<(&'b solana_program::account_info::AccountInfo<'a>, bool)>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    sysvar_instructions: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    spl_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    create_args: Option<CreateArgs>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
