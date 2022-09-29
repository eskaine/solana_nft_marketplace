use anchor_lang::prelude::*;
use anchor_spl::token::{ self, InitializeMint, InitializeAccount, Mint, TokenAccount, MintTo };
use anchor_spl::associated_token::{ self, Create };

const MINT_DECIMALS: u8 = 0;
const MAX_TOKEN_AMOUNT: u64 = 1;

#[derive(Accounts)]
#[instruction(nft_name: String)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
        constraint = user_token.mint == mint.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [user.key().as_ref(), nft_name.as_bytes().as_ref(), b"nft-token"],
        bump,
        payer = user,
        token::mint = mint,
        token::authority = user
    )]
    pub user_token: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
}

impl<'info> CreateToken<'info> {
    fn initialize_mint(&self) -> Result<()> {
        msg!("Initializing mint...");

        let mint_account = InitializeMint {
            mint: self.mint.to_account_info().clone(),
            rent: self.rent.to_account_info().clone()        
        };

        token::initialize_mint(
            CpiContext::new(self.token_program.clone(), mint_account),
            MINT_DECIMALS,
            &self.user.key(),
            Some(&self.user.key())
        )?;

        Ok(())
    }

    fn create_token_account(&self) -> Result<()> {
        msg!("Creating token account...");

        let token_account = InitializeAccount {
            account: self.user_token.to_account_info().clone(),
            mint: self.mint.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
            rent: self.rent.to_account_info().clone()
        };

        token::initialize_account(
            CpiContext::new(self.token_program.clone(), token_account),
        )?;

        Ok(())
    }

    fn create_token(&self) -> Result<()> {
        msg!("Creating token...");

        let token = Create {
            payer: self.user.to_account_info().clone(),
            associated_token: self.user_token.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
            mint: self.mint.to_account_info().clone(),
            system_program: self.system_program.to_account_info().clone(),
            token_program: self.token_program.to_account_info().clone(),
            rent: self.rent.to_account_info().clone()
        };

        associated_token::create(
            CpiContext::new(self.token_program.clone(), token),
        )?;

        Ok(())
    }

    fn mint_to_token_account(&self) -> Result<()> {
        msg!("Minting token to token account...");

        let mint_to = MintTo {
            mint: self.mint.to_account_info().clone(),
            to: self.user_token.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };

        token::mint_to(
            CpiContext::new(self.token_program.clone(), mint_to),
            MAX_TOKEN_AMOUNT
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateToken>) -> Result<()> {
    ctx.accounts.initialize_mint();
    ctx.accounts.create_token_account();
    ctx.accounts.create_token();
    ctx.accounts.mint_to_token_account();

    Ok(())
}
