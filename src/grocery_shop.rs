use std::collections::HashMap;

use crate::grocery_shop::catalog::{Category, Item};
pub mod catalog;

pub struct GroceryShop {
    catalog: HashMap<String, Item>,
}
impl GroceryShop {
    pub fn sell_item(&mut self, item_name: String, amount: usize) {
        let particular_item = self.catalog.get_mut(&item_name).unwrap();
        particular_item.change_amount(amount);
    }

    pub fn new(catalog: HashMap<String, Item>) -> Self {
        Self {
            catalog
        }
    }
}