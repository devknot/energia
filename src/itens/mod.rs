
type Id = u16;

#[derive(Debug, PartialEq)]
enum Tipo {
    Combustivel,
}

pub trait Item {
    const ID: Id;
    const TIPO: Tipo;
}

