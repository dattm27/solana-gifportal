use anchor_lang::prelude::*;

declare_id!("9FrSRz7QnsyPA5dpKhpGkjZCNopRjYNr2iCnL3iEfx2J");

#[program]
pub mod gifportal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
