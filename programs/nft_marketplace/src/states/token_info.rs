use anchor_lang::prelude::*;

pub const TOKEN_INFO_SIZE: usize = 
      32 // USER ADDRESS
    + 32 // TOKEN ADDRESS
    + 32 // MINT ADDRESS
    + 1; // BUMP

#[account]
pub struct TokenInfo {
    pub owner: Pubkey,
    pub token: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
}

impl TokenInfo {
    pub fn init(
        &mut self,
        owner: Pubkey,
        token: Pubkey,
        mint: Pubkey,
        bump: u8,
    ) {
        self.owner = owner;
        self.token = token;
        self.mint = mint;
        self.bump = bump;
    }
}
