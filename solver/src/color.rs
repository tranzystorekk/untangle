use strum_macros::{Display, EnumString};

#[derive(Copy, Clone, Display, Debug, EnumString, PartialEq)]
pub enum Color {
    #[strum(serialize = "y", serialize = "Y", to_string = "Yellow")]
    Yellow,

    #[strum(serialize = "r", serialize = "R", to_string = "Red")]
    Red,

    #[strum(serialize = "p", serialize = "P", to_string = "Purple")]
    Purple,

    #[strum(serialize = "g", serialize = "G", to_string = "Green")]
    Green,

    #[strum(serialize = "o", serialize = "O", to_string = "Orange")]
    Orange,

    #[strum(serialize = "*", to_string = "Blank")]
    Blank,
}

impl Color {
    pub fn non_blank(&self) -> bool {
        !matches!(self, Color::Blank)
    }
}
