pub mod temperatura;
pub use temperatura::*;

pub type Comum = i64;

pub trait Temperatura {
    const FUSAO: Comum; // fusão da água como referência
    const EBULICAO: Comum; // ebulição da água como referência
    const ZERO: Comum; // zero absoluto
}

