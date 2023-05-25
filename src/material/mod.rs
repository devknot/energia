
type Joule = u64;

type Grama = u64;

pub trait Material<Produto> {
    fn consumir(self, gramas: Grama) -> (Joule, Produto);
}

