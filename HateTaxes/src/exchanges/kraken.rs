use super::account::*;
use super::coin::*;
use super::errors::*;
use super::exchange::*;
use std::fs::File;
use super::transaction::*;
use std::io::{BufRead, BufReader, Lines};
use chrono::{DateTime, FixedOffset, TimeZone};


enum SpecificType {
  Spend,
  Deposit,
  Withdrawal,
  Receive,
  Trade,
  MarginTrade,
  Rollover,
  Transfer,
  Adjustment,
  Settled,
  Reward,
  Sale
}

enum TransType {
  Deposit,
  Withdrawl,
  InternalWithdrawl,
  InternalDeposit,
  Trade,
  Reward,
}

struct KrakenEntry {
  txid: String,
  refid: String,
  transaction_type: TransType,
  time: DateTime<FixedOffset>,
  specific_type: SpecificType,
  asset: Coins,
  amount: f64,
  fees: Fees,
}

pub struct Kraken {
    pub account: Account,
    pub prev_transaction: Option<KrakenEntry>
}
impl Exchange for Kraken {
    pub fn new(file: String) -> Kraken {
      let mut kraken = Kraken {
        account: Account::new(),
        prev_transaction: None
      }
    }

    fn text_to_coin(coin_text: String) -> Coins {
        match coin_text.as_str() {
            "XXBT" => Coins::Bitcoin,
            "USD.HOLD" => Coins::USD,
            "ZUSD" => Coins::NOP,
        }
    }

    fn process_line(&self, line: String) -> SpecialResult<()> {
        let parts: Vec<&str> = line.split(',').collect();
        let entry = KrakenEntry {
          txid: parts[0],
          refid: parts[1],
          transaction_type: match refid.chars().nth(0).unwrap() {
            'Q' => TransType::Deposit,
            'A' => TransType::Withdrawl,
            'B' => TransType::InternalWithdrawl,
            'T' => TransType::Trade,
            'R' => TransType::InternalDeposit,
            'S' => TransType::Reward,
          },
          time: DateTime::parse_from_str(parts[2], "%y-%m-%d %H:%M:%S").unwrap(),
          specific_type: parts[4],
          asset: self.text_to_coin(parts[6]),
          amount: parts[7],
          fees: Fees {
            network_fee: None,
            exchange_fee: parts[8],
            currency: asset
          }
        }
        self.process_entry(&entry);
        self.prev_transaction = entry;
        Ok()
        }
    }

    fn process_entry(&mut self, entry: &KrakenEntry) {
      // Ignore anything with ZUSD, it's just placeholder and doesn't really do anything, USD.HOLD has the intereting stuff
      match entry.transaction_type {
        TransType::Deposit => if entry.asset == Coins::NOP {
          return
        } else {
          
        },

      }
    }

    fn process_conversion(&mut self, entry: &KrakenEntry) {
      // If refid != prev_refid && currency != USD, do nothing
      // Conversion will be processed on next line,
      // If refid == prev_refid && currency != USD, get prev entry data and create conversion
      // Only create conversion 
    }

    pub fn process_header(&self, line: String) -> SpecialResult<bool> {
      let parts: Vec<String> = line.split(',').collect();
      Ok(true)
    }
}
