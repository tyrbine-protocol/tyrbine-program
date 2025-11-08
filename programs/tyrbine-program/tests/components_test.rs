#[cfg(test)]
mod tyrbine {

    use anchor_lang::prelude::Pubkey;
    use tyrbine_program::{components::{calculate_fee_amount, calculate_yield, fees_setting, raw_amount_out}, states::Vault, utils::SCALE};
    

#[test]
fn calculating_fee_amount() {
    let amount_in: u64 = 1_000_000_000_000_000; // Input amount in BONK atoms (1e16)

    // Call the fee function: returns (amount after all fees, LP fee, partner fee, protocol fee)
    let (after_fee, lp_fee, protocol_fee, partner_fee) = calculate_fee_amount(amount_in, 10, 1, 0);

    // Print results for clarity
    println!("Input: {}", amount_in);
    println!("After fee: {}", after_fee);
    println!("LP fee (0.09%): {}", lp_fee);
    println!("Protocol fee (0.01%): {}", protocol_fee);
    println!("Partner fee: {}", partner_fee);

    // Check that the sum after distributing all fees equals the original amount
    let total: u64 = after_fee + lp_fee + partner_fee + protocol_fee;
    assert_eq!(total, amount_in as u64, "The total after distributing fees does not equal the original amount");
}

#[test]
fn calculating_yield_per_lp() {
    let lp_fee: u128 = 39960; 
    let total_lp: u64 = 2000000000000;

    let cumulative_yield_per_lp = (lp_fee as u128 * SCALE) / total_lp as u128;

    println!("Cum Yield Per Lp: {}", cumulative_yield_per_lp);
}

#[test]
fn calculating_yield() {
    let cumulative_yield_per_lp: u128 = 1998; 
    let staker_lp_balance = 2000000000;
    let last_cumulative_yield: u128 = 0;

    let yield_amount = calculate_yield(cumulative_yield_per_lp, staker_lp_balance, last_cumulative_yield);

    println!("Yield: {}", yield_amount);
}


#[test]
fn testing_raw_amount_out() {
    let amount_in: u64 = 24604301;
    let price_a: u64 = 99984320;
    let price_b: u64 = 24600443821;
    let token_a_decimals: u8 = 6;
    let token_b_decimals: u8 = 9;

    let amount = raw_amount_out(amount_in, token_a_decimals, token_b_decimals, price_a, price_b);

    println!("Amount out: {:?}", amount);
    //assert_eq!(amount_out, 24604301);
}

#[test]
fn testing_fees_setting() {
    let pubkey = Pubkey::default();
    let vault_in = Vault {base_fee: 10, initial_liquidity: 1, current_liquidity: 1, is_active: true, token_mint: pubkey, pyth_price_account: pubkey, max_age_price: 300, lp_mint: pubkey, cumulative_yield_per_lp: 0, protocol_yield: 0};
    let vault_out = Vault{ base_fee: 10, initial_liquidity: 1, current_liquidity: 1, is_active: true, token_mint: pubkey, pyth_price_account: pubkey, max_age_price: 300, lp_mint: pubkey, cumulative_yield_per_lp: 0, protocol_yield: 0};
    let proto_fee_bps: u64 = 10;

    let fees = fees_setting(&vault_in, &vault_out, proto_fee_bps);

    println!("Swap fee: {:?}", fees.0);
    println!("Protocol fee: {:?}", fees.1);
    //assert_eq!(amount_out, 24604301);
}
    
}