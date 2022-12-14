use anchor_lang::prelude::*;
use crate::states::*;

const DEFAULT_NAME: &str = "Unknown";

#[derive(Accounts)]
pub struct CreateUser<'info> {
  #[account(mut)]
  pub initializer: Signer<'info>,
  #[account(
      init_if_needed, 
      seeds = [initializer.key.as_ref()],
      bump,
      payer = initializer,
      space = USER_SIZE,
  )]
  pub user_account: Account<'info, User>,
  pub system_program: Program<'info, System>
}

pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
  ctx.accounts.user_account.name = DEFAULT_NAME.to_owned();
  ctx.accounts.user_account.is_initialized = true;

  Ok(())
}
