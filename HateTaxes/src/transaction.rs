use super::coin::*;
use chrono::prelude::*;

#[derive(Debug)]
pub enum TransactionKind {
    Deposit,
    Withdrawl,
    Conversion,
}

#[derive(Debug)]
pub enum TaxableType {
    Payment,
    Sell,
    Buy,
    Conversion,
    Transfer,
    Income,
}

#[derive(Debug)]
pub enum AccountTransferType {
    MyWallet(Option<u128>),
    OtherWallet,
    None,
}

#[derive(Debug)]
pub struct Transaction {
    pub kind: TransactionKind,
    pub taxable_type: TaxableType,
    pub amount: f64,
    pub coin: Coins,
    pub conversion: Option<Conversion>,
    pub fees: Fees,
    pub to: AccountTransferType,
    pub from: AccountTransferType,
    pub coin_price: f64,
    pub time: DateTime<FixedOffset>,
}
impl Transaction {
    pub fn new(
        kind: TransactionKind,
        taxable_type: TaxableType,
        amount: f64,
        coin: Coins,
        conversion: Option<Conversion>,
        fees: Fees,
        to: AccountTransferType,
        from: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction {
            kind,
            taxable_type,
            amount,
            coin,
            conversion,
            fees,
            to,
            from,
            coin_price,
            time,
        }
    }
    pub fn new_deposit(
        taxable_type: TaxableType,
        amount: f64,
        coin: Coins,
        fees: Fees,
        to: AccountTransferType,
        from: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction::new(
            TransactionKind::Deposit,
            taxable_type,
            amount,
            coin,
            None,
            fees,
            to,
            from,
            coin_price,
            time,
        )
    }
    pub fn new_withdrawl(
        taxable_type: TaxableType,
        amount: f64,
        coin: Coins,
        fees: Fees,
        to: AccountTransferType,
        from: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction::new(
            TransactionKind::Withdrawl,
            taxable_type,
            amount,
            coin,
            None,
            fees,
            to,
            from,
            coin_price,
            time,
        )
    }
    pub fn new_conversion(
        to_amount: f64,
        from_amount: f64,
        to_coin: Coins,
        from_coin: Coins,
        to_fees: Fees,
        from_fees: Fees,
        to: AccountTransferType,
        from: AccountTransferType,
        to_coin_price: f64,
        from_coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction::new(
            TransactionKind::Conversion,
            TaxableType::Conversion,
            to_amount,
            to_coin,
            Some(Conversion {
                from_coin,
                from_coin_price,
                from_amount,
                from_fees,
                to_coin,
                to_coin_price,
                to_amount,
                to_fees,
            }),
            to_fees,
            to,
            from,
            to_coin_price,
            time,
        )
    }
    pub fn new_buy(
        amount: f64,
        coin: Coins,
        fees: Fees,
        to: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction::new_deposit(
            TaxableType::Buy,
            amount,
            coin,
            fees,
            to,
            AccountTransferType::OtherWallet,
            coin_price,
            time,
        )
    }
    pub fn new_sell(
        amount: f64,
        coin: Coins,
        fees: Fees,
        from: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction::new_withdrawl(
            TaxableType::Sell,
            amount,
            coin,
            fees,
            AccountTransferType::OtherWallet,
            from,
            coin_price,
            time,
        )
    }
    pub fn new_payment(
        amount: f64,
        coin: Coins,
        fees: Fees,
        from: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> Transaction {
        Transaction::new_withdrawl(
            TaxableType::Sell,
            amount,
            coin,
            fees,
            AccountTransferType::OtherWallet,
            from,
            coin_price,
            time,
        )
    }
}

#[derive(Debug)]
pub struct Conversion {
    pub from_coin: Coins,
    pub from_coin_price: f64,
    pub from_amount: f64,
    pub from_fees: Fees,
    pub to_coin: Coins,
    pub to_coin_price: f64,
    pub to_amount: f64,
    pub to_fees: Fees,
}

#[derive(Debug, Clone, Copy)]
pub struct Fees {
    pub network_fee: Option<f64>,
    pub exchange_fee: Option<f64>,
    pub currency: Coins,
}
