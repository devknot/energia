
type Temperatura = u64;

type Volume = [u64; 3];

type Calor = u64;

type CondutividadeTermica = u64;

pub trait Gas {
    fn calor_volume() -> Calor;
    fn condutividade_termica() -> CondutividadeTermica;
}

/*
pub trait Compartimento {
    fn despressurizar(&mut self);
    fn pressurizar(&mut self, gas: crate::Gas);
}
*/

