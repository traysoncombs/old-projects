use super::coin::*;
use super::errors::*;
use super::transaction::*;
use chrono::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct Account {
    pub transactions: Vec<Transaction>,
    pub balances: HashMap<Coins, f64>,
    pub account_id: u128,
}
impl Account {
    pub fn new() -> Account {
        Account {
            transactions: Vec::new(),
            balances: HashMap::new(),
            account_id: Uuid::new_v4().as_u128(),
        }
    }
    pub fn deposit(
        &mut self,
        coin: Coins,
        amount: f64,
        taxable_type: TaxableType,
        fees: Fees,
        from: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> SpecialResult<&Transaction> {
        self.modify_bal(coin, amount)?;
        self.push_and_return(Transaction::new_deposit(
            taxable_type,
            amount,
            coin,
            fees,
            AccountTransferType::MyWallet(Some(self.account_id)),
            from,
            coin_price,
            time,
        ))
    }

    pub fn withdraw(
        &mut self,
        coin: Coins,
        amount: f64,
        taxable_type: TaxableType,
        fees: Fees,
        to: AccountTransferType,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> SpecialResult<&Transaction> {
        self.modify_bal(coin, amount * -1.0000000000000)?;
        self.push_and_return(Transaction::new_withdrawl(
            taxable_type,
            amount,
            coin,
            fees,
            to,
            AccountTransferType::MyWallet(Some(self.account_id)),
            coin_price,
            time,
        ))
    }

    pub fn convert(
        &mut self,
        to_coin: Coins,
        from_coin: Coins,
        to_amount: f64,
        from_amount: f64,
        to_fees: Fees,
        from_fees: Fees,
        to_coin_price: f64,
        from_coin_price: f64,
        time: DateTime<Utc>,
    ) -> SpecialResult<&Transaction> {
        self.modify_bal(from_coin, from_amount * -1.0000000000000)?;
        self.modify_bal(to_coin, to_amount)?;
        self.push_and_return(Transaction::new_conversion(
            to_amount,
            from_amount,
            to_coin,
            from_coin,
            to_fees,
            from_fees,
            AccountTransferType::MyWallet(Some(self.account_id)),
            AccountTransferType::MyWallet(Some(self.account_id)),
            to_coin_price,
            from_coin_price,
            time,
        ))
    }

    pub fn buy(
        &mut self,
        coin: Coins,
        amount: f64,
        fees: Fees,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> SpecialResult<&Transaction> {
        self.deposit(
            coin,
            amount,
            TaxableType::Buy,
            fees,
            AccountTransferType::OtherWallet,
            coin_price,
            time,
        )
    }

    pub fn sell(
        &mut self,
        coin: Coins,
        amount: f64,
        fees: Fees,
        coin_price: f64,
        time: DateTime<Utc>,
    ) -> SpecialResult<&Transaction> {
        self.withdraw(
            coin,
            amount,
            TaxableType::Sell,
            fees,
            AccountTransferType::OtherWallet,
            coin_price,
            time,
        )
    }

    pub fn push_and_return(&mut self, transaction: Transaction) -> SpecialResult<&Transaction> {
        self.transactions.push(transaction);
        self.transactions.last().ok_or(SpecialError::new(
            SpecialErrorKind::TransactionCreationError,
            "Unable to create transaction",
        ))
    }

    pub fn modify_bal(&mut self, coin: Coins, amount: f64) -> SpecialResult<bool> {
        let coin_balance = self.balances.entry(coin).or_insert(0.0);
        if (*coin_balance + amount) < 0.0 {
            Err(SpecialError::new(
                SpecialErrorKind::InsuffcientBalance,
                "Insufficient balance",
            ))
        } else {
            *coin_balance += amount;
            Ok(true)
        }
    }

    pub fn get_bal(&self, coin: &Coins) -> Option<f64> {
        self.balances.get(coin).map(|f| f.to_owned())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn buy_test() {
        let mut account = Account::new();
        let fees = Fees {
            network_fee: Some(0.0),
            exchange_fee: None,
            currency: Coins::Bitcoin,
        };
        let trans = account.buy(Coins::Bitcoin, 0.000012, fees, 30_000.00, Utc::now());
        assert_eq!(0.000012, account.get_bal(&Coins::Bitcoin).unwrap());
    }

    #[test]
    fn sell_test() {
        let mut account = Account::new();
        let buy_fees = Fees {
            network_fee: Some(0.0),
            exchange_fee: None,
            currency: Coins::Bitcoin,
        };
        let sell_fees = Fees {
            network_fee: Some(0.0),
            exchange_fee: None,
            currency: Coins::Bitcoin,
        };
        let trans = account.buy(
            Coins::Bitcoin,
            0.0000120000000,
            buy_fees,
            30_000.00,
            Utc::now(),
        );
        let sell = account.sell(
            Coins::Bitcoin,
            0.0000050000000,
            sell_fees,
            40_000.00,
            Utc::now(),
        );
        assert_eq!(0.0000070000000, account.get_bal(&Coins::Bitcoin).unwrap())
    }
}
