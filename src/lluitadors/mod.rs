use super::combat::lluitador as lluita;

/// Implementació de jugador que fa les coses aleatòriament
pub struct LluitadorRandom {
    nom: String,
}

impl LluitadorRandom {
    /// Crea una instància de lluitadorRandom a partir del
    /// nom d'un lluitador
    ///
    /// # Arguments
    ///
    /// * n: nom de lluitador
    pub fn new(n: String) -> LluitadorRandom {
        LluitadorRandom { nom: n }
    }
}

/// LluitadorRandom implementa un lluitador que dóna cops aleatoris i es
/// protegeix de forma aleatòria
impl lluita::ILluitador for LluitadorRandom {
    /// Retorna el nom del lluitador
    fn get_nom_lluitador(&self) -> &str {
        &self.nom
    }

    /// Retorna el lloc on vol Picar el lluitador
    fn pica(&self) -> lluita::LlocOnPicar {
        let pica: lluita::LlocOnPicar = rand::random();
        pica
    }

    /// Retorna els tres punts que protegeix el lluitador
    fn protegeix(&self) -> Vec<lluita::LlocOnPicar> {
        let mut llocs = lluita::get_all_llocs_on_picar();
        let no_protegeix: lluita::LlocOnPicar = rand::random();

        let index = llocs.iter().position(|x| *x == no_protegeix).unwrap();
        llocs.remove(index);

        llocs
    }
}
