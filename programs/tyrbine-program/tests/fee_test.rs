#[cfg(test)]
mod tyrbine {

    use tyrbine_program::components::fee;
    

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

    
}
