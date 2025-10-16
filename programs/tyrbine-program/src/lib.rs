use anchor_lang::prelude::*;

use instructions::admin::*;
use instructions::staker::*;
use instructions::trader::*;

pub mod states;
pub mod instructions;
pub mod components;
pub mod utils;


declare_id!("FRJbAEJeEzjBVMjyQyeyGRLUEFPpCTHTHjXQ62vmFJPj");

#[program]
pub mod tyrbine_program {
    use super::*;

    pub fn init_treasury(ctx: Context<InitTreasuryInstructionAccounts>) -> Result<()> {
        instructions::admin::init_treasury(ctx)
    }

    pub fn update_treasury(ctx: Context<UpdateTreasuryInstructionAccounts>, stoptap: bool) -> Result<()> {
        instructions::admin::update_treasury(ctx, stoptap)
    }

    pub fn init_vault(ctx: Context<InitVaultInstructionAccounts>, is_active: bool, base_fee: u64) -> Result<()> {
        instructions::admin::init_vault(ctx, is_active, base_fee)
    }

    pub fn update_vault(ctx: Context<UpdatePoolInstructionAccounts>, is_active: bool, base_fee: u64) -> Result<()> {
        instructions::admin::update_vault(ctx, is_active, base_fee)
    }

    // Staker instructions
    pub fn staking(ctx: Context<StakingInstructionAccounts>, amount: u64) -> Result<()> {
        instructions::staker::staking(ctx, amount)
    }

    pub fn unstaking(ctx: Context<UnstakingInstructionAccounts>, amount: u64) -> Result<()> {
        instructions::staker::unstaking(ctx, amount)
    }

    pub fn claim(ctx: Context<ClaimInstructionAccounts>) -> Result<()> {
        instructions::staker::claim(ctx)
    }

    // Trader instruction
    pub fn swap(ctx: Context<SwapInstructionAccounts>, amount_in: u64, partner_fee: u64) -> Result<()> {
        instructions::trader::swap(ctx, amount_in, partner_fee)
    }
    
}
