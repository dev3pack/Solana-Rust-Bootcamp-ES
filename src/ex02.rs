//! ex02 - Funciones y control de flujo.
//!
//! Practico los conceptos de `tasks/ex02.md` con ejemplos de cuentas.

/// Calculo la comisión base del ejercicio: 5.000 lamports por firma.
pub fn base_fee(signatures: u64) -> u64 {
    signatures * 5_000
}

/// Sumo los montos prestados; si no hay elementos, devuelvo cero.
pub fn total(amounts: &[u64]) -> u64 {
    let mut sum = 0;
    for amount in amounts {
        sum += *amount;
    }
    sum
}

/// Cuento los montos que superan el umbral.
pub fn count_above(amounts: &[u64], threshold: u64) -> usize {
    let mut count = 0;
    for amount in amounts {
        if *amount > threshold {
            count += 1;
        }
    }
    count
}

/// Limito el monto al saldo disponible.
pub fn clamp_transfer(amount: u64, balance: u64) -> u64 {
    if amount > balance {
        balance
    } else {
        amount
    }
}
