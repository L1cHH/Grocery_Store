use iced::{Background, Border, Color, Theme, Vector};
use iced::border::Radius;

use iced::widget::{button, container, text, text_input};
use iced::widget::text_input::Appearance;


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

pub struct UserInputStyle;

impl text_input::StyleSheet for UserInputStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(187, 104, 147, 0.5),
                radius: Radius::from(20),
                width: 3.0
            },
            background: Background::Color(Color::default()),
            icon_color: Color::from_rgba8(187, 104, 147, 0.8)
        }
    }

    fn disabled(&self, style: &Self::Style) -> Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::BLACK,
                radius: Radius::from(20),
                ..Default::default()
            },
            background: Background::Color(Color::default()),
            icon_color: Color::default()
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        Color::default()
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(214, 58, 138, 0.8),
                radius: Radius::from(20),
                width: 2.0
            },
            background: Background::Color(Color::default()),
            icon_color: Color::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> Appearance {
        text_input::Appearance {
            border: Border {
                color: Color::from_rgba8(187, 104, 147, 0.9),
                radius: Radius::from(20),
                width: 3.0
            },
            background: Background::Color(Color::default()),
            icon_color: Color::default()
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        Color::default()
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        Color::from_rgba8(187, 104, 147, 0.9)
    }
}

pub struct CategoryContainerStyle;

impl container::StyleSheet for CategoryContainerStyle {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border : Border {
                color: Color::BLACK,
                width: 3.0,
                radius: Radius::from(20)
            },
            ..Default::default()
        }
    }
}

pub struct CategoryButtonStyle;

impl button::StyleSheet for CategoryButtonStyle {
    type Style = Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            ..Default::default()
        }
    }
}