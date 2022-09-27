use anchor_lang::prelude::*;

pub const retainer_size: usize = 32 // NFT MINT
  + 32  // SELLER ADDRESS
  + 32  // SELLER TOKEN ADDRESS
  + 32  // RETAINER TOKEN ADDRESS
  + 16  // NFT PRICE 
  + 1; // bump

#[account]
pub struct Retainer {
    pub nft_mint: Pubkey,
    pub seller_address: Pubkey,
    pub seller_token_address: Pubkey,
    pub retainer_token_address: Pubkey,
    pub nft_price: u128,
    pub bump: u8,
}

impl Retainer {
    pub fn init(
        &mut self,
        nft_mint: Pubkey,
        seller_address: Pubkey,
        seller_token_address: Pubkey,
        retainer_token_address: Pubkey,
        nft_price: u128,
        bump: u8,
    ) {
        self.nft_mint = nft_mint;
        self.seller_address = seller_address;
        self.seller_token_address = seller_token_address;
        self.retainer_token_address = retainer_token_address;
        self.nft_price = nft_price;
        self.bump = bump;
    }
}
