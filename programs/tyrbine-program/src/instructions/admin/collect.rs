use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{components::check_admin, states::{Treasury, Vault}, utils::{TREASURY_SEED, TYRBINE_SEED, VAULT_SEED}};


pub fn collect(ctx: Context<CollectInstructionAccounts>) -> Result<()> {
    check_admin(&ctx.accounts.treasury_pda, &ctx.accounts.signer)?;

    let protocol_yield = ctx.accounts.vault_pda.protocol_yield;

    // Transfer [amount] to Signer
    let seeds = &[TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes(), &[ctx.bumps.treasury_pda]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.treasury_ata.to_account_info(),
        to: ctx.accounts.signer_ata.to_account_info(), 
        authority: ctx.accounts.treasury_pda.to_account_info()
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            cpi_accounts, 
            signer_seeds), 
        protocol_yield)?;

    ctx.accounts.vault_pda.protocol_yield -= protocol_yield;

    msg!("Collected: {}", protocol_yield);

    Ok(())
}

#[derive(Accounts)]
pub struct CollectInstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    pub vault_mint: Account<'info, Mint>,

    #[account(mut, token::authority = signer, token::mint = vault_mint)]
    pub signer_ata: Account<'info, TokenAccount>,

    #[account(mut, seeds = [VAULT_SEED.as_bytes(), vault_mint.key().as_ref()], bump)]
    pub vault_pda: Account<'info, Vault>,

    #[account(mut, seeds = [TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes()], bump)]
    pub treasury_pda: Account<'info, Treasury>,

    #[account(
        mut,
        associated_token::mint = vault_mint,
        associated_token::authority = treasury_pda,
    )]
    pub treasury_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
