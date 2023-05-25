use crate::{Gas, Temperatura, Volume};

pub struct Atmosfera;

pub struct Compartimento<Gs> 
where
    Gs: Gas,
{
    temperatura: Temperatura,
    volume: Volume,
}

