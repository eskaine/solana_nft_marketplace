use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, CreateAccount};
use anchor_spl::token::{ self, InitializeMint, MintTo, Token };
use anchor_spl::associated_token:: { self, Create, AssociatedToken};

const MINT_DECIMALS: u8 = 0;
const LAMPORTS: u64 = 10000000;
const SPACE: u64 = 82;
const MAX_TOKEN_AMOUNT: u64 = 1;

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: Creating account...
    #[account(mut)]
    pub nft_token: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreateToken<'info> {
    fn create_mint_account(&self) -> Result<()> {
        msg!("Creating mint account...");

        let mint_account = CreateAccount {
            from: self.user.to_account_info().clone(),
            to: self.mint.to_account_info().clone()
        };
    
        system_program::create_account(
            CpiContext::new(
                self.token_program.to_account_info().clone(), 
                mint_account
            ),
            LAMPORTS,
            SPACE,
            &self.token_program.key()
        )?;

        Ok(())
    }

    fn initialize_mint(&self) -> Result<()> {
        msg!("Initializing mint...");

        let mint = InitializeMint {
            mint: self.mint.to_account_info().clone(),
            rent: self.rent.to_account_info().clone()        
        };

        token::initialize_mint(
            CpiContext::new(
                self.token_program.to_account_info().clone(), 
                mint
            ),
            MINT_DECIMALS,
            &self.user.key(),
            Some(&self.user.key())
        )?;

        Ok(())
    }

    fn create_token(&self) -> Result<()> {
        msg!("Creating token...");

        let token = Create {
            payer: self.user.to_account_info().clone(),
            associated_token: self.nft_token.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
            mint: self.mint.to_account_info().clone(),
            system_program: self.system_program.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),
            rent: self.rent.to_account_info().clone()
        };

        associated_token::create(
            CpiContext::new(
                self.associated_token_program.to_account_info().clone(), 
                token
            ),
        )?;

        Ok(())
    }

    fn mint_to_token_account(&self) -> Result<()> {
        msg!("Minting token to token account...");
        println!("hello there!");

        let mint_to = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.nft_token.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };

        token::mint_to(
            CpiContext::new(
                self.token_program.to_account_info().clone(), 
                mint_to
            ),
            MAX_TOKEN_AMOUNT
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateToken>) -> Result<()> {
    ctx.accounts.create_mint_account()?;
    ctx.accounts.initialize_mint()?;
    ctx.accounts.create_token()?;
    ctx.accounts.mint_to_token_account()?;

    Ok(())
}
