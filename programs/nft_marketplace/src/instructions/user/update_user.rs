use anchor_lang::prelude::*;
use crate::states::*;
use crate::instructions::error::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct UpdateUser<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
      init_if_needed,
      seeds = [user.key.as_ref()],
      bump,
      payer = user,
      space = USER_SIZE,
      constraint = 4 + name.to_owned().len()<= NAME_SIZE
        @ UserError::InvalidDataLength
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
