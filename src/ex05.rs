//! ex05 - Structs e `impl`.
//!
//! Practico los conceptos de `tasks/ex05.md` con ejemplos de cuentas.

/// Represento una cuenta simplificada; owner identifica al programa dueño.
#[derive(Debug, Clone, PartialEq)]
pub struct Account {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
}

impl Account {
    /// Creo una cuenta vacía cuyo programa dueño es system.
    pub fn new(address: String) -> Self {
        Self {
            address,
            lamports: 0,
            owner: String::from("system"),
        }
    }

    /// Sumo el depósito al saldo.
    pub fn deposit(&mut self, amount: u64) {
        self.lamports += amount;
    }

    /// Verifico los fondos antes de restar; si no alcanzan, devuelvo false.
    pub fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.lamports {
            return false;
        }
        self.lamports -= amount;
        true
    }

    /// Consulto el saldo en SOL enteros, sin modificar la cuenta.
    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / crate::ex01::LAMPORTS_PER_SOL
    }
}
