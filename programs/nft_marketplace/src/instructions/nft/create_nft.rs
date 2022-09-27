use anchor_lang::prelude::*;

use crate::states::*;
use anchor_spl::token::{
    self, GetOrCreateAssociatedTokenAccount, InitializeMint, Mint, SetAuthority, TokenAccount,
    Transfer,
};
use spl_token::instruction::AuthorityType;

pub struct NFT {
    pub name: String,
    pub address: Pubkey,
    pub mint_address: Pubkey,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
      mut,
      constraint = owner_token.amount == 1,
      constraint = owner_token.owner == seller.to_account_info().key()
  )]
    pub owner_token: Account<'info, TokenAccount>,
    #[account(
      constraint = nft_mint.decimals == 0,
      constraint = nft_mint.supply == 1,
      constraint = owner_token.mint == nft_mint.key()
  )]
    pub nft_mint: Account<'info, Mint>,
    #[account(init, payer = owner, space = 8 + LISTING_PROOF_LEN, seeds = [seller_token.key().as_ref()], bump)]
    pub listing_proof: Account<'info, Nft>,
    #[account(
      init,
      seeds = [owner_token.key().as_ref(), b"escrow-token"],
      bump,
      payer = owner,
      token::mint = nft_mint,
      token::authority = seller,
  )]
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: AccountInfo<'info>,
}

impl<'info> TransferOwnership for CreateNft<'info> {
    fn create_mint(&self) -> Result<()> {
        let cpi_accounts = InitializeMint {
            account_or_mint: self.escrow_token.to_account_info().clone(),
            current_authority: self.seller.to_account_info().clone(),
        };

        Ok(())
    }
}

pub fn handler(// ctx: Context<CreateListing>,
    // list_price: u128,
    // listing_proof_bump: u8,
) -> Result<()> {
    // ctx.accounts.listing_proof.init_listing_proof(
    //     ctx.accounts.nft_mint.key(),
    //     ctx.accounts.seller.key(),
    //     ctx.accounts.seller_token.key(),
    //     ctx.accounts.escrow_token.key(),
    //     list_price,
    //     listing_proof_bump,
    // );

    // ctx.accounts.set_authority_to_listing_proof()?;
    // ctx.accounts.transfer_to_escrow_token()?;

    Ok(())
}
