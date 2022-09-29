use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint };

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = 9,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
    Ok(())
}

// #[derive(Accounts)]
// pub struct CreateUser<'info> {
//   #[account(mut)]
//   pub initializer: Signer<'info>,
//   #[account(
//       init_if_needed, 
//       seeds = [initializer.key.as_ref()],
//       bump,
//       payer = initializer,
//       space = 8 + USER_SIZE,
//       constraint = 32 + (4 + DEFAULT_NAME.to_owned().len()) <= 1000
//   )]
//   pub user_account: Account<'info, User>,
//   pub system_program: Program<'info, System>
// }

// use anchor_lang::prelude::*;
// use crate::states::*;

// const DEFAULT_NAME: &str = "Unknown"; 

// #[derive(Accounts)]
// pub struct CreateUser<'info> {
//   #[account(mut)]
//   pub initializer: Signer<'info>,
//   #[account(
//       init_if_needed, 
//       seeds = [initializer.key.as_ref()],
//       bump,
//       payer = initializer,
//       space = 8 + USER_SIZE,
//       constraint = 32 + (4 + DEFAULT_NAME.to_owned().len()) <= 1000
//   )]
//   pub user_account: Account<'info, User>,
//   pub system_program: Program<'info, System>
// }

// pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
//   ctx.accounts.user_account.name = DEFAULT_NAME.to_owned();
//   Ok(())
// }
