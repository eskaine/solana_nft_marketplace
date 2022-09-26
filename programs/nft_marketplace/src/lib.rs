use anchor_lang::prelude::*;

pub use instructions::user::*;

pub mod instructions;
pub mod states;

declare_id!("GjFofTmc4TMKLdPLVayhWyVc9bBRsN9ayVBwYGJPwCZH");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        instructions::user::create_user(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
