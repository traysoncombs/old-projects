use base64::{encode, decode};
use num_bigint::{BigInt};

pub fn bigint_to_string(big: &BigInt) -> String {
  encode(&big.to_signed_bytes_le()[..])
}

pub fn bigint_from_string(s: String) -> BigInt {
  BigInt::from_signed_bytes_le(&decode(s).unwrap()[..])
}