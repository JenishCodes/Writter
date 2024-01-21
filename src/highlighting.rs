use termion::color;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match,
    String,
    Character,
    PrimaryKeywords,
    SecondaryKeywords,
}

impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
            Self::Number => color::Rgb(220, 163, 163),
            Self::Match => color::Rgb(38, 210, 139),
            Self::String => color::Rgb(211, 54, 130),
            Self::Character => color::Rgb(108, 113, 196),
            Self::PrimaryKeywords => color::Rgb(181, 137, 0),
            Self::SecondaryKeywords => color::Rgb(42, 161, 152),
            _ => color::Rgb(255, 255, 255),
        }
    }
}
