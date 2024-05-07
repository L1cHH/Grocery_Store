use std::collections::HashMap;

use crate::grocery_shop::catalog::{Category, Item};
use crate::pages::Page;

pub mod catalog;


pub struct GroceryShop {
    catalog: HashMap<String, Item>,
    page: Page
}
impl GroceryShop {
    pub fn sell_item(&mut self, item_name: String, amount: usize) {
        let particular_item = self.catalog.get_mut(&item_name).unwrap();
        particular_item.change_amount(amount);
    }

    pub fn new(catalog: HashMap<String, Item>, page: Page) -> Self{
        Self {
            catalog,
            page
        }
    }
    pub fn get_curr_page(&self) -> &Page {
        &self.page
    }

    pub fn change_curr_page(&mut self, page: Page) {
        self.page = page
    }
}