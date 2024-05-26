use crate::cart::Cart;

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
}




