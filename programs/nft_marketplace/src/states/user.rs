use anchor_lang::prelude::*;

pub const USER_SIZE: usize = 24 //NAME
    + 32; // ADDRESS

#[account]
pub struct User {
    pub name: String,
    pub address: Pubkey
}
