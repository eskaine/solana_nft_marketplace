use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateUser<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
      init,
      seeds = [user.key.as_ref()],
      bump,
      payer = user,
      space = 8 + USER_SIZE,
      constraint = 32 + (4 + name.len()) <= NAME_SIZE
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
