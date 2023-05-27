use std::marker::PhantomData;

pub struct Combustivel<Cm>
where
    Cm: Consumivel,
{
    peso: Peso,
    phantom: PhantomData<Cm>,
}

impl <Cm> Combustivel<Cm>
where
    Cm: Consumivel
{
    pub fn gerar(peso: Peso) -> Self {
        Self {
            peso,
            phantom: PhantomData,
        }
    }
}




