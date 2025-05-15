use crate::state::Counter;
use crate::constants::DISCRIMINATOR;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(init,
        payer = user,
        space = DISCRIMINATOR + Counter::INIT_SPACE
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateCounter<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    counter.count = 0;
    msg!("Counter Account Created");
    msg!("Current Count: { }", counter.count);
    Ok(())
}

pub fn increment(ctx: Context<UpdateCounter>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    msg!("Previous counter: {}", counter.count);
    counter.count = counter.count.checked_add(1).unwrap();
    msg!("Counter incremented. Current count: {}", counter.count);
    Ok(())
}