//! ex04 - Borrowing.
//!
//! Practico los conceptos de `tasks/ex04.md` con ejemplos de cuentas.

/// Leo los montos prestados y acumulo su suma.
pub fn total(amounts: &[u64]) -> u64 {
    let mut sum = 0;
    for amount in amounts {
        sum += *amount;
    }
    sum
}

/// Modifico el monto original a través de una referencia mutable.
pub fn add_fee(amount: &mut u64, fee: u64) {
    *amount += fee;
}

/// Cuento las marcas verdaderas sin consumir el slice.
pub fn count_writable(flags: &[bool]) -> usize {
    let mut count = 0;
    for flag in flags {
        if *flag {
            count += 1;
        }
    }
    count
}

/// Guardo una copia de la nota en el historial prestado.
pub fn append_note(history: &mut Vec<String>, note: &str) {
    history.push(note.to_string());
}
