use anchor_lang::prelude::*;

#[error_code]
pub enum UserError{
  #[msg("Input data exceeds max length")]
  InvalidDataLength,
}
