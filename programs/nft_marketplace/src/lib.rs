use anchor_lang::prelude::*;
pub use instructions::user::*;
pub use instructions::token::*;

pub mod instructions;
pub mod states;

declare_id!("CexBmrzVQBJB9BKy2ujzVppkyL85jFVxxSWQ9BiWXspc");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        instructions::user::create_user(ctx)
    }
    
    pub fn update_user(ctx: Context<UpdateUser>, name: String) -> Result<()> {
        instructions::user::update_user(ctx, name)
    }

    pub fn create_nft(
        ctx: Context<CreateNft>, 
        metadata_title: String, 
        metadata_symbol: String,
        metadata_uri: String,
        royalty: u16
    ) -> Result<()> {
        instructions::token::create_nft(ctx, metadata_title, metadata_symbol, metadata_uri, royalty)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
