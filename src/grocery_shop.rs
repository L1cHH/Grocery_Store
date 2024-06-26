use std::collections::HashMap;

use crate::grocery_shop::catalog::{Category, Item};
use crate::pages::Page;
use crate::user::Buyer;

pub mod catalog;


pub struct GroceryShop {
    catalog: HashMap<Category, Vec<Item>>,
    pub page: Page,
    buyer: Option<Buyer>
}
impl GroceryShop {
    pub fn sell_item(&mut self, item_name: String, amount: usize) {
        //let particular_item = self.catalog.get_mut(&item_name).unwrap();
        //particular_item.change_amount(amount);
    }

    pub fn new(catalog: HashMap<Category, Vec<Item>>, page: Page) -> Self{
        Self {
            catalog,
            page,
            buyer: None
        }
    }
    pub fn get_curr_page(&self) -> &Page {
        &self.page
    }

    pub fn change_curr_page(&mut self, page: Page) {
        self.page = page
    }

    pub fn add_buyer(&mut self, buyer: Buyer) {
        self.buyer = Some(buyer)
    }

    pub fn get_curr_catalog(&self) -> &HashMap<Category, Vec<Item>> {
        &self.catalog
    }

    pub fn get_items_by_category(&self, category: &Category) -> Option<&Vec<Item>> {
        self.catalog.get(category)
    }

    pub fn get_items_by_category_mut(&mut self, category: Category) -> Option<&mut Vec<Item>> {
        self.catalog.get_mut(&category)
    }

    pub fn get_user(&self) -> Option<&Buyer> {
        self.buyer.as_ref()
    }

    pub fn get_user_mut(&mut self) -> Option<&mut Buyer> {
        self.buyer.as_mut()
    }
}