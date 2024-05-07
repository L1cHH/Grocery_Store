use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use iced::{Application, Command, Element, Renderer, Sandbox, Settings, Theme};
use crate::grocery_shop::catalog::{Item, Category};
use crate::grocery_shop::GroceryShop;

mod user;
mod grocery_shop;
mod pages;


fn main() -> iced::Result {
    GroceryShop::run(Settings::default())
}

enum Message {
    
}


impl Application for GroceryShop {
    type Executor = ();
    type Message = Message;
    type Flags = ();
    type Theme = Theme::TokyoNight;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        "Grocery Shop".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        todo!()
    }
}








fn read_json_file() -> Vec<Item> {
    let mut curr_dir = env::current_dir().unwrap();
    curr_dir.push("src\\grocery_shop\\catalog.json");

    let mut file = File::open(curr_dir).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).expect("Cant read file");
    let items: Vec<Item> = serde_json::from_str(&content).expect("Cant deserialize json");

    items
}
