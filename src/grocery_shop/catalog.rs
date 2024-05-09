use iced::{Element, Renderer};
use iced::widget::{container, text};
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Vegetables,
    Fruits,
    Sweets,
    Meat,
    Cheeses,
    Drinks,
    Baking
}

impl Category {
    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {
        let name_of_category = text(format!("{self}"));

        container()

    }
}