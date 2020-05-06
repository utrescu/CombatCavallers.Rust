pub mod lluitador;

use rand::Rng;

trait IResultat
where
    Self: Sized,
{
    fn get_nom<'a>(&'a self) -> &'a str;
    fn get_punts(&self) -> i32;
    fn es_ko(&self) -> bool;
}

trait ICombatent
where
    Self: Sized,
{
    fn treu_vida(&mut self) -> i32;
    fn pica(&self) -> lluitador::LlocOnPicar;
    fn protegeix(&self) -> Vec<lluitador::LlocOnPicar>;
}

#[derive(Debug)]
pub struct Resultat {
    lluitador: Box<dyn lluitador::ILluitador>,
    vida: i32,
}

impl Resultat {
    pub fn new(ll: Box<dyn lluitador::ILluitador>, v: i32) -> Resultat {
        Resultat {
            lluitador: ll,
            vida: v,
        }
    }
}

impl IResultat for Resultat {
    fn get_nom<'a>(&'a self) -> &'a str {
        &self.lluitador.get_nom_lluitador()
    }

    fn get_punts(&self) -> i32 {
        self.vida
    }

    fn es_ko(&self) -> bool {
        self.vida == 0
    }
}

impl ICombatent for Resultat {
    fn pica(&self) -> lluitador::LlocOnPicar {
        self.lluitador.pica()
    }

    fn protegeix(&self) -> Vec<lluitador::LlocOnPicar> {
        self.lluitador.protegeix()
    }

    fn treu_vida(&mut self) -> i32 {
        self.vida = self.vida - 1;
        self.vida
    }
}

// ---- Ring

pub struct Ring {
    resultat: Vec<Resultat>,
}

impl Ring {
    pub fn new(
        lluitador1: Box<dyn lluitador::ILluitador>,
        lluitador2: Box<dyn lluitador::ILluitador>,
    ) -> Ring {
        Ring {
            resultat: vec![Resultat::new(lluitador1, 20), Resultat::new(lluitador2, 20)],
        }
    }

    pub fn lluiteu(mut self) -> Vec<Resultat> {
        print!(
            "Combat entre {} vs {}\n",
            self.resultat[0].get_nom(),
            self.resultat[1].get_nom()
        );
        print!("--------------------\n");

        let mut rng = rand::thread_rng();
        let mut qui_pica = rng.gen_range(0, 2);
        print!(
            "Es fa el sorteig i comença ... {}({})\n",
            self.resultat[qui_pica].get_nom(),
            self.resultat[qui_pica].get_punts()
        );

        while !self.resultat[0].es_ko() && !self.resultat[1].es_ko() {
            let qui_rep = (qui_pica + 1) % 2;

            let ataca_a = self.resultat[qui_pica].pica();
            let protegeix_a = self.resultat[qui_rep].protegeix();

            if protegeix_a.iter().any(|i| *i == ataca_a) {
                print!(
                    "{}({}) atura el cop dirigit a {:?} de {}({})\n",
                    self.resultat[qui_rep].get_nom(),
                    self.resultat[qui_rep].get_punts(),
                    ataca_a,
                    self.resultat[qui_pica].get_nom(),
                    self.resultat[qui_pica].get_punts()
                );
            } else {
                self.resultat[qui_rep].treu_vida();
                print!(
                    "{}({}) rep un cop a {:?} de {}({})\n",
                    self.resultat[qui_rep].get_nom(),
                    self.resultat[qui_rep].get_punts(),
                    ataca_a,
                    self.resultat[qui_pica].get_nom(),
                    self.resultat[qui_pica].get_punts()
                );
            }
            qui_pica = qui_rep;
        }

        let guanyador = if self.resultat[0].es_ko() {
            &self.resultat[1]
        } else {
            &self.resultat[0]
        };
        let perdedor = if self.resultat[0].es_ko() {
            &self.resultat[0]
        } else {
            &self.resultat[1]
        };

        let missatge = if guanyador.get_punts() - perdedor.get_punts() > 5 {
            "Quina pallissa!!"
        } else {
            ""
        };

        print!("{} cau a terra!\n", perdedor.get_nom());
        print!("\n");
        print!("VICTORIA DE {}. {}\n", guanyador.get_nom(), missatge);

        self.resultat
    }
}
