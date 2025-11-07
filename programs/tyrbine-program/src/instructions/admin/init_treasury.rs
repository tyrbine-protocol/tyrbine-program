use crate::{states::Treasury, utils::{TREASURY_SEED, TYRBINE_SEED, TyrbineError}};
use anchor_lang::prelude::*;

#[inline(never)]
pub fn init_treasury(
    ctx: Context<InitTreasuryInstructionAccounts>,
    proto_fee: u64
) -> Result<()> {
    // Check Signer
    if ctx.accounts.signer.key.to_string() != "aZZ8CAZ1b1Ar3x4UoB6QxTeobpg5DusHYDM1NpLX8mQ" {
        return Err(TyrbineError::InvalidAdmin.into());
    }

    ctx.accounts.treasury_pda.admin = ctx.accounts.signer.key();
    ctx.accounts.treasury_pda.stoptap = false;
    // charged from the total fees. indicated in whole percentages (e.g. 1 = 10%)
    ctx.accounts.treasury_pda.proto_fee = proto_fee;

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
        space = 8 + 1 + 32 + 8,
    )]
    pub treasury_pda: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}
