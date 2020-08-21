use strum_macros::{Display, EnumString};

#[derive(Display, Debug, EnumString, PartialEq)]
pub enum Color {
    #[strum(serialize="y")]
    Yellow,

    #[strum(serialize="r")]
    Red,

    #[strum(serialize="p")]
    Purple,

    #[strum(serialize="g")]
    Green,

    #[strum(serialize="o")]
    Orange,

    #[strum(serialize="0")]
    Blank,
}