use anchor_lang::prelude::*;

#[account]
pub struct Identity {
  user_account: Pubkey
}