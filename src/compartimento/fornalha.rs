
use crate::item::{self, Tipo};

use crate::utilidades::temperatura;

use crate::erro;

pub struct Fornalha {
    calor: temperatura::Calor<temperatura::Celsius>,
    massa: 
}

impl Quardador for Fornalha {
    fn quardar<Im>(&mut self, im: Im) -> Result<(), (Im, erro::item)>
    where
        Im: item::Item,
    {
        if Im::Tipo != Tipo::Combustivel {
            Err((im, erro::item::Nec));
        }

        
    }
}
