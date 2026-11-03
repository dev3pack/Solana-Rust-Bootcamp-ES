//! ex03 - Ownership.
//!
//! Lee primero `tasks/ex03.md`. Reemplaza cada `todo!()` con una implementación.

/// Construye una etiqueta a partir del nombre de un dueño (con ownership).
///
/// `make_label(String::from("alice"))` debe retornar `"account:alice"`.
/// La función toma ownership del nombre.
pub fn make_label(owner: String) -> String {
    let account = format!("account:{owner}");
    account
}

/// Retorna una copia con ownership de un string prestado.
///
/// La entrada se presta con `&str`, así que quien llama conserva el ownership.
pub fn duplicate(text: &str) -> String {
    let copy = text.to_string();
    copy
}

/// Toma ownership del nombre de una cuenta y lo retorna junto con su longitud.
///
/// Quien llama no puede usar el `String` original después de esta llamada.
pub fn take_and_return(account: String) -> (String, usize) {
    let account_length = account.len();
    let name_with_size = (account, account_length);
    name_with_size
}

/// Consume un vector de nombres de cuentas y retorna cuántos había.
pub fn consume_all(accounts: Vec<String>) -> usize {
    let amount = accounts.len();
    amount
}
