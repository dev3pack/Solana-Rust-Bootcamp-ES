//! ex01 - Variables y tipos.
//!
//! Practico los conceptos de `tasks/ex01.md` con ejemplos de cuentas.

/// Uso mil millones de lamports por SOL.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

/// Convierto SOL enteros a lamports.
pub fn sol_to_lamports(sol: u64) -> u64 {
    sol * LAMPORTS_PER_SOL
}

/// Convierto lamports a SOL enteros y descarto el resto.
pub fn lamports_to_sol(lamports: u64) -> u64 {
    lamports / LAMPORTS_PER_SOL
}

/// Sumo el bono con una variable mutable.
pub fn apply_bonus(balance: u64, bonus: u64) -> u64 {
    let mut balance = balance;
    balance += bonus;
    balance
}

/// Duplico el valor con shadowing: declaro otra variable con el mismo nombre.
pub fn double(value: u64) -> u64 {
    let value = value * 2;
    value
}

/// Compruebo si el saldo es cero.
pub fn is_empty(balance: u64) -> bool {
    balance == 0
}
