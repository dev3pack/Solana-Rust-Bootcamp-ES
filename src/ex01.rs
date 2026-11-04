//! ex01 - Variables y tipos.
//!
//! Lee primero `tasks/ex01.md`. Reemplaza cada `todo!()` con una implementación.

/// La cantidad de lamports en un SOL.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Convierte SOL enteros a lamports.
pub fn sol_to_lamports(sol: u64) -> u64 {
    sol * LAMPORTS_PER_SOL
}

/// Convierte lamports a SOL enteros, descartando el residuo.
pub fn lamports_to_sol(lamports: u64) -> u64 {
    lamports / LAMPORTS_PER_SOL
}

/// Suma un bono a un saldo y retorna el nuevo saldo.
pub fn apply_bonus(balance: u64, bonus: u64) -> u64 {
    balance + bonus
}

/// Duplica un valor usando shadowing.
pub fn double(value: u64) -> u64 {
    let value = value * 2;
    value
}

/// Retorna `true` cuando el saldo es cero.
pub fn is_empty(balance: u64) -> bool {
    balance == 0
}