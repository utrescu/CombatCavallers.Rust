//! combat_cavallers
//! Defineix implementacions i interfícies pel combat entre cavallers
//!
//! Per crear un nou lluitador n'hi ha prou amb implementar la interfície
//! combat::lluitador::ILluitador
mod combat;
mod lluitadors;

extern crate rand;
extern crate strum;
extern crate strum_macros;

use combat::IRing;

/// exemple d'implementació
fn main() {
    let lluitador1 = Box::new(lluitadors::LluitadorRandom::new("TrencaCaps".to_string()));
    let lluitador2 = Box::new(lluitadors::LluitadorRandom::new("TallaFerro".to_string()));

    let ring = combat::Ring::new(lluitador1, lluitador2);

    let _ = ring.lluiteu();
}
