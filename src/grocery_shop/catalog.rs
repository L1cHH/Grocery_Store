use std::fmt::format;
use std::path::PathBuf;
use iced::{Element, Renderer, Theme};
use iced::Alignment::Center;
use iced::widget::{container, Svg, text, column, button, Button, Text, Space, Image, svg, image, row};
use serde::{Deserialize, Serialize};
use crate::cart::CartMessage;
use crate::grocery_shop::GroceryShop;
use crate::Message;
use crate::styles::{AddButtonStyle, CategoryButtonStyle, CategoryContainerStyle, UserButtonStyle, UserTextStyle};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub category: Category,
    pub price: usize,
    pub weight: usize,
    pub amount: usize,
    pub svg: String
}
impl Item {
    pub fn new(id: usize, name: String, category: Category, price: usize, weight: usize, amount: usize, svg: String) -> Self {
        Self {
            id,
            name,
            category,
            price,
            weight,
            amount,
            svg
        }
    }
    pub fn change_amount(&mut self, amount: usize) {
        self.amount -= amount;
    }

    pub fn view(&self) -> Element<'_, <GroceryShop as iced::Application>::Message, <GroceryShop as iced::Application>::Theme, Renderer> {
        let mut path_to_items:PathBuf = PathBuf::from("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\items".to_string());
        path_to_items.push(&self.svg);

        let item_svg = Image::<image::Handle>::new(path_to_items).width(100).height(100);

        let name_of_item = &self.name;
        let amount_of_item = &self.amount;
        let price_of_item = &self.price;
        let weight_of_item = &self.weight;

        let item_label = text(name_of_item).size(10);
        let price_label = text(format!("Цена: {price_of_item} р.")).size(10);
        let weight_label = text(format!("Вес: {weight_of_item}")).size(10);
        let amount_label = text(format!("Кол-во: {amount_of_item}")).size(10);

        let info_col = column![item_label, price_label, weight_label, amount_label].spacing(10);

        let svg_btn_handler = svg::Handle::from_path("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\icons\\new-add.svg".to_string());

        let svg_btn = Svg::new(svg_btn_handler).width(50).height(50);

        let btn_add = button(svg_btn).style(iced::theme::Button::Custom(Box::new(AddButtonStyle))).on_press(Message::AddToCart(self.clone()));

        container(column![
            item_svg,
            row![info_col, btn_add].spacing(10).align_items(Center)
        ].spacing(10).align_items(Center)).padding(10).style(iced::theme::Container::Custom(Box::new(CategoryContainerStyle))).into()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct  Category {
    pub category_name: String,
    pub svg: String
}

impl Category {
    pub fn view(&self) -> Element<'_, <GroceryShop as iced::Application>::Message, <GroceryShop as iced::Application>::Theme, Renderer> {
        let category_name = &self.category_name;
        let category_text = text(category_name).size(15).style(iced::Color::WHITE);

        let mut path:PathBuf = PathBuf::from("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\categories".to_string());
        path.push(&self.svg);

        let svg_handler = svg::Handle::from_path(path);
        let svg = Svg::new(svg_handler).width(50).height(50);

        let svg_button_handler = svg::Handle::from_path("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\icons\\arrow.svg".to_string());
        let btn_svg = Svg::new(svg_button_handler).width(100).height(100);

        let btn = Button::new(btn_svg).style(iced::theme::Button::Custom(Box::new(CategoryButtonStyle))).on_press(Message::ToItemsPage(self.clone()));

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