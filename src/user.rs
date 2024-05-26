use iced::Element;
use iced::widget::text;
use crate::cart::Cart;
use crate::Message;

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

    pub fn get_cart(&self) -> &Cart {
        &self.cart
    }

    pub fn get_cart_mut(&mut self) -> &mut Cart {&mut self.cart}

    // pub fn view(&self) -> Element<Message> {
    //     let bonus_balance = &self.bonus_card_balance;
    //     let credit_balance = &self.credit_card_balance;
    //     let cash_balance = &self.cash_in_hand;
    //
    //     let bonus_text = text(format!("Баланс бонусов: {bonus_balance}")).size(20);
    //     let credit_text = text(format!("Баланс кредитной карты: {credit_balance}")).size(20);
    //     let cash_text = text(format!("Наличные: {cash_balance}")).size(20);
    //
    //
    //
    // }
}




