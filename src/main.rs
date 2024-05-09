use std::collections::HashMap;
use std::{env, fs};
use std::env::current_dir;
use std::fs::File;
use std::io::{Read, Write};
use iced::{Application, Color, Command, Element, executor, Font, Length, Padding, Renderer, Sandbox, Settings, Theme};
use iced::Alignment::Center;
use iced::widget::{button, container, text, Svg, column, Column, Space, Text, text_input, TextInput, Container, Row};
use iced::widget::svg::Handle;
use crate::grocery_shop::catalog::{Item, Category};
use crate::grocery_shop::GroceryShop;
use crate::pages::catalog_page_state::CatalogPageState;
use crate::pages::category_page_state::CategoryPageState;
use crate::pages::entry_page_state::EntryPageState;
use crate::pages::Page;
use crate::styles::{UserButtonStyle, UserContainerStyle, UserTextStyle, UserInputStyle};
use crate::user::Buyer;

mod user;
mod grocery_shop;
mod pages;
mod styles;


fn main() -> iced::Result {
    GroceryShop::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    ToCategoryPage,
    BonusCardInputChanged(String),
    CreditCardInputChanged(String),
    CashInputChanged(String),
    ToItemsPage
}


impl Application for GroceryShop {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let shop_catalog = read_json_file();
        (Self::new(shop_catalog, Page::EntryPage(EntryPageState::default())),
         Command::none())
    }

    fn title(&self) -> String {
        "Grocery Shop".to_string()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dracula
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &mut self.page {
            Page::EntryPage(entry_page_state) => {
                match message {
                    Message::BonusCardInputChanged(amount) => {
                        entry_page_state.bonus_card_input = amount;
                        Command::none()
                    }
                    Message::CreditCardInputChanged(amount) => {
                        entry_page_state.credit_card_input = amount;
                        Command::none()
                    }
                    Message::CashInputChanged(amount) => {
                        entry_page_state.cash_input = amount;
                        Command::none()
                    }
                    Message::ToCategoryPage => {
                        let buyer = Buyer::new(
                            entry_page_state.bonus_card_input.clone(),
                            entry_page_state.credit_card_input.clone(),
                            entry_page_state.cash_input.clone(),
                        );

                        self.add_buyer(buyer);
                        self.page = Page::CategoryPage(CategoryPageState::default());
                        Command::none()
                    },
                    _ => {Command::none()}
                }
            }
            Page::CategoryPage(category_page_state) => {
                match message {
                    Message::ToItemsPage => {
                        self.page = Page::CatalogPage(CatalogPageState::default());
                        Command::none()
                    },
                    _ => {Command::none()}
                }
            },
            _ => {todo!()}
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        //ICON DIRECTORY


        let current_page = self.get_curr_page();
        let content = match current_page {
            Page::EntryPage(entry_page_state) => {

                let user_label: Text<'_, Theme, Renderer> = text("Пользователь")
                    .size(35)
                    .style(iced::theme::Text::Color(Color::from_rgba8(187, 104, 147, 0.8))).into();

                let btn = button("Войти")
                    .padding(Padding::from([15, 40]))
                    .style(iced::theme::Button::Custom(Box::new(UserButtonStyle)))
                    .on_press(Message::ToCategoryPage);

                let mut icon_directory = current_dir().unwrap();
                icon_directory.push("src\\icons");

                let mut svg_path = icon_directory;
                svg_path.push("user-svg.svg");

                //let user_svg_handler = Handle::from_memory(include_bytes!("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\icons\\user-svg.svg").as_slice());
                let user_svg_handler = Handle::from_path("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\icons\\user-svg.svg");

                let svg = Svg::new(user_svg_handler).width(150).height(150);

                let inputs_col = column![
                    user_input("Бонусная карта...", &entry_page_state.bonus_card_input)
                        .on_input(Message::BonusCardInputChanged),
                    user_input("Кредитная карта...", &entry_page_state.credit_card_input)
                        .on_input(Message::CreditCardInputChanged),
                    user_input("Наличные...", &entry_page_state.cash_input)
                        .on_input(Message::CashInputChanged),
                ].spacing(10).align_items(Center);

                container(column![
                    user_label,
                    Space::with_height(10),
                    Svg::from(svg),
                    inputs_col,
                    btn,
                ].spacing(15).align_items(Center))
                    .width(400)
                    .height(500)
                    .center_x()
                    .center_y()
                    .style(iced::theme::Container::Custom(Box::new(UserContainerStyle)))
                    .into()
            }
            Page::CategoryPage(category_page_state) => {
                let user_label: Text<'_, Theme, Renderer> = text("Выберите Категорию")
                    .size(35)
                    .style(iced::theme::Text::Color(Color::from_rgba8(187, 104, 147, 0.8))).into();

                let category_vec:Vec<Element<'_, Self::Message, Self::Theme, Renderer>> = {
                    self.get_curr_catalog().keys().map(|category| {
                        category.view()
                    }).collect()
                };

                let category_row = Row::from_vec(category_vec).spacing(10);

                container(column![
                    user_label,
                    Space::with_height(20),
                    container(category_row).padding(20)
                ].spacing(20).align_items(Center))
                    .width(600)
                    .height(600)
                    .center_x()
                    .center_y()

            },
            _ => {todo!()}

        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}








fn read_json_file() -> HashMap<Category, Vec<Item>> {
    let mut curr_dir = env::current_dir().unwrap();
    curr_dir.push("src\\grocery_shop\\catalog.json");

    let mut file = File::open(curr_dir).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Cant read file");
    let items: Vec<Item> = serde_json::from_str(&content).expect("Cant deserialize json catalog");

    let mut items_map:HashMap<Category, Vec<Item>> = HashMap::new();

    for item in items.into_iter() {
        items_map.entry(item.category.clone()).and_modify(|vector| vector.push(item)).or_insert(Vec::new());
    }
    println!("{items_map:?}");

    items_map
}

// fn write_json() {
//     let mut curr_dir = current_dir().unwrap();
//     curr_dir.push("src\\grocery_shop\\categories.json");
//
//     let mut file = fs::File::open(curr_dir).expect("Cant open file");
//     let mut content = String::new();
//
//     file.read_to_string(&mut content).expect("Cant read json");
//
//     let category:Vec<Category> = serde_json::from_str(&content).expect("Cant deserialize json");
//
//     println!("{category:?}")
// }


fn user_input(placeholder: &str, val: &str) -> TextInput<'static, Message>{
    text_input(placeholder, val)
        .width(150)
        .padding(10)
        .size(15)
        .style(iced::theme::TextInput::Custom(Box::new(UserInputStyle)))
}
