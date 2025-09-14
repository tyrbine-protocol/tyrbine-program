/// Calculate the yield for a staker based on cumulative yield and LP balances.
/// Returns 0 if total_lp is 0 or cumulative_yield < last_cumulative_yield
pub fn calculating_yield(
    cumulative_yield: u64,
    total_lp: u64,
    staker_lp_balance: u64,
    last_cumulative_yield: u64,
) -> u64 {
    // Check for underflow
    let cumulative_yield_unscale = match cumulative_yield.checked_sub(last_cumulative_yield) {
        Some(val) => val,
        None => return 0, // underflow, return 0 yield
    };

    // Avoid division by zero
    if total_lp == 0 {
        return 0;
    }

    // Use u128 to prevent overflow during multiplication
    let staker_yield = (cumulative_yield_unscale as u128)
        .checked_mul(staker_lp_balance as u128)
        .unwrap_or(0)
        / total_lp as u128;

    staker_yield as u64
}
