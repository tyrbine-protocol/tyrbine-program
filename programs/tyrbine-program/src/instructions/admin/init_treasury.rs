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
    // Charged from the total fees. Indicated in whole percentages (e.g. 10 = 10%)
    // Minimum proto_fee = 10 when swap_fee_bps = 10 in vault. If proto_fee is lower, the protocol will not receive income. Increase swap_fee_bps > 100 and then you can set proto_fee < 10
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
