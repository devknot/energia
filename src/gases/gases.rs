use crate::{Gas, Temperatura, Volume, Calor, CondutividadeTermica};

pub struct Carbono;

impl Gas for Carbono{
    fn calor_volume() -> Calor {
        6489500 //kcal/kg*C
    }
    fn condutividade_termica() -> CondutividadeTermica {
        166
    }
}


