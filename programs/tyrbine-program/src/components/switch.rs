use crate::utils::TyrbineError;

/// Checks whether a switch operation is allowed based on delta values.
/// Returns an error if `delta_a` is smaller than `delta_b`.
pub fn switch(delta_a: i64, delta_b: i64) -> Result<(), TyrbineError> {
    if delta_a < delta_b {
        return Err(TyrbineError::SwitchOff);
    }
    Ok(())
}