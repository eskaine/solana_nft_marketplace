use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Transfer};

trait TransferOwnership {
    fn transfer(&self) -> Result<()> {
        // let cpi_accounts = Transfer {
        //     from: self.seller_token.to_account_info().clone(),
        //     to: self.escrow_token.to_account_info().clone(),
        //     authority: self.seller.to_account_info().clone(),
        // };

        // token::transfer(CpiContext::new(self.token_program.clone(), cpi_accounts), 1)?;

        Ok(())
    }
}
