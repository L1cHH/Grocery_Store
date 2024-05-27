use std::collections::HashMap;
use std::{env};
use std::env::current_dir;
use std::fs::File;
use std::io::{Read};
use iced::{Application, Color, Command, Element, executor, Length, Padding, Renderer, Sandbox, Settings, Theme};
use iced::Alignment::Center;
use iced::widget::{button, container, text, Svg, column, Text, text_input, TextInput, Row, Scrollable, row, Space};
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::svg::Handle;
use crate::grocery_shop::catalog::{Item, Category};
use crate::grocery_shop::GroceryShop;
use crate::pages::entry_page_state::EntryPageState;
use crate::pages::Page;
use crate::styles::{UserButtonStyle, UserContainerStyle, UserInputStyle};
use crate::user::Buyer;

mod user;
mod grocery_shop;
mod pages;
mod styles;
mod cart;

fn main() -> iced::Result {
    GroceryShop::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    BonusCardInputChanged(String),
    CreditCardInputChanged(String),
    CashInputChanged(String),

    ToCategoryPage,
    ToItemsPage(Category),

    AddToCart(Item),
    DeleteFromCart(Item),

    PayByCash(usize),
    PayByCreditCard(usize),

    PayByCashAndBonus(usize),
    PayByCreditCardAndBonus(usize),
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
                        match is_correct_input(amount.clone()) {
                            Ok(()) => entry_page_state.bonus_card_input = amount,
                            Err(str) => eprintln!("{str}")
                        }

                        Command::none()
                    }
                    Message::CreditCardInputChanged(amount) => {
                        match is_correct_input(amount.clone()) {
                            Ok(()) => entry_page_state.credit_card_input = amount,
                            Err(str) => eprintln!("{str}")
                        }

                        Command::none()
                    }
                    Message::CashInputChanged(amount) => {
                        match is_correct_input(amount.clone()) {
                            Ok(()) => entry_page_state.cash_input = amount,
                            Err(str) => eprintln!("{str}")
                        }

                        Command::none()
                    }
                    Message::ToCategoryPage => {
                        let buyer = Buyer::new(
                            entry_page_state.bonus_card_input.clone(),
                            entry_page_state.credit_card_input.clone(),
                            entry_page_state.cash_input.clone(),
                        );

                        self.add_buyer(buyer);
                        self.page = Page::CategoryPage;
                        Command::none()
                    },
                    _ => {Command::none()}
                }
            }
            Page::CategoryPage => {
                match message {
                    Message::ToItemsPage(category) => {
                        self.page = Page::CatalogPage(category);
                        Command::none()
                    },
                    Message::AddToCart(item) => {
                        {
                            let items = self.get_items_by_category_mut(item.category.clone()).unwrap();
                            match items.iter_mut().find(|element| **element == item) {
                                Some(item) => { item.amount -= 1 },
                                None => panic!("Cant find particular item in CartMessage::AddItem")
                            }
                        }


                        let user_handler = self.get_user_mut().unwrap();

                        let cart = user_handler.get_cart_mut();

                        cart.add_item(item);


                        Command::none()
                    }

                    Message::DeleteFromCart(item) => {
                        {
                            let items = self.get_items_by_category_mut(item.category.clone()).unwrap();
                            match items.iter_mut().find(|element| element.name == item.name) {
                                Some(item) => { item.amount += 1 },
                                None => panic!("Cant find particular item in CartMessage::DeleteFromCart")
                            }
                        }

                        let user_handler = self.get_user_mut().unwrap();

                        let cart = user_handler.get_cart_mut();

                        cart.delete_item(item);

                        Command::none()
                    }
                    _ => {Command::none()}
                }
            }
            Page::CatalogPage(_) => {
                match message {
                    Message::AddToCart(item) => {
                        {
                            let items = self.get_items_by_category_mut(item.category.clone()).unwrap();
                            match items.iter_mut().find(|element| **element == item) {
                                Some(item) => { item.amount -= 1 },
                                None => panic!("Cant find particular item in CartMessage::AddItem")
                            }
                        }


                        let user_handler = self.get_user_mut().unwrap();

                        let cart = user_handler.get_cart_mut();

                        cart.add_item(item);


                        Command::none()
                    }

                    Message::DeleteFromCart(item) => {
                        {
                            let items = self.get_items_by_category_mut(item.category.clone()).unwrap();
                            match items.iter_mut().find(|element| element.name == item.name) {
                                Some(item) => { item.amount += 1 },
                                None => panic!("Cant find particular item in CartMessage::DeleteFromCart")
                            }
                        }

                        let user_handler = self.get_user_mut().unwrap();

                        let cart = user_handler.get_cart_mut();

                        cart.delete_item(item);

                        Command::none()
                    }

                    Message::ToCategoryPage => {
                        self.page = Page::CategoryPage;
                        Command::none()
                    }

                    Message::PayByCash(price) => {
                        let user = self.get_user_mut().unwrap();

                        match user.try_to_pay_cash(price) {
                            Ok(msg) => {
                                println!("{msg}")
                            }
                            Err(err_msg) => {
                                eprintln!("{err_msg}")
                            }
                        }

                        Command::none()
                    }

                    Message::PayByCreditCard(price) => {
                        let user = self.get_user_mut().unwrap();

                        match user.try_to_pay_credit_card(price) {
                            Ok(msg) => {
                                println!("{msg}")
                            }
                            Err(err_msg) => {
                                eprintln!("{err_msg}")
                            }
                        }

                        Command::none()
                    }

                    Message::PayByCreditCardAndBonus(price) => {
                        let user = self.get_user_mut().unwrap();

                        match user.use_bonuses(price) {
                            Ok(rest) => {
                                match rest {
                                    0 => {},
                                    rest => {
                                        match user.try_to_pay_credit_card(rest) {
                                            Ok(msg) => {
                                                println!("{msg}")
                                            }
                                            Err(err_msg) => {
                                                eprintln!("{err_msg}")
                                            }
                                        }
                                    }
                                }
                            }
                            Err(err_msg) => {
                                eprintln!("{err_msg}")
                            }
                        }



                        Command::none()
                    }

                    Message::PayByCashAndBonus(price) => {
                        let user = self.get_user_mut().unwrap();

                        match user.use_bonuses(price) {
                            Ok(rest) => {
                                match rest {
                                    0 => {},
                                    rest => {
                                        match user.try_to_pay_cash(rest) {
                                            Ok(msg) => {
                                                println!("{msg}")
                                            }
                                            Err(err_msg) => {
                                                eprintln!("{err_msg}")
                                            }
                                        }
                                    }
                                }
                            }
                            Err(err_msg) => {
                                eprintln!("{err_msg}")
                            }
                        }



                        Command::none()
                    }

                    _ => {todo!()}
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, Renderer> {

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
            },
            Page::CategoryPage => {
                let user_label: Text<'_, Theme, Renderer> = text("Выберите Категорию")
                    .size(35)
                    .style(iced::theme::Text::Color(Color::from_rgba8(187, 104, 147, 0.8))).into();

                let category_vec:Vec<Element<'_, Self::Message, Self::Theme, Renderer>> = {
                    self.get_curr_catalog().keys().map(|category| {
                        category.view()
                    }).collect()
                };

                let category_row = Row::from_vec(category_vec).spacing(10);

                let scrollable = Scrollable::new(category_row)
                    .width(Length::Fill)
                    .height(Length::Shrink)
                    .direction(Direction::Horizontal(Properties::new()));

                let user = self.get_user().unwrap();

                let cart = user.get_cart();

                let cart_items_view: Element<_> = cart.view();

                container(column![
                    user_label,
                    Space::with_height(10),
                    container(scrollable).padding(20),
                    Space::with_height(20),
                    cart_items_view
                ].spacing(20).align_items(Center))
                    .width(800)
                    .height(1500)
                    .center_x()
                    .center_y()

            },
            Page::CatalogPage(category) => {
                let category_name = &category.category_name;
                let catalog_label = text(format!("Каталог категории '{category_name}'")).size(20);


                let catalog = self.get_items_by_category(category).unwrap();

                let mut items_container: Vec<Element<_>> = {
                    catalog
                        .iter()
                        .map(|item| {
                            item.view()
                        }).collect()
                };

                let items_row = Row::from_vec(items_container).spacing(10);

                let items_scroll = Scrollable::new(items_row)
                        .width(Length::Fill)
                        .height(Length::Shrink)
                        .direction(Direction::Horizontal(Properties::new()));

                let category_btn = button("Сменить категорию")
                    .padding(Padding::from([15, 40]))
                    .on_press(Message::ToCategoryPage);

                let user = self.get_user().unwrap();

                let cart = user.get_cart();

                let cart_items_view: Element<_> = cart.view();

                let payment_menu = user.view();

                let items_container = container(column![
                    catalog_label,
                    Space::with_height(10),
                    items_scroll,
                    category_btn,
                ].spacing(20).align_items(Center)).center_x().center_y().padding(15).style(iced::theme::Container::Custom(Box::new(UserContainerStyle)));


                container(row![
                    column![
                    items_container,
                    Space::with_height(10),
                    cart_items_view].spacing(20).align_items(Center),
                    payment_menu].spacing(10).align_items(Center)
                ).width(1000).height(1200).center_y().center_x().into()

            },
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
        items_map.entry(item.category.clone()).and_modify(|vector| vector.push(item.clone())).or_insert(vec![item]);
    }

    items_map
}
fn user_input(placeholder: &str, val: &str) -> TextInput<'static, Message>{
    text_input(placeholder, val)
        .width(150)
        .padding(10)
        .size(15)
        .style(iced::theme::TextInput::Custom(Box::new(UserInputStyle)))
}
fn is_correct_input(amount: String) -> Result<(), String> {
    match amount.parse::<usize>() {
        Ok(_) => {
            Ok(())
        }
        Err(_) => {
            Err("Incorrect input".to_string())
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::cart::{Cart, CartWithItems};
    use crate::grocery_shop::catalog::{Category, Item};
    use crate::grocery_shop::GroceryShop;
    use crate::pages::entry_page_state::EntryPageState;
    use crate::pages::Page;
    use crate::read_json_file;
    use crate::user::Buyer;

    #[test]
    fn it_returns_true() {
        assert!(true)
    }

    #[test]
    fn it_successfully_add_catalog() {
        let data = read_json_file();

        let shop = GroceryShop::new(data.clone(), Page::EntryPage(EntryPageState::default()));
        assert_eq!(&data, shop.get_curr_catalog())
    }

    #[test]
    fn it_successfully_add_item_to_cart() {

        let user = Buyer::new(
            "6000".to_string(),
            "6000".to_string(),
            "6000".to_string(),
        );

        let item = Item::new(
            1,
            "Креветки".to_string(),
            Category {category_name: "Seafood".to_string(), svg: "some_path".to_string()},
            1000,
            50,
            120,
            "some_path".to_string()
        );

        let cart_with_items = CartWithItems::new();

        let mut cart = Cart::NonEmptyCart(cart_with_items);

        cart.add_item(item);

        assert_eq!(cart.get_cart_items_amount(), 1)
    }


}