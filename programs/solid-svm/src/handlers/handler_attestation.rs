use anchor_lang::Accounts;
use anchor_lang::context::Context;

#[derive(Accounts)]
pub struct MockAttestationCtx {}

pub fn process(ctx: Context<MockAttestationCtx>) -> anchor_lang::Result<()> {
 Ok(())
}