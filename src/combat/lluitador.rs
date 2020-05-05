use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(EnumIter, PartialEq, Eq)]
pub enum LlocOnPicar {
    Cap,
    CostatEsquerra,
    CostatDret,
    Panxa,
}

impl fmt::Debug for LlocOnPicar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LlocOnPicar::Cap => write!(f, "Cap"),
            LlocOnPicar::CostatEsquerra => write!(f, "CostatEsquerra"),
            LlocOnPicar::CostatDret => write!(f, "CostatDret"),
            LlocOnPicar::Panxa => write!(f, "Panxa"),
        }
    }
}

pub fn get_all_llocs_on_picar() -> Vec<LlocOnPicar> {
    let mut llocs = Vec::<LlocOnPicar>::new();

    for lloc in LlocOnPicar::iter() {
        llocs.push(lloc);
    }
    return llocs;
}

impl Distribution<LlocOnPicar> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> LlocOnPicar {
        match rng.gen_range(0, 3) {
            0 => LlocOnPicar::Cap,
            1 => LlocOnPicar::CostatEsquerra,
            2 => LlocOnPicar::CostatDret,
            3 => LlocOnPicar::Panxa,
            _ => LlocOnPicar::Cap,
        }
    }
}

pub trait ILluitador {
    fn get_nom_lluitador(&self) -> &str;
    fn protegeix(&self) -> Vec<LlocOnPicar>;
    fn pica(&self) -> LlocOnPicar;
}