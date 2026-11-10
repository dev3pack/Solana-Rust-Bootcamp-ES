//! ex02 - Funciones y control de flujo.
//!
//! Lee primero `tasks/ex02.md`. Reemplaza cada `todo!()` con una implementación.

/// La comisión base de una transacción, en lamports.
///
/// Solana cobra 5_000 lamports por firma. Esta es solo la comisión base; las
/// comisiones de prioridad se suman por separado.
pub fn base_fee(signatures: u64) -> u64 {
    let fee= signatures * 5_000;
    fee
}

/// Suma todos los montos del slice.
///
/// Un slice vacío suma cero.
pub fn total(amounts: &[u64]) -> u64 {
    let mut sum = 0; //aca llevamos la cuenta de la suma
    for amount in amounts {
        sum += amount;
    }
    sum
}

/// Cuenta cuántos montos son estrictamente mayores que `threshold`.
pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    let mut contador = 0;
    for amount in amounts {
        if *amount > threshold {
            contador += 1;
        }
    }
    contador
}

/// Nunca transfieras más que el saldo.
///
/// Retorna el monto que realmente se puede transferir.
pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    if amount > balance {
        balance
    } else {
        amount
    }
}
