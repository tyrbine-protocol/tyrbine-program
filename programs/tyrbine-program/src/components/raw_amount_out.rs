use crate::utils::TyrbineError;

pub fn raw_amount_out(amount_in: u64, decimals_in: u8, decimals_out: u8, price_in: u64, price_out: u64) -> Result<u64, TyrbineError> {
    
    let decimals_diff = decimals_out as i32 - decimals_in as i32;

    let adjusted_amount_in = if decimals_diff > 0 {
        let _scale = 10u64.checked_pow(decimals_diff as u32).ok_or(TyrbineError::OverflowInPow)?;
        amount_in.checked_mul(_scale).ok_or(TyrbineError::OverflowInMul)?
    } else {
        let _scale = 10u64.checked_pow(-decimals_diff as u32).ok_or(TyrbineError::OverflowInPow)?;
        amount_in.checked_div(_scale).ok_or(TyrbineError::OverflowInDiv)?
    };

    let raw_amount_out = adjusted_amount_in
        .checked_mul(price_in)
        .ok_or(TyrbineError::OverflowInMul)?
        .checked_div(price_out)
        .ok_or(TyrbineError::OverflowInDiv)?;

    Ok(raw_amount_out)
}