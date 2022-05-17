#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Coins {
    Bitcoin,
    USD,
}
impl Coins {
    pub fn coin_id(&self) -> &str {
        match *self {
            Coins::Bitcoin => "bitcoin",
            Coins::USD => "USD",
            Coins::NOP => ""
        }
    }
}
