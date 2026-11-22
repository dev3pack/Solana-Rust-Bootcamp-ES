//! ex04 - Borrowing.
//!
//! Lee primero `tasks/ex04.md`. Reemplaza cada `todo!()` con una implementación.

/// Lee un slice de montos y retorna su suma.
///
/// El slice está prestado, así que quien llama conserva el ownership.
pub fn total(amounts: &[u64]) -> u64 {
    let mut suma = 0;
    for amount in amounts {
        suma += *amount;
    }
    suma
}

/// Suma una comisión a un saldo a través de una referencia mutable.
///
/// La función no retorna nada. El valor de quien llama debe cambiar.
pub fn add_fee(amount: &mut u64, fee: u64) {
    *amount = *amount + fee;
}

/// Cuenta cuántas cuentas son escribibles (writable).
pub fn count_writable(flags: &[bool]) -> usize {
    let mut contador = 0;
    for flag in flags {
        if *flag {
            contador += 1;
        }
    }
    contador
}

/// Agrega una nota a un historial a través de una referencia mutable.
pub fn append_note(history: &mut Vec<String>, note: &str) {
    let aux = note.to_string();
    history.push(aux)
}
