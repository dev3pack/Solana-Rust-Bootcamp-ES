//! ex07 - `Result` y el operador `?`.
//!
//! Practico los conceptos de `tasks/ex07.md` con ejemplos de cuentas.

/// Distingo los errores esperados de una transferencia.
#[derive(Debug, PartialEq)]
pub enum TransferError {
    ZeroAmount,
    SameAccount,
    InsufficientFunds { available: u64, required: u64 },
}

/// Rechazo primero el monto cero y después el envío a la misma cuenta.
pub fn validate(from: &str, to: &str, amount: u64) -> Result<(), TransferError> {
    if amount == 0 {
        return Err(TransferError::ZeroAmount);
    }
    if from == to {
        return Err(TransferError::SameAccount);
    }
    Ok(())
}

/// Valido los datos y los fondos antes de calcular el saldo restante.
pub fn transfer(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    validate(from, to, amount)?;
    if amount > balance {
        return Err(TransferError::InsufficientFunds {
            available: balance,
            required: amount,
        });
    }
    Ok(balance - amount)
}

/// Propago el error de transfer con ? y devuelvo el saldo si hay éxito.
pub fn pay(balance: u64, from: &str, to: &str, amount: u64) -> Result<u64, TransferError> {
    let remaining = transfer(balance, from, to, amount)?;
    Ok(remaining)
}
