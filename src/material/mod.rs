mod objeto;

pub trait Consumivel {
    fn consumir(self) -> (Calor, Gas);
}

pub trait Combustivel {
    fn gerar(self);
}

