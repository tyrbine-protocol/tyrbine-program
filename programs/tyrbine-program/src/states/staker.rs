use anchor_lang::prelude::*;

#[account]
pub struct Staker {
    pub last_cumulative_yield: u64,
    pub pending_claim: u64
}