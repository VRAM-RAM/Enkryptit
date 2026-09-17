use comfy_table::{
    Attribute, Color,
};

pub enum EnkryptitStyle {
    Label,
    Value,
    Green,
    Orange,
    Accent,
    Warning,
    Error,
}

impl EnkryptitStyle {
    pub fn color(&self) -> Color {
        match self {
            &Self::Accent => Color::Cyan,
            &Self::Green => Color::Green,
            &Self::Label => Color::Grey,
            &Self::Orange => Color::AnsiValue(214),
            &Self::Value => Color::White,
            &Self::Warning => Color::Red,
            &Self::Error => Color::DarkRed,

        }
    }

    pub fn attribute(&self) -> Attribute {
        match self {
            &Self::Accent => Attribute::Bold,
            &Self::Label => Attribute::Bold,
            &Self::Warning => Attribute::Bold,
            &Self::Error => Attribute::Italic,
            _ => Attribute::NoUnderline
        }
    }
}