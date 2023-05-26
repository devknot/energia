//mod usina;
mod utilidades;

use utilidades::{Calor, Celsius, Kelvin};

type Tm = Celsius;

type Tp = Kelvin;

fn main() {
    let a: Calor<Tm> = Calor::gerar(2.0);
    let b: Calor<Tm> = Calor::gerar(3.0);

    let c: Calor<Tm> = a + b;

    let d: Calor<Tp> = Calor::from(c);

    println!("{}", d);
}


