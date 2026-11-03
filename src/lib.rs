//! Mi práctica de Rust para Solana.
//!
//! ## Mi recorrido
//!
//! - En ex01 practico tipos, mutabilidad y shadowing. Represento los montos con
//!   enteros en lamports para evitar el redondeo del punto flotante.
//! - En ex02 uso funciones, expresiones y bucles sobre slices prestados.
//!   La última expresión sin punto y coma es el valor que devuelvo.
//! - En ex03 practico ownership: al pasar un String por valor, cedo su propiedad.
//!   Consulto su longitud antes de moverlo a una tupla; len cuenta bytes.
//! - En ex04 uso préstamos: con & leo y con &mut modifico el valor original.
//!   Puedo tener varios lectores o un préstamo mutable exclusivo.
//! - En ex05 agrupo datos con struct y agrego métodos con impl. Uso &self para
//!   consultar y &mut self para cambiar el saldo.
//! - En ex06 distingo variantes con enum y match. Al cubrir todos los casos,
//!   el compilador me avisa si agrego una variante y olvido manejarla.
//! - En ex07 represento fallos esperados con Result. Con ? devuelvo el error
//!   anticipadamente o continúo con el valor de Ok.
//! - En ex08 combino estos conceptos. Valido antes de modificar el saldo y
//!   registro solamente las operaciones exitosas en un Vec<String>.
//!
//! ## Mi vínculo con Solana
//!
//! Practico lógica local: este crate todavía no despliega un programa ni mueve
//! SOL reales. Las referencias de Rust controlan los préstamos de memoria;
//! los permisos de escritura y las firmas de una cuenta requieren validaciones
//! propias en un programa de Solana. Estos ejemplos tampoco manejan desbordes
//! aritméticos: antes de usarlos con fondos reales necesito operaciones checked
//! y errores adecuados.
//!
//! ## Mi verificación
//!
//! Desde Ubuntu/WSL, en la carpeta del repositorio, ejecuto:
//! ```text
//! cargo test --test ex01
//! cargo test --locked
//! sha256sum -c tests.sha256
//! ```
//! Conservo los tests y el workflow originales. Para revisar el formato de mi
//! código sin modificar los tests, ejecuto rustfmt --check --edition 2021 src/*.rs.
//!
//! ## Mi entrega
//!
//! Después de revisar y entender cada solución, ejecuto desde el repositorio:
//! ```text
//! git add src
//! git commit -m "Resuelvo los ocho ejercicios de Rust"
//! git push -u origin solutions/julia-gastellu
//! ```
//! Abro el PR hacia dev3pack/Solana-Rust-Bootcamp-ES, con base main y mi rama
//! solutions/julia-gastellu. Uso el título «Soluciones — Julia Gastellu».
//! En la descripción aclaro: «Usé IA para generar una propuesta de soluciones
//! y como apoyo para estudiar los conceptos». Agrego mi propia explicación
//! después de practicar. No hago merge: el instructor revisa la entrega.

pub mod ex01;
pub mod ex02;
pub mod ex03;
pub mod ex04;
pub mod ex05;
pub mod ex06;
pub mod ex07;
pub mod ex08;
