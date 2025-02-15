use anchor_lang::prelude::*;
use crate::state::User;
use crate::common::SolidError;

#[accounts]
#[instruction(username: String)]
pub struct Register {
  #[account(mut)]
  user: Signer,

  #[account(
    mut,
    payer = user,
    seeds = [b"user_account", user.key().as_ref()],
    bump
  )]
  user_account: Account<'info, User>,

  #[account(
    mut,
    payer = user,
    seeds = [b"identity", username.as_ref()],
    bump
  )]
  identity: Account<'info, User>,
}

pub fn process(ctx: Context<Register>, username: String) -> Result<()> {
  let user_account = &mut ctx.accounts.user_account;

  let username_bytes = username.as_bytes();
  require_gt!(username_bytes.len(),200, SolidError::UsernameTooLong);

  let mut username_array = [0u8; 200];
  username_array[..username_bytes.len()].copy_from_slice(username_bytes);

  user_account.username = username_array;
  user_account.master = ctx.accounts.user.key();
  user_account.linking_wallets = Vec::new();

  let identity = &ctx.accounts.identity;

  identity.user_account = ctx.accounts.user_account.key();

  Ok(())
}
