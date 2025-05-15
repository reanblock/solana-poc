use anchor_lang::prelude::*;

declare_id!("F34cPavCtpxoH3xfvXWw3nRYrK12X8U9Vb5kcAcAVJDK");

#[program]
pub mod solana_poc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
