use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount}};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{components::{amount_out, check_stoptap, fee}, states::{Vault, Treasury}, utils::{TyrbineError, TREASURY_SEED, TYRBINE_SEED, VAULT_SEED}};

pub fn swap(
    ctx: Context<SwapInstructionAccounts>,
    amount_in: u64,
    partner_fee: u64,
) -> Result<()> {

    check_stoptap(&ctx.accounts.vault_pda_in, &ctx.accounts.treasury_pda)?;
    check_stoptap(&ctx.accounts.vault_pda_out, &ctx.accounts.treasury_pda)?;

    let vault_a = &mut ctx.accounts.vault_pda_in;
    let vault_b = &mut ctx.accounts.vault_pda_out;

    let delta_a = vault_a.initial_liquidity as i64 - vault_a.current_liquidity as i64;
    let delta_b = vault_b.initial_liquidity as i64 - vault_b.current_liquidity as i64;

    if delta_a < delta_b {
        return Err(TyrbineError::SwitchOff.into());
    }

    if ctx.accounts.pyth_price_account_in.key() != vault_a.pyth_price_account {
        return Err(TyrbineError::InvalidPythAccount.into());
    }
    
    if ctx.accounts.pyth_price_account_out.key() != vault_b.pyth_price_account {
        return Err(TyrbineError::InvalidPythAccount.into());
    }

    let price_a = ctx.accounts.pyth_price_account_in.price_message.price as u64;
    let price_b = ctx.accounts.pyth_price_account_out.price_message.price as u64;

    msg!("Token A price: {}", price_a);
    msg!("Token B price: {}", price_b);

    let token_a_decimals = ctx.accounts.mint_in.decimals;
    let token_b_decimals = ctx.accounts.mint_out.decimals;

    let token_b_out = amount_out(amount_in, token_a_decimals, token_b_decimals, price_a, price_b)?;

    let (after_fee, lp_fee, protocol_fee, partner_fee) = fee(token_b_out, vault_b.base_fee, 1, partner_fee);
    
    msg!("Amount In: {}", amount_in);
    msg!("Amount Out: {}", after_fee);
    msg!("LP Fee: {}", lp_fee);
    msg!("Protocol Fee: {}", protocol_fee);
    msg!("Partner Fee: {}", partner_fee);

    vault_a.current_liquidity += amount_in;
    vault_b.current_liquidity -= after_fee;

    vault_b.cumulative_yield += lp_fee;

    let cpi_accounts = token::Transfer {
        from: ctx.accounts.signer_ata_in.to_account_info(),
        to: ctx.accounts.treasury_ata_in.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };

    token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts), amount_in)?;
    
    let seeds = &[TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes(), &[ctx.bumps.treasury_pda]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = token::Transfer {
        from: ctx.accounts.treasury_ata_out.to_account_info(),
        to: ctx.accounts.signer_ata_out.to_account_info(), 
        authority: ctx.accounts.treasury_pda.to_account_info()
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            cpi_accounts, 
            signer_seeds), 
        after_fee)?;
    
    if partner_fee > 0 {
        let partner_fee_account = ctx.accounts.partner_fee_ata.as_ref().ok_or(TyrbineError::MissingSPLAccount)?;

        let seeds = &[TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes(), &[ctx.bumps.treasury_pda]];
        let signer_seeds = &[&seeds[..]];
    
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.treasury_ata_out.to_account_info(),
            to: partner_fee_account.to_account_info(), 
            authority: ctx.accounts.treasury_pda.to_account_info()
        };
    
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(), 
                cpi_accounts, 
                signer_seeds), 
                partner_fee)?;
    }

    Ok(())
}

#[derive(Accounts)]
pub struct SwapInstructionAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK:
    pub mint_in: Account<'info, Mint>,

    /// CHECK:
    pub mint_out: Account<'info, Mint>,

    /// CHECK: pool_pda.pyth
    pub pyth_price_account_in: Account<'info, PriceUpdateV2>,

    /// CHECK: pool_pda.pyth
    pub pyth_price_account_out: Account<'info, PriceUpdateV2>,

    #[account(mut, token::authority = signer, token::mint = mint_in)]
    pub signer_ata_in: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_out,
        associated_token::authority = signer,
    )]
    pub signer_ata_out: Account<'info, TokenAccount>,

    /// CHECK:
    #[account(mut, seeds = [VAULT_SEED.as_bytes(), mint_in.key().as_ref()], bump)]
    pub vault_pda_in: Account<'info, Vault>,

    /// CHECK:
    #[account(mut, seeds = [VAULT_SEED.as_bytes(), mint_out.key().as_ref()], bump)]
    pub vault_pda_out: Account<'info, Vault>,

    #[account(mut, seeds = [TYRBINE_SEED.as_bytes(), TREASURY_SEED.as_bytes()], bump)]
    pub treasury_pda: Account<'info, Treasury>,

    #[account(mut, token::authority = treasury_pda, token::mint = mint_in)]
    pub treasury_ata_in: Account<'info, TokenAccount>,

    #[account(mut, token::authority = treasury_pda, token::mint = mint_out)]
    pub treasury_ata_out: Account<'info, TokenAccount>,

    #[account(mut)]
    pub partner_fee_ata: Option<AccountInfo<'info>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
