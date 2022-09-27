use anchor_lang::prelude::*;

pub const USER_SIZE: usize = 24 //NAME
    + 32 // ADDRESS
    + 32 // MINT ADDRESS
    + 1; //BUMP

#[account]
pub struct Nft {
    pub name: String,
    pub address: Pubkey,
    pub mint_address: Pubkey,
    pub bump: u8,
}

impl Nft {
    pub fn init(&mut self, name: String, address: Pubkey, mint_address: Pubkey, bump: u8) {
        self.name = name;
        self.address = address;
        self.mint_address = mint_address;
        self.bump = bump;
    }
}
