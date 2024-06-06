use anchor_lang::prelude::*;

pub mod account;
pub mod utils;
pub mod error   ;

use account::*;
use utils::*;
use error::*;

declare_id!("E13jNxzoQbUuyaZ9rYJUdRAirYZKU75NJNRV9CHdDhHE");

#[program]
pub mod jackpot {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.sol_vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            ctx.accounts.rent.minimum_balance(0),
        )?;
        Ok(())
    }
    
    pub fn play_game(ctx: Context<PlayGame>, ts: u64, amount: u64) -> Result<()> {
        let game_pool = &mut ctx.accounts.game_pool;
        
        let timestamp = Clock::get()?.unix_timestamp;
        // Get the random number of the entrant amount
        let (player_address, _bump) = Pubkey::find_program_address(
            &[
                RANDOM_SEED.as_bytes(),
                timestamp.to_string().as_bytes(),
            ],
            &jackpot::ID,
        );
        let char_vec: Vec<char> = player_address.to_string().chars().collect();
        let mut mul = 1;
        for i in 0..7 {
            mul *= u64::from(char_vec[i as usize]);
        }
        mul += u64::from(char_vec[7]);

        game_pool.start_ts = ts;
        game_pool.rand = mul;
        game_pool.total_deposit = amount;
        game_pool.entrants.push(ctx.accounts.admin.key());
        game_pool.deposit_amounts.push(amount);

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.sol_vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount
        )?;
        
        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.cody_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 72 / 10000
        )?;

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.jerzy_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 72 / 10000
        )?;

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.bet3_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 108 / 10000
        )?;

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.exper_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 108 / 10000
        )?;
        
        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.team_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 40 / 10000
        )?;
        
        Ok(())
    }

    pub fn enter_game(ctx: Context<EnterGame>, amount: u64) -> Result<()> {
        let game_pool = &mut ctx.accounts.game_pool;
        require!(game_pool.claimed == 0, JackpotError::AlreadyClaimed);

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.sol_vault.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount
        )?;
        
        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.cody_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 72 / 10000
        )?;

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.jerzy_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 72 / 10000
        )?;

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.bet3_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 108 / 10000
        )?;

        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.exper_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 108 / 10000
        )?;
        
        sol_transfer_user(
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.team_wallet.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            amount * 40 / 10000
        )?;

        resize_account(
            game_pool.to_account_info().clone(),
            GamePool::space(game_pool.entrants.len() as usize),
            ctx.accounts.admin.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
        )?;
        game_pool.append(ctx.accounts.admin.key(), amount);

        Ok(())
    }

    pub fn claim_reward(ctx: Context<ClaimReward>, vault_bump: u8) -> Result<()> {
        let game_pool = &mut ctx.accounts.game_pool;
        require!(game_pool.claimed == 0, JackpotError::AlreadyClaimed);

        game_pool.set_winner();

        require!(ctx.accounts.winner.key() == game_pool.winner, JackpotError::NotWinner);

        let seeds = &[
            VAULT_SEED.as_bytes(),
            &[vault_bump],
        ];
        let signer = &[&seeds[..]];

        sol_transfer_with_signer(
            ctx.accounts.sol_vault.to_account_info(),
            ctx.accounts.winner.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            signer,
            game_pool.total_deposit,
        )?;

        game_pool.claimed = 1;

        Ok(())
    }

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()], 
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(
    ts: u64
)]
pub struct PlayGame<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [GAME_SEED.as_ref(), admin.to_account_info().key.as_ref(), ts.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        space = 8 + GamePool::MAX_DATA_SIZE
    )]
    pub game_pool: Account<'info, GamePool>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,

    #[account(
        mut,
        constraint = cody_wallet.key() == CODY_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub cody_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = bet3_wallet.key() == BET3_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub bet3_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = jerzy_wallet.key() == JERZY_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub jerzy_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = exper_wallet.key() == EXPER_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub exper_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = team_wallet.key() == TEAM_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub team_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct EnterGame<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub game_pool: Account<'info, GamePool>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,
    
    #[account(
        mut,
        constraint = cody_wallet.key() == CODY_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub cody_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = bet3_wallet.key() == BET3_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub bet3_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = jerzy_wallet.key() == JERZY_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub jerzy_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = exper_wallet.key() == EXPER_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub exper_wallet: SystemAccount<'info>,
    #[account(
        mut,
        constraint = team_wallet.key() == TEAM_WALLET.parse::<Pubkey>().unwrap()
    )]
    pub team_wallet: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub game_pool: Account<'info, GamePool>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub winner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [VAULT_SEED.as_ref()],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sol_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}