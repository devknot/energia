//mod usina;
mod utilidades;

use utilidades::Calor;

pub fn main() {
    let mut a = Calor::gerar(2);
    a += Calor::gerar(3);

    println!("{}", a);
}
