use crate::{components::check_admin, states::Treasury, utils::{TREASURY_SEED, TYRBINE_SEED}};
use anchor_lang::prelude::*;

#[inline(never)]
pub fn update_treasury(
    ctx: Context<UpdateTreasuryInstructionAccounts>,
    stoptap: bool,
    proto_fee: u64
) -> Result<()> {
    check_admin(&ctx.accounts.treasury_pda, &ctx.accounts.signer)?;
    
    ctx.accounts.treasury_pda.admin = ctx.accounts.new_admin.key();
    ctx.accounts.treasury_pda.stoptap = stoptap;
    ctx.accounts.treasury_pda.proto_fee = proto_fee;

    msg!("New admin: {}", ctx.accounts.treasury_pda.admin);
    msg!("Stoptap: {}", ctx.accounts.treasury_pda.stoptap);
    msg!("Protocol fee: {}", ctx.accounts.treasury_pda.proto_fee);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateTreasuryInstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    pub new_admin: AccountInfo<'info>,

    #[account(mut, seeds = [TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes()], bump)]
    pub treasury_pda: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}
