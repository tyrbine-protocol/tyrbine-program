use anchor_lang::prelude::*;

#[event]
pub struct ClaimEvent {
    pub staker: Pubkey,
    pub token: Pubkey,
    pub claimed_amount: u64,
    pub last_cumulative_yield_per_token: u128,
    pub pending_claim: u64,
    pub timestamp: i64,
}