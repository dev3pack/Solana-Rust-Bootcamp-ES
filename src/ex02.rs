//! ex02 - Funciones y control de flujo.
//!
//! Lee primero `tasks/ex02.md`. Reemplaza cada `todo!()` con una implementación.

/// La comisión base de una transacción, en lamports.
///
/// Solana cobra 5_000 lamports por firma. Esta es solo la comisión base; las
/// comisiones de prioridad se suman por separado.
pub fn base_fee(signatures: u64) -> u64 {
    signatures * 5000
}

/// Suma todos los montos del slice.
///
/// Un slice vacío suma cero.
pub fn total(amounts: &[u64]) -> u64 {
    let mut c: u64 = 0;
    for amount in amounts {
        c += amount;
    }
    c
}

/// Cuenta cuántos montos son estrictamente mayores que `threshold`.
pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    let mut is_higher = 0;
    for amount in amounts {
        if *amount > threshold {
            is_higher += 1;
        }
    }
    is_higher
}

/// Nunca transfieras más que el saldo.
///
/// Retorna el monto que realmente se puede transferir.
pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    // A diferencia de js, rust no retorna valores en 
    // if statments si no tiene el else 
    // if amount > balance {
    //     balance
    // }

    // amount

    if amount > balance {
        balance
    } else {
        amount
    }
}
