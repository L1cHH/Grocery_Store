pub trait Client {
    fn check_balance(&mut self) -> usize;
    
}