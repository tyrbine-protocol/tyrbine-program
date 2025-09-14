use crate::{states::Treasury, utils::{TREASURY_SEED, TYRBINE_SEED}};
use anchor_lang::prelude::*;

#[inline(never)]
pub fn init_treasury(
    ctx: Context<InitTreasuryInstructionAccounts>,
) -> Result<()> {
    ctx.accounts.treasury_pda.admin = ctx.accounts.signer.key();
    ctx.accounts.treasury_pda.stoptap = false;

    Ok(())
}

#[derive(Accounts)]
pub struct InitTreasuryInstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes()],
        bump,
        space = 8 + 1 + 32,
    )]
    pub treasury_pda: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}
