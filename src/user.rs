use iced::{Element, Padding};
use iced::Alignment::Start;
use iced::widget::{button, container, text, column, row, Space};
use crate::cart::Cart;
use crate::Message;
use crate::styles::UserContainerStyle;

mod shopping_list;
mod client_trait;

pub struct Buyer {
    bonus_card_balance: usize,
    credit_card_balance: usize,
    cash_in_hand: usize,
    cart: Cart
}
impl Buyer {
    pub fn new(bonus_balance: String, credit_balance: String, cash_balance: String) -> Self {

        let bonus = bonus_balance.parse::<usize>().expect("Cant parse bonus balance");
        let credit = credit_balance.parse::<usize>().expect("Cant parse credit balance");
        let cash = cash_balance.parse::<usize>().expect("Cant parse cash balance");

        Self {
            bonus_card_balance: bonus,
            credit_card_balance: credit,
            cash_in_hand: cash,
            cart: Cart::new()
        }
    }

    pub fn use_bonuses(&mut self, cart_price: usize) -> Result<String, String> {
        match self.bonus_card_balance {
            0 => Err("You don't have any bonuses".to_string()),
            _ => {
                if self.bonus_card_balance >= cart_price {
                    self.bonus_card_balance -= cart_price;
                    self.cart.reset_cart();
                } else {
                    self.cart.change_final_price(self.bonus_card_balance);
                    self.bonus_card_balance = 0;
                }
                Ok("You successfully used your bonuses".to_string())
            }
        }
    }

    pub fn is_enough_money(user_money: usize, cart_price: usize) -> bool {
        if user_money >= cart_price {true} else {false}
    }

    pub fn try_to_pay_cash(&mut self, cart_price: usize) -> Result<String, String> {
        match Self::is_enough_money(self.cash_in_hand, cart_price) {
            true => {
                self.cash_in_hand -= cart_price;

                self.cart.reset_cart();

                Ok("Successfully paid by cash!".to_string())
            }
            false => {
                let need_to = cart_price - self.cash_in_hand;
                Err(format!("Not successfully paid by cash, not enough {need_to} to pay"))
            }
        }
    }

    pub fn try_to_pay_credit_card(&mut self, cart_price: usize) -> Result<String, String> {
        match Self::is_enough_money(self.credit_card_balance, cart_price) {
            true => {
                self.credit_card_balance -= cart_price;

                self.cart.reset_cart();

                Ok("Successfully paid by credit card!".to_string())
            }
            false => {
                let need_to = cart_price - self.credit_card_balance;
                Err(format!("Not successfully paid by credit card, not enough {need_to} to pay"))
            }
        }
    }

    pub fn get_cart(&self) -> &Cart {
        &self.cart
    }

    pub fn get_cart_mut(&mut self) -> &mut Cart {&mut self.cart}

    pub fn view(&self) -> Element<Message> {
        let bonus_balance = &self.bonus_card_balance;
        let credit_balance = &self.credit_card_balance;
        let cash_balance = &self.cash_in_hand;
        let cart_price = self.cart.get_final_price();

        let bonus_text = text(format!("Баланс бонусов: {bonus_balance}")).size(15);
        let credit_text = text(format!("Баланс кредитной карты: {credit_balance}")).size(15);
        let cash_text = text(format!("Наличные: {cash_balance}")).size(15);
        let cart_price_text = text(format!("К оплате: {cart_price}")).size(20);

        let pay_credit_btn = button("Оплатить кред/картой").padding(Padding::from([10, 20])).on_press(Message::PayByCreditCard(*cart_price));
        let pay_cash_btn = button("Оплатить наличными").padding(Padding::from([10, 20])).on_press(Message::PayByCash(*cart_price));

        let use_bonus_btn = button("Использовать бонусы").padding(Padding::from([10, 20])).on_press(Message::UseBonuses(*cart_price));

        container(column![
            bonus_text,
            credit_text,
            cash_text,
            Space::with_height(5),
            cart_price_text,
            Space::with_height(5),
            row![column![pay_credit_btn, pay_cash_btn].spacing(10).align_items(iced::Alignment::Center), use_bonus_btn].spacing(20).align_items(iced::Alignment::Center)
        ].align_items(Start).spacing(10)).padding(15).style(iced::theme::Container::Custom(Box::new(UserContainerStyle))).into()

    }
}




