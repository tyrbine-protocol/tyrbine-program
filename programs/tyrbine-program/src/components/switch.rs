use crate::utils::TyrbineError;

pub fn switch(delta_a: i64, delta_b: i64) -> Result<(), TyrbineError> {
    if delta_a < delta_b {
        return Err(TyrbineError::SwitchOff);
    }
    Ok(())
}