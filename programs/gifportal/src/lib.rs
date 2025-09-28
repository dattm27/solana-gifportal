use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;



declare_id!("9FrSRz7QnsyPA5dpKhpGkjZCNopRjYNr2iCnL3iEfx2J");

#[program]
pub mod gifportal {

    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult{
        Ok(())
    }
    
}


#[derive(Accounts)]
pub struct StartStuffOff {

}

