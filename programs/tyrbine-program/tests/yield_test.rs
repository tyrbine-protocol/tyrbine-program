#[cfg(test)]
mod tyrbine {

    use tyrbine_program::components::{calculating_yield};
    

#[test]
fn calculating_fee() {
    let cumulative_yield: u64 = 1000000; 
    let total_lp = 1000000;
    let staker_lp_balance = 100000;
    let last_cumulative_yield = 0;

    let yield_amount = calculating_yield(cumulative_yield, total_lp, staker_lp_balance, last_cumulative_yield);

    println!("Yield: {}", yield_amount);
}

    
}
