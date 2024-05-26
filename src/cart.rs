use std::collections::HashMap;
use std::fmt::format;
use std::path::PathBuf;
use iced::{Element, Length, Renderer};
use iced::Alignment::Center;
use iced::widget::{container, image, Image, Row, text, row, Scrollable, Column, column, svg, Svg, button, Space};
use iced::widget::scrollable::{Direction, Properties};
use crate::grocery_shop::catalog::Item;
use crate::grocery_shop::GroceryShop;
use crate::Message;
use crate::styles::{AddButtonStyle, CategoryContainerStyle, UserContainerStyle};

#[derive(Default)]
pub enum Cart {
    NonEmptyCart(CartWithItems),
    #[default]
    EmptyCart
}

impl Cart {
    pub fn new() -> Self {
        Self::EmptyCart
    }
}
#[derive(Clone)]
struct CartWithItems {
    cart_items: HashMap<Item, usize>,
    final_price: usize,
    items_amount: usize
}

impl CartWithItems {
    fn new() -> Self {
        Self {
            cart_items: HashMap::new(),
            final_price: 0,
            items_amount: 0
        }
    }

    fn add_item_to_cart(&mut self, item: Item) {
        self.final_price += item.price;
        self.cart_items.entry(item).and_modify(|amount| *amount += 1).or_insert(1);
        self.items_amount += 1;
    }
}

#[derive(Clone, Debug)]
pub enum CartMessage {
    AddItem(Item),
    DeleteItem(Item),
}

impl Cart {

    pub fn add_item(&mut self, mut item: Item) {
        item.amount = 1;
        match self {
            Cart::EmptyCart => {
                let mut new_cart = CartWithItems::new();
                new_cart.add_item_to_cart(item);
                *self = Self::NonEmptyCart(new_cart.clone());

            },

            Cart::NonEmptyCart(cart) => {
                cart.add_item_to_cart(item);
            }
        }
    }

    pub fn delete_item(&mut self, mut item: Item) {
        match self {
            Cart::NonEmptyCart(cart) => {
                let mut a = cart.cart_items.get_mut(&item).expect("Cant del item");
                *a -= 1;
                cart.items_amount -= 1;
                cart.final_price -= item.price;
                match a {
                    0 => {
                        cart.cart_items.remove(&item);
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn update(&mut self, message: CartMessage) {
        match message {
            CartMessage::AddItem(mut item) => {
                item.amount = 1;
                self.add_item(item);
            },
            CartMessage::DeleteItem(mut item) => {
                item.amount = 1;
                self.delete_item(item);
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            Cart::EmptyCart => {
                let cart_text = text("Корзина пуста").size(30);
                container(cart_text).into()
            },
            Cart::NonEmptyCart(cart) => {
                let final_price = cart.final_price;
                let items_amount = cart.items_amount;

                let cart_text = text("Ваша корзина").size(30);
                let final_price_text = text(format!("Итоговая цена: {final_price}")).size(20);
                let items_amount_text = text(format!("Кол-во продуктов: {items_amount}")).size(20);

                let items_vec: Vec<(&Item, &usize)> = cart.cart_items.iter().map(|(item, amount)| (item, amount)).collect();

                let items_row = cart_items_view(items_vec);

                let items_scroll = Scrollable::new(items_row)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .direction(Direction::Horizontal(Properties::new()));

                container(column![
                    cart_text,
                    row![final_price_text, items_amount_text].spacing(40).align_items(Center),
                    Space::with_height(20),
                    items_scroll
                ].spacing(15).align_items(Center)).padding(10).style(iced::theme::Container::Custom(Box::new(UserContainerStyle))).into()

            }
        }
    }
}

fn cart_items_view<'a>(cart_items: Vec<(&'a Item, &'a usize)>) -> Row<Message> {

    let path = "C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\icons\\cross-svg.svg";

    let vec: Vec<Element<Message>> = cart_items.iter().map(|(item, amount)| {
        let mut path_to_items:PathBuf = PathBuf::from("C:\\Users\\kiril\\RustroverProjects\\grocery-shop\\src\\items".to_string());
        path_to_items.push(&item.svg);
        let item_svg = Image::<image::Handle>::new(path_to_items).width(100).height(100);

        let name_of_item = &item.name;
        let amount_of_item = amount;
        let price_of_item = &item.price;
        let weight_of_item = &item.weight;

        let item_label = text(name_of_item).size(10);
        let price_label = text(format!("Цена: {price_of_item} р.")).size(10);
        let weight_label = text(format!("Вес: {weight_of_item}")).size(10);
        let amount_label = text(format!("Кол-во: {amount_of_item}")).size(10);

        let info_col = iced::widget::column![item_label, price_label, weight_label, amount_label].spacing(5);

        let svg_btn_handler = svg::Handle::from_path(path.to_string());

        let svg_btn = Svg::new(svg_btn_handler).width(35).height(35);


        let btn_del = button(svg_btn).style(iced::theme::Button::Custom(Box::new(AddButtonStyle))).on_press(Message::DeleteFromCart(item.clone().clone()));

        container(iced::widget::column![
            item_svg,
            row![info_col, btn_del].spacing(10).align_items(Center)
        ].spacing(10).align_items(Center)).padding(10).style(iced::theme::Container::Custom(Box::new(CategoryContainerStyle))).into()

    }).collect();

    Row::from_vec(vec).spacing(10)
}

