use strum_macros::{Display, EnumString};

#[derive(Copy, Clone, Display, Debug, EnumString, PartialEq)]
pub enum Color {
    #[strum(serialize = "y", serialize = "Y")]
    Yellow,

    #[strum(serialize = "r", serialize = "R")]
    Red,

    #[strum(serialize = "p", serialize = "P")]
    Purple,

    #[strum(serialize = "g", serialize = "G")]
    Green,

    #[strum(serialize = "o", serialize = "O")]
    Orange,

    #[strum(serialize = "0")]
    Blank,
}
