use super::combat::lluitador as lluita;

pub struct LluitadorRandom {
    nom: String,
}

impl LluitadorRandom {
    pub fn new(n: String) -> LluitadorRandom {
        LluitadorRandom { nom: n }
    }
}

impl lluita::ILluitador for LluitadorRandom {
    fn get_nom_lluitador(&self) -> &str {
        &self.nom
    }

    fn pica(&self) -> lluita::LlocOnPicar {
        let pica: lluita::LlocOnPicar = rand::random();
        pica
    }

    fn protegeix(&self) -> Vec<lluita::LlocOnPicar> {
        let mut llocs = lluita::get_all_llocs_on_picar();
        let no_protegeix: lluita::LlocOnPicar = rand::random();

        let index = llocs.iter().position(|x| *x == no_protegeix).unwrap();
        llocs.remove(index);

        llocs
    }
}
