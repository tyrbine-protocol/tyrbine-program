pub fn calculate_fee_amount(amount_out: u64, fee: u64, protocol_fee: u64, partner_fee: u64) -> (u64, u64, u64, u64) {

    let total_fee = amount_out
        .checked_mul(fee) 
        .unwrap()
        .checked_div(10_000)
        .unwrap();

    let protocol_fee_amount = amount_out
        .checked_mul(protocol_fee)
        .unwrap()
        .checked_div(10_000)
        .unwrap();

    let partner_fee_amount = amount_out
        .checked_mul(partner_fee)
        .unwrap()
        .checked_div(10_000)
        .unwrap();

    let lp_fee = total_fee
        .checked_sub(protocol_fee_amount)
        .unwrap()
        .checked_sub(partner_fee_amount)
        .unwrap();

    let amount_after_fee = amount_out.checked_sub(total_fee).unwrap();

    (amount_after_fee as u64, lp_fee as u64, protocol_fee_amount as u64, partner_fee_amount as u64)
}
