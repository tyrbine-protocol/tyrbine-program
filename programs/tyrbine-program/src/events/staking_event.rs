use anchor_lang::prelude::*;

#[event]
pub struct StakingEvent {
    pub staker: Pubkey,
    pub token: Pubkey,
    pub staking_amount: u64,
    pub timestamp: i64,
}