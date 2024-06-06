use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::invoke, program::invoke_signed, system_instruction::transfer,
};
use std::cmp::Ordering;


// Here are the constants and the utility functions 

pub const GLOBAL_AUTHORITY_SEED: &str = "global-authority";
pub const VAULT_SEED: &str = "vault-authority";
pub const GAME_SEED: &str = "game-authority";
pub const RANDOM_SEED: &str = "random";

pub const CODY_WALLET: &str = "GBfam19CeWi6msbrgHxFPmEBZMu4zHavQyNchdZTDtNU";   // 0.72%
pub const BET3_WALLET: &str = "41m5znXJg9CkxKBocJxntJZKn59U3BomkafEguxDDEWG";   // 1.08%
pub const JERZY_WALLET: &str = "79Zzb6b6JwxSc6SxwDJ8XuVci2hz45XwLQgi1aQBnMYX";  // 0.72%
pub const EXPER_WALLET: &str = "5dxyb7RWSdw1o9VXBN1gfL9oViBubeERx9g2HY74AHyD";  // 1.08%
pub const TEAM_WALLET: &str = "8Gbqb5ppmsocN8JMGBLNUHdn9zoZuiA6qzgwEPgN6j71";   // 0.4%

pub const HOUSE_FEE: u64 = 4;

// Here are some normal sample functions here
pub fn sol_transfer_user<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke(&ix, &[source, destination, system_program])?;
    Ok(())
}

pub fn sol_transfer_with_signer<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    signers: &[&[&[u8]]; 1],
    amount: u64,
) -> Result<()> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke_signed(&ix, &[source, destination, system_program], signers)?;
    Ok(())
}


pub fn resize_account<'info>(
    account_info: AccountInfo<'info>,
    new_space: usize,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
  ) -> Result<()> {
    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_space);
    let current_balance = account_info.lamports();
  
    match new_minimum_balance.cmp(&current_balance) {
      Ordering::Greater => {
        let lamports_diff = new_minimum_balance.saturating_sub(current_balance);
        invoke(
          &transfer(&payer.key(), &account_info.key(), lamports_diff),
          &[payer.clone(), account_info.clone(), system_program.clone()],
        )?;
      }
      Ordering::Less => {
        let lamports_diff = current_balance.saturating_sub(new_minimum_balance);
        **account_info.try_borrow_mut_lamports()? = new_minimum_balance;
        **payer.try_borrow_mut_lamports()? = payer
          .lamports()
          .checked_add(lamports_diff)
          .expect("Add error");
      }
      Ordering::Equal => {}
    }
    account_info.realloc(new_space, false)?;
    Ok(())
  }
  