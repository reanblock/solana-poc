use anchor_lang::prelude::*;
use instructions::*;

pub mod constants;
pub mod instructions;
pub mod state;
 
declare_id!("F34cPavCtpxoH3xfvXWw3nRYrK12X8U9Vb5kcAcAVJDK");

#[program]
pub mod solana_poc {
    use super::*;

    pub fn initialize_counter(
        ctx: Context<InitializeCounter>,
    ) -> Result<()> {
        instructions::manage_counter::initialize(ctx)
    }

    pub fn increment_counter(
        ctx: Context<UpdateCounter>,
    ) -> Result<()> {
        instructions::manage_counter::increment(ctx)
    }
}