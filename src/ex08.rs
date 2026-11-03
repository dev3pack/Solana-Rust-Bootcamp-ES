//! ex08 - Proyecto final: una mini cuenta bancaria.
//!
//! Practico los conceptos de `tasks/ex08.md` con ejemplos de cuentas.
//! Nada de `HashMap` ni traits: solo un struct, un historial `Vec<String>` y
//! `Result`.

/// Distingo los errores esperados del mini banco.
#[derive(Debug, PartialEq)]
pub enum BankError {
    ZeroAmount,
    InsufficientFunds { available: u64, required: u64 },
}

/// Agrupo la titular, el saldo y el historial de la cuenta.
#[derive(Debug, Clone, PartialEq)]
pub struct BankAccount {
    pub owner: String,
    pub lamports: u64,
    pub history: Vec<String>,
}

impl BankAccount {
    /// Creo una cuenta vacía y guardo una copia del nombre.
    pub fn new(owner: &str) -> Self {
        Self {
            owner: owner.to_string(),
            lamports: 0,
            history: Vec::new(),
        }
    }

    /// Rechazo cero antes de sumar y registrar el depósito.
    pub fn deposit(&mut self, amount: u64) -> Result<(), BankError> {
        if amount == 0 {
            return Err(BankError::ZeroAmount);
        }
        self.lamports += amount;
        self.history.push(format!("deposit {amount}"));
        Ok(())
    }

    /// Valido el retiro antes de modificar el saldo y el historial.
    pub fn withdraw(&mut self, amount: u64) -> Result<(), BankError> {
        if amount == 0 {
            return Err(BankError::ZeroAmount);
        }
        if amount > self.lamports {
            return Err(BankError::InsufficientFunds {
                available: self.lamports,
                required: amount,
            });
        }
        self.lamports -= amount;
        self.history.push(format!("withdraw {amount}"));
        Ok(())
    }

    /// Consulto el saldo en SOL enteros.
    pub fn balance_in_sol(&self) -> u64 {
        self.lamports / crate::ex01::LAMPORTS_PER_SOL
    }
}
