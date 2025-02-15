use anchor_lang::prelude::*;

#[account]
pub struct User {
  username: [u8; 200],
  master: Pubkey,
  linking_wallets: Vec<Pubkey>
}