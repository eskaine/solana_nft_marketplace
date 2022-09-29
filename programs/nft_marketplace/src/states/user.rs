use anchor_lang::{prelude::*, solana_program};
use solana_program::program_pack::{IsInitialized, Sealed};

pub const NAME_SIZE: usize = 4 + 24;
pub const USER_SIZE: usize = NAME_SIZE //NAME
    + 32; // ADDRESS

#[account]
pub struct User {
    pub is_initialized: bool,
    pub name: String,
}

impl Sealed for User {}

impl IsInitialized for User {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
