use anchor_lang::prelude::*;

declare_id!("A9FV1a1U1w1TkfAA6JWeH2qUaRJJrTyqhVoeomJUBVzr");

#[program]
pub mod solid_svm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
