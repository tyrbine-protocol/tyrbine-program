/// Apply a 0.1% fee (10 bps), distributed as follows:
/// - protocol_fee bps goes to the protocol
/// - partner_fee bps goes to the partner
/// - the remainder of the total fee goes to LP
pub fn fee(amount_out: u64, fee: u64, protocol_fee: u64, partner_fee: u64) -> (u64, u64, u64, u64) {
    // Total fee (0.1% of amount_out)
    let total_fee = amount_out
        .checked_mul(fee)   // e.g., 10 bps
        .unwrap()
        .checked_div(10_000)
        .unwrap();

    // Protocol fee (protocol_fee bps of amount_out)
    let protocol_fee_amount = amount_out
        .checked_mul(protocol_fee)
        .unwrap()
        .checked_div(10_000)
        .unwrap();

    // Partner fee (partner_fee bps of amount_out)
    let partner_fee_amount = amount_out
        .checked_mul(partner_fee)
        .unwrap()
        .checked_div(10_000)
        .unwrap();

    // LP fee = total fee minus protocol fee minus partner fee
    let lp_fee = total_fee
        .checked_sub(protocol_fee_amount)
        .unwrap()
        .checked_sub(partner_fee_amount)
        .unwrap();

    // Amount after total fee
    let amount_after_fee = amount_out.checked_sub(total_fee).unwrap();

    // Return:
    // (amount after total fee, LP fee, protocol fee, partner fee)
    (amount_after_fee as u64, lp_fee as u64, protocol_fee_amount as u64, partner_fee_amount as u64)
}
