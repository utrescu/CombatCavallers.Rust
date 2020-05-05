mod combat;

extern crate rand;
extern crate strum;
extern crate strum_macros;

use combat::lluitador::ILluitador as lluitador;
use combat::lluitador::LlocOnPicar;

fn main() {
    let lluitador1 = LluitadorRandom::new("TrencaCaps".to_string());
    let lluitador2 = LluitadorRandom::new("TallaFerro".to_string());

    let ring = combat::Ring::new(&lluitador1, &lluitador2);

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
        return &self.nom;
    }

    fn pica(&self) -> LlocOnPicar {
        let pica: LlocOnPicar = rand::random();
        return pica;
    }

    fn protegeix(&self) -> Vec<LlocOnPicar> {
        let mut llocs = combat::lluitador::get_all_llocs_on_picar();
        let no_protegeix: LlocOnPicar = rand::random();

        let index = llocs.iter().position(|x| *x == no_protegeix).unwrap();
        llocs.remove(index);

        return llocs;
    }
}