use crate::{components::check_admin, states::{Vault, Treasury}, utils::{VAULT_SEED, TYRBINE_SEED}};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use pyth_solana_receiver_sdk::{pda::TREASURY_SEED, price_update::PriceUpdateV2};

pub fn update_vault(
    ctx: Context<UpdatePoolInstructionAccounts>,
    is_active: bool,
    base_fee: u64,
) -> Result<()> {
    check_admin(&ctx.accounts.treasury_pda, &ctx.accounts.signer)?;

    ctx.accounts.vault_pda.base_fee = base_fee;
    ctx.accounts.vault_pda.is_active = is_active;
    ctx.accounts.vault_pda.pyth_price_account = ctx.accounts.pyth_price_account.key();

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePoolInstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    pub vault_mint: Account<'info, Mint>,

    pub pyth_price_account: Account<'info, PriceUpdateV2>,

    #[account(mut, seeds = [VAULT_SEED.as_bytes(), &vault_mint.key().to_bytes()], bump)]
    pub vault_pda: Account<'info, Vault>,

    #[account(mut, seeds = [TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes()], bump)]
    pub treasury_pda: Account<'info, Treasury>,

    pub system_program: Program<'info, System>,
}
