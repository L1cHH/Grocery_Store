use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::grocery_shop::catalog::{Item, Category};
mod user;
mod grocery_shop;


fn main() {


    let items = read_json_file();

    let data: HashMap<String, Item> = items.into_iter().map(|item| (item.name.clone(), item)).collect();

    let shop = grocery_shop::GroceryShop::new(data);







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
