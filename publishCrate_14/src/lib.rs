pub use self::kinds::PrimaryColor;
pub use self::utils::mix;


pub mod kinds {

    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }

}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }

}