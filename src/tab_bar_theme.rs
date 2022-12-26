use iced_aw::tabs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 5] = [
        Theme::Light,
        Theme::Dark,
    ];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

impl From<Theme> for String {
    fn from(theme: Theme) -> Self {
        String::from(match theme {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        })
    }
}

impl From<Theme> for Box<dyn tabs::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::TabBar.into(),
        }
    }
}



mod purple {
    use iced::{Color, Background};
    use iced_aw::tabs::{self};

    pub struct TabBar;

    impl tabs::StyleSheet for TabBar {
        fn active(&self, is_selected: bool) -> tabs::Style {
            let text_color = if is_selected {
                [0.7, 0.0, 1.0].into()
            } else {
                Color::BLACK
            };

            Style {
                background: None,
                border_color: None,
                border_width: 0.0,
                tab_label_background: Background::Color(Color::WHITE),
                tab_label_border_color: Color::TRANSPARENT,
                tab_label_border_width: 0.0,
                icon_color: text_color,
                text_color,
            }
        }

        fn hovered(&self, is_selected: bool) -> tabs::Style {
            let text_color = [0.7, 0.0, 1.0].into();

            Style {
                icon_color: text_color,
                text_color,
                ..self.active(is_selected)
            }
        }
    }
}