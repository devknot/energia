mod estruturas;

use estruturas::eletrica;

trait A {}

struct B;

impl A for B {}

trait Dig<As>
where
    As: A,
{
    fn p(&self, a: As);
}

struct Ficticio;

impl <As: A> Dig<As> for Ficticio {
    fn p(&self, _: As) {
        println!("tudo certo.")
    }
}

fn main() {
    let fumaceiro = eletrica::Fumaceiro::gerar();

    let f = Ficticio;
    f.p(B);
}


/*
mod utilidades;

mod gases;

use utilidades::{
    temperatura::{Calor, Celsius, Kelvin}, forca};

type Tm = Celsius;

type Tp = Kelvin;

fn main() {
    let a: Calor<Tm> = Calor::gerar(2.0);
    let b: Calor<Tm> = Calor::gerar(3.0);

    let c: Calor<Tm> = a + b;

    let d: Calor<Tp> = Calor::from(c);

    println!("{}", d);

    let forca = forca::Forca::gerar(2.0);

    println!("{}", forca);
}
*/

