#[cfg(test)]
mod tyrbine {

    use tyrbine_program::components::{fee, calculating_yield, amount_out};
    

#[test]
fn testing_fee() {
    let amount_in: u64 = 1_000_000_000_000_000; // Input amount in BONK atoms (1e16)

    // Call the fee function: returns (amount after all fees, LP fee, partner fee, protocol fee)
    let (after_fee, lp_fee, protocol_fee, partner_fee) = fee(amount_in, 10, 1, 0);

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
fn calculating_fee() {
    let cumulative_yield: u64 = 1000000; 
    let total_lp = 1000000;
    let staker_lp_balance = 100000;
    let last_cumulative_yield = 0;

    let yield_amount = calculating_yield(cumulative_yield, total_lp, staker_lp_balance, last_cumulative_yield);

    println!("Yield: {}", yield_amount);
}


#[test]
fn testing_amount_out() {
    let amount_in: u64 = 24604301;
    let price_a: u64 = 99984320;
    let price_b: u64 = 24600443821;
    let token_a_decimals: u8 = 6;
    let token_b_decimals: u8 = 9;

    let amount = amount_out(amount_in, token_a_decimals, token_b_decimals, price_a, price_b);

    println!("Amount out: {:?}", amount);
    //assert_eq!(amount_out, 24604301);
}

    
}