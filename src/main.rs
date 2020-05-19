mod combat;
mod lluitadors;

extern crate rand;
extern crate strum;
extern crate strum_macros;

use combat::IRing;

fn main() {
    let lluitador1 = Box::new(lluitadors::LluitadorRandom::new("TrencaCaps".to_string()));
    let lluitador2 = Box::new(lluitadors::LluitadorRandom::new("TallaFerro".to_string()));

    let ring = combat::Ring::new(lluitador1, lluitador2);

    let _ = ring.lluiteu();
}
