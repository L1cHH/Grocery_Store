use iced::{Background, Border, Color, Theme, Vector};
use iced::border::Radius;

use iced::widget::{button, container, text};



pub struct UserButtonStyle;

impl button::StyleSheet for UserButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector::new(0.0, 2.0),
            background: Some(Background::Color(Color::from_rgba8(187, 104, 147, 0.8))),
            border: Border {
                color: Color::BLACK,
                width: 2.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}

pub struct UserContainerStyle;

impl container::StyleSheet for UserContainerStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border: Border {
                color: Color::BLACK,
                width: 2.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}

pub struct UserTextStyle;

impl text::StyleSheet for UserTextStyle {
    type Style = Theme;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(Color::from_linear_rgba(187.0, 104.0, 147.0, 0.8))
        }
    }
}