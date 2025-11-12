use anchor_lang::prelude::*;

#[event]
pub struct UnstakingEvent {
    pub staker: Pubkey,
    pub token: Pubkey,
    pub unstaking_amount: u64,
    pub timestamp: i64,
}