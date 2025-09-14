use anchor_lang::prelude::*;

#[account]
pub struct Pool {
    pub is_active: bool,
    pub base_fee: u64, // scale 1:FEE_SCALE
    pub token_mint: Pubkey,

    pub pyth_price_account: Pubkey,

    pub lp_mint: Pubkey,
    pub initial_liquidity: u64,
    pub current_liquidity: u64,
    
    pub cumulative_yield: u64, // scale 1:SPICE_SCALE
}