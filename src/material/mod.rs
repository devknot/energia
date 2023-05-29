mod objeto;

pub trait Consumivel {
    fn consumir(self) -> (Calor, Gas);
}

