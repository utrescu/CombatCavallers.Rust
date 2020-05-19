mod combat;

extern crate rand;
extern crate strum;
extern crate strum_macros;

use combat::lluitador::ILluitador as lluitador;
use combat::lluitador::LlocOnPicar;
use combat::IRing;

fn main() {
    let lluitador1 = Box::new(LluitadorRandom::new("TrencaCaps".to_string()));
    let lluitador2 = Box::new(LluitadorRandom::new("TallaFerro".to_string()));

    let ring = combat::Ring::new(lluitador1, lluitador2);

    let _ = ring.lluiteu();
}

pub struct LluitadorRandom {
    nom: String,
}

impl LluitadorRandom {
    pub fn new(n: String) -> LluitadorRandom {
        LluitadorRandom { nom: n }
    }
}

impl lluitador for LluitadorRandom {
    fn get_nom_lluitador(&self) -> &str {
        &self.nom
    }

    fn pica(&self) -> LlocOnPicar {
        let pica: LlocOnPicar = rand::random();
        pica
    }

    fn protegeix(&self) -> Vec<LlocOnPicar> {
        let mut llocs = combat::lluitador::get_all_llocs_on_picar();
        let no_protegeix: LlocOnPicar = rand::random();

        let index = llocs.iter().position(|x| *x == no_protegeix).unwrap();
        llocs.remove(index);

        llocs
    }
}
