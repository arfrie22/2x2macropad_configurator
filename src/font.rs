use iced::Font;

pub const ROBOTO: Font = iced::Font::External {
    name: "Roboto",
    bytes: include_bytes!("../assets/fonts/Roboto-Regular.ttf"),
};

pub const ROBOTO_BOLD: Font = iced::Font::External {
    name: "Roboto Bold",
    bytes: include_bytes!("../assets/fonts/Roboto-Bold.ttf"),
};

pub const ICON_FONT: Font = iced::Font::External {
    name: "Remix Icon", // Can not be icons otherwise it conflicts with the default icon font
    bytes: include_bytes!("../assets/fonts/remixicon.ttf"),
};

#[derive(Debug, Clone)]
pub enum Icon {
    Keyboard,
    Light,
    Gear,
    Sun,
    Moon,
    Add,
    Close,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::Keyboard => '\u{ee72}',
            Icon::Light => '\u{eea6}',
            Icon::Gear => '\u{f0e5}',
            Icon::Sun => '\u{f1bc}',
            Icon::Moon => '\u{ef72}',
            Icon::Add => '\u{ea12}',
            Icon::Close => '\u{eb98}',
        }
    }
}

impl From<Icon> for String {
    fn from(icon: Icon) -> Self {
        char::from(icon).to_string()
    }
}