use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    Vegetables,
    Fruits,
    Sweets,
    Meat,
    Cheeses,
    Drinks,
    Baking
}