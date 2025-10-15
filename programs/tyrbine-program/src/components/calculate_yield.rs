pub fn calculate_yield(
    cumulative_yield: u64,
    total_lp: u64,
    staker_lp_balance: u64,
    last_cumulative_yield: u64,
) -> u64 {
    let cumulative_yield_unscale = match cumulative_yield.checked_sub(last_cumulative_yield) {
        Some(val) => val,
        None => return 0,
    };

    if total_lp == 0 {
        return 0;
    }

    let staker_yield = (cumulative_yield_unscale as u128)
        .checked_mul(staker_lp_balance as u128)
        .unwrap_or(0)
        / total_lp as u128;

    staker_yield as u64
}
