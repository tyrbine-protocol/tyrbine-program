use crate::{components::check_admin, states::{Vault, Treasury}, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

#[inline(never)]
pub fn init_vault(
    ctx: Context<InitVaultInstructionAccounts>,
    is_active: bool,
    base_fee: u64,
    max_age_price: u64
) -> Result<()> {
    check_admin(&ctx.accounts.treasury_pda, &ctx.accounts.signer)?;

    ctx.accounts.vault_pda.is_active = is_active;
    ctx.accounts.vault_pda.base_fee = base_fee;
    ctx.accounts.vault_pda.token_mint = ctx.accounts.vault_mint.key();
    ctx.accounts.vault_pda.pyth_price_account = ctx.accounts.pyth_price_account.key();
    ctx.accounts.vault_pda.max_age_price = max_age_price;
    ctx.accounts.vault_pda.lp_mint = ctx.accounts.lp_mint.key();
    ctx.accounts.vault_pda.initial_liquidity = 0;
    ctx.accounts.vault_pda.current_liquidity = 0;
    ctx.accounts.vault_pda.cumulative_yield_per_lp = 0;
    ctx.accounts.vault_pda.protocol_yield = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct InitVaultInstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    pub vault_mint: Account<'info, Mint>,

    pub pyth_price_account: Account<'info, PriceUpdateV2>,

    #[account(
        init,
        payer = signer,
        seeds = [MINT_SEED.as_bytes(), vault_pda.key().as_ref()], 
        bump,
        mint::decimals = vault_mint.decimals,
        mint::authority = treasury_pda.key(),
        mint::freeze_authority = treasury_pda.key()
    )]
    pub lp_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = signer,
        seeds = [VAULT_SEED.as_bytes(), vault_mint.key().as_ref()],
        bump,
        space = 8 + 1 + 8 + 32 + 32 + 8 + 32 + 8 + 8 + 16 + 8,
    )]
    pub vault_pda: Account<'info, Vault>,

    #[account(mut, seeds = [TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes()], bump)]
    pub treasury_pda: Account<'info, Treasury>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
