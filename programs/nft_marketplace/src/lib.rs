use anchor_lang::prelude::*;
pub use instructions::user::*;
pub use instructions::token::*;

pub mod instructions;
pub mod states;

declare_id!("3emUs6bwmP7StdKakHe8pdbmLjhpZBsKuymy2sNHVxtL");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        instructions::user::create_user(ctx)
    }
    
    pub fn update_user(ctx: Context<UpdateUser>, name: String) -> Result<()> {
        instructions::user::update_user(ctx, name)
    }

    pub fn initialize_mint(ctx: Context<CreateToken>) -> Result<()> {
        instructions::token::create_token::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
