use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::system_program::{ self, CreateAccount };
use anchor_spl::token::{ self, InitializeMint, MintTo, Token };
use anchor_spl::associated_token::{ self, Create, AssociatedToken };
use mpl_token_metadata::state::Creator;
use mpl_token_metadata::instruction::{ create_metadata_accounts_v3 };

const MINT_DECIMALS: u8 = 0;
const LAMPORTS: u64 = 10000000;
const MINT_SIZE: u64 = 82;
const MAX_TOKEN_AMOUNT: u64 = 1;

#[derive(Accounts)]
#[instruction(metadata_title: String, metadata_symbol: String, metadata_uri: String, royalty: u16)]
pub struct CreateNft<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub nft_token: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn create_nft(
    ctx: Context<CreateNft>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
    royalty: u16
) -> Result<()> {
    msg!("Creating NFT...");

    let mint_account = CreateAccount {
        from: ctx.accounts.mint_authority.to_account_info().clone(),
        to: ctx.accounts.mint.to_account_info().clone(),
    };

    system_program::create_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info().clone(), mint_account),
        LAMPORTS,
        MINT_SIZE,
        &ctx.accounts.token_program.key()
    )?;

    let mint = InitializeMint {
        mint: ctx.accounts.mint.to_account_info().clone(),
        rent: ctx.accounts.rent.to_account_info().clone(),
    };

    token::initialize_mint(
        CpiContext::new(ctx.accounts.token_program.to_account_info().clone(), mint),
        MINT_DECIMALS,
        &ctx.accounts.mint_authority.key(),
        Some(&ctx.accounts.mint_authority.key())
    )?;

    msg!("Creating token...");

    let token = Create {
        payer: ctx.accounts.mint_authority.to_account_info().to_account_info().clone(),
        associated_token: ctx.accounts.nft_token.to_account_info().clone(),
        authority: ctx.accounts.mint_authority.to_account_info().clone(),
        mint: ctx.accounts.mint.to_account_info().clone(),
        system_program: ctx.accounts.system_program.to_account_info().clone(),
        token_program: ctx.accounts.token_program.to_account_info().clone(),
        rent: ctx.accounts.rent.to_account_info().clone(),
    };

    associated_token::create(
        CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info().clone(), 
            token
        )
    )?;

    msg!("Minting...");

    let mint_to = MintTo {
        mint: ctx.accounts.mint.to_account_info().clone(),
        to: ctx.accounts.nft_token.to_account_info().clone(),
        authority: ctx.accounts.mint_authority.to_account_info().clone(),
    };

    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info().clone(), mint_to),
        MAX_TOKEN_AMOUNT
    )?;

    let creator = match royalty {
        0 => None,
        _ =>
            Some(
                vec![Creator {
                    address: ctx.accounts.mint_authority.key(),
                    verified: false,
                    share: 100,
                }]
            ),
    };

    invoke(
        &create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(), 
            ctx.accounts.metadata.key(), 
            ctx.accounts.mint.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            ctx.accounts.mint_authority.key(), 
            metadata_title, 
            metadata_symbol, 
            metadata_uri, 
            creator,
            royalty,
            true, 
            false, 
            None, 
            None,
            None
        ),
        &[
            ctx.accounts.metadata.to_account_info().clone(),
            ctx.accounts.mint.to_account_info().clone(),
            ctx.accounts.mint_authority.to_account_info().clone(), 
            ctx.accounts.token_metadata_program.to_account_info().clone(),
            ctx.accounts.token_program.to_account_info().clone(),
            ctx.accounts.system_program.to_account_info().clone(),
            ctx.accounts.rent.to_account_info().clone(), 
        ],
    )?;

    Ok(())
}