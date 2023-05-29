pub mod armazenador;

use crate::erro::item;
use crate::item;

pub trait Quardador {
    fn quardar<Im: item::Item>(&mut self, im: Im) -> Result<(), (Im, item::erro)>;
}

/*
pub trait Estocador {
    fn estocar<Gs: Gas>(&mut self, gs: Gs) -> Result<(), (Gs, gases::erro)>;
}
*/