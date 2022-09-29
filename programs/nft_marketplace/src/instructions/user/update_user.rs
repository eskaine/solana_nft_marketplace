use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateUser<'info> {
  #[account(mut)]
  pub initializer: Signer<'info>,
  #[account(
      init, 
      seeds = [initializer.key.as_ref()],
      bump,
      payer = initializer,
      space = 8 + USER_SIZE,
      constraint = 32 + (4 + name.len()) <= 1000
        @ ProgramError::MaxAccountsDataSizeExceeded
  )]
  pub user_account: Account<'info, User>,
  pub system_program: Program<'info, System>
}

pub fn update_user(
  ctx: Context<UpdateUser>,
  name: String
) -> Result<()> {
  ctx.accounts.user_account.name = name;
  Ok(())
}
