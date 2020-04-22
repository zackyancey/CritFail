use iced::{button, Background, Color, Vector};

pub mod text {
    use iced::Text;
    pub fn header(text: &str) -> Text {
        Text::new(text).size(30)
    }

    pub fn paragraph(text: &str) -> Text {
        Text::new(text)
    }
}

pub enum Button {
    Primary,
    Secondary,
}

impl button::StyleSheet for Button {
    fn active(&self) -> button::Style {
        button::Style {
            background: match self {
                Button::Primary => Some(Background::Color(Color::from_rgb(0.11, 0.42, 0.87))),
                Button::Secondary => Some(Background::Color(Color::from_rgb8(0xEE, 0xEE, 0xEE))),
            },
            border_radius: 15,
            shadow_offset: match self {
                Button::Primary => Vector::new(1.0, 1.0),
                Button::Secondary => Vector::new(0.0, 0.0),
            },

            text_color: match self {
                Button::Primary => Color::from_rgb8(0xE8, 0xE8, 0xEE),
                Button::Secondary => Color::from_rgb8(0x1A, 0x2A, 0x3A),
            },

            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            text_color: Color::WHITE,
            shadow_offset: Vector::new(1.0, 2.0),
            background: match self {
                Button::Primary => active.background,
                Button::Secondary => Some(Background::Color(Color::from_rgb8(0xAA, 0xAA, 0xAA))),
            },
            ..self.active()
        }
    }
}
