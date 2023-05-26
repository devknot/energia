pub mod temperatura;
pub mod forca;
pub use temperatura::*;

pub type Comum = f32;

pub trait Temperatura {
    const FUSAO: Comum; // fusão da água como referência
    const EBULICAO: Comum; // ebulição da água como referência
    const ZERO: Comum; // zero absoluto
}

pub trait Potencia {}

