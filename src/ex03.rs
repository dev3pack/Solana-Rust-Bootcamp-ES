//! ex03 - Ownership.
//!
//! Practico los conceptos de `tasks/ex03.md` con ejemplos de cuentas.

/// Recibo el nombre por valor y construyo su etiqueta.
pub fn make_label(owner: String) -> String {
    format!("account:{owner}")
}

/// Creo un String propio a partir del texto prestado.
pub fn duplicate(text: &str) -> String {
    text.to_string()
}

/// Consulto la longitud en bytes antes de devolver el String y moverlo.
pub fn take_and_return(account: String) -> (String, usize) {
    let length = account.len();
    (account, length)
}

/// Recibo el vector por valor y devuelvo su cantidad de elementos.
pub fn consume_all(accounts: Vec<String>) -> usize {
    accounts.len()
}
