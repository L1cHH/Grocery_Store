mod shopping_list;
mod client_trait;

pub struct Buyer {
    bonus_card_balance: usize,
    credit_card_balance: usize,
    cash_in_hand: usize
}
impl Buyer {
    pub fn new(bonus_balance: String, credit_balance: String, cash_balance: String) -> Self {

        let bonus = bonus_balance.parse::<usize>().expect("Cant parse bonus balance");
        let credit = credit_balance.parse::<usize>().expect("Cant parse credit balance");
        let cash = cash_balance.parse::<usize>().expect("Cant parse cash balance");

        Self {
            bonus_card_balance: bonus,
            credit_card_balance: credit,
            cash_in_hand: cash
        }
    }
}




