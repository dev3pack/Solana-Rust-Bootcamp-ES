//! ex06 - Enums y `match`.
//!
//! Practico los conceptos de `tasks/ex06.md` con ejemplos de cuentas.

/// Represento los datos de una instrucción; las cuentas se pasan por separado.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Transfer {
        from: String,
        to: String,
        lamports: u64,
    },
    CreateAccount {
        address: String,
    },
    Close(String),
}

/// Identifico cada variante con un match exhaustivo.
pub fn kind(ix: &Instruction) -> &'static str {
    match ix {
        Instruction::Transfer { .. } => "transfer",
        Instruction::CreateAccount { .. } => "create",
        Instruction::Close(_) => "close",
    }
}

/// Leo los lamports de Transfer; en las otras variantes devuelvo cero.
pub fn lamports_moved(ix: &Instruction) -> u64 {
    match ix {
        Instruction::Transfer { lamports, .. } => *lamports,
        Instruction::CreateAccount { .. } => 0,
        Instruction::Close(_) => 0,
    }
}

/// Devuelvo una referencia a la dirección guardada en la instrucción.
pub fn account_touched(ix: &Instruction) -> &str {
    match ix {
        Instruction::Transfer { from, .. } => from,
        Instruction::CreateAccount { address } => address,
        Instruction::Close(address) => address,
    }
}
