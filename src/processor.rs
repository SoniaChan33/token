use borsh::{de, BorshDeserialize, BorshSerialize};
use solana_program::account_info::{self, next_account_info};
use solana_program::msg;
use solana_program::program::{invoke, invoke_signed};
use solana_program::program_pack::Pack;
use solana_program::system_instruction;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{self, ProgramResult},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
};
use spl_token::instruction::{initialize_mint, mint_to, mint_to_checked};
use spl_token::state::Mint;

// 引用自定义的tokeninstruction模块
use crate::instruction::TokenInstruction;
pub struct Process;

impl Process {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // 逻辑处理
        let instruction: TokenInstruction = TokenInstruction::try_from_slice(instruction_data)?;
        match instruction {
            TokenInstruction::CreateToken { decimals } => {
                // 处理创建Token逻辑
                Self::create_token(accounts, decimals);
            }
            TokenInstruction::Mint { amount } => {
                // 处理铸币逻辑
                Self::mint_tokens(accounts, amount);
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        }
        Ok(())
    }

    fn create_token(accounts: &[AccountInfo], decimals: u8) -> ProgramResult {
        // 这里可以实现创建Token的逻辑

        // 获取账户信息
        let account_iter = &mut accounts.iter();
        let mint_account = next_account_info(account_iter)?;
        let mint_authority = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let rent_sysvar = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;

        msg!("Creating mint account...");
        msg!("mint_account: {:?}", mint_account.key);

        // TODO 不懂ivoke是干嘛的
        invoke(
            &system_instruction::create_account(
                payer.key,
                mint_account.key,
                Rent::default().minimum_balance(Mint::LEN),
                Mint::LEN as u64,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
            ],
        )?;
        let mint_init_ix = &initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            None,
            decimals,
        )?;
        msg!("Initializing mint account...");
        invoke_signed(
            mint_init_ix,
            &[
                mint_account.clone(),
                mint_authority.clone(),
                rent_sysvar.clone(),
                token_program.clone(),
            ],
            &[],
        )?;

        msg!("SPL Token created successfully!");
        Ok(())
    }
    fn mint_tokens(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        // 这里可以实现铸币的逻辑
        let account_iter = &mut accounts.iter();

        // 创建铸币账户
        let mint_account = next_account_info(account_iter)?;
        let associated_token_account = next_account_info(account_iter)?;
        // TODO 这个到底是什么账户
        let rent_sysvar = next_account_info(account_iter)?;
        let payer = next_account_info(account_iter)?;
        let system_program = next_account_info(account_iter)?;
        let token_program = next_account_info(account_iter)?;
        let associated_token_program = next_account_info(account_iter)?;

        msg!("ATA: {:?}", associated_token_account);
        if associated_token_account.lamports() == 0 {
            msg!("Creating associated token account...");
            let create_ata_ix: &solana_program::instruction::Instruction =
                &spl_associated_token_account::instruction::create_associated_token_account(
                    payer.key,
                    payer.key,
                    mint_account.key,
                    token_program.key,
                );

            // 创建ATA账户
            invoke(
                create_ata_ix,
                &[
                    payer.clone(),
                    associated_token_account.clone(),
                    mint_account.clone(),
                    token_program.clone(),
                    system_program.clone(),
                    associated_token_program.clone(),
                ],
            )?;
        }

        msg!("Minting {} tokens to ata", amount);
        let mint_ix = &mint_to(
            token_program.key,
            mint_account.key,
            associated_token_account.key,
            payer.key,
            &[payer.key],
            amount,
        )?;
        // TODO: 为什么这里又不需要invoke_signed
        invoke(
            mint_ix,
            &[
                mint_account.clone(),
                associated_token_account.clone(),
                payer.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("Minting {} tokens to ata,success!", amount);

        Ok(())
    }
}
