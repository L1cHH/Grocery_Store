use std::env;
use std::path::PathBuf;
use std::str::Bytes;
use iced::{Element, Renderer, Theme};
use iced::Alignment::Center;
use iced::theme::Text::Color;
use iced::widget::{container, Svg, text, column, button, Button, Text, Space};
use iced::widget::svg::Handle;
use serde::{Deserialize, Serialize};
use crate::grocery_shop::GroceryShop;
use crate::Message;
use crate::styles::{CategoryButtonStyle, CategoryContainerStyle, UserButtonStyle, UserTextStyle};

#[derive(Deserialize, Serialize, Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub category: Category,
    pub price: usize,
    pub weight: usize,
    amount: usize,
}
impl Item {
    pub fn new(id: usize, name: String, category: Category, price: usize, weight: usize, amount: usize) -> Self {
        Self {
            id,
            name,
            category,
            price,
            weight,
            amount
        }
    }
    pub fn change_amount(&mut self, amount: usize) {
        self.amount -= amount;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct  Category {
    category_name: String,
    svg: String
}

impl Category {
    pub fn view(&self) -> Element<'_, <GroceryShop as iced::Application>::Message, <GroceryShop as iced::Application>::Theme, Renderer> {
        let category_name = &self.category_name;
        let category_text = text(category_name).size(15).style(iced::Color::WHITE);

        let mut path:PathBuf = PathBuf::from("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\items".to_string());
        path.push(&self.svg);

        let svg_handler = Handle::from_path(path);
        let svg = Svg::new(svg_handler).width(50).height(50);

        let svg_button_handler = Handle::from_path("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\icons\\arrow.svg".to_string());
        let btn_svg = Svg::new(svg_button_handler).width(100).height(100);

        let btn = Button::new(btn_svg).style(iced::theme::Button::Custom(Box::new(CategoryButtonStyle))).on_press(Message::ToItemsPage);

        container(
            column![
                category_text,
                svg,
                btn
            ].spacing(10).align_items(Center)
        )
            .width(150)
            .height(150)
            .center_y()
            .center_x()
            .padding(10)
            .style(iced::theme::Container::Custom(Box::new(CategoryContainerStyle)))
            .into()

    }
}