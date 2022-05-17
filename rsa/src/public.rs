use num_bigint::{BigUint, BigInt};
use num_bigint::{ToBigInt, RandBigInt, ToBigUint};
use num_traits::pow::Pow;
use std::str;
use crate::encoding;


#[derive(Debug)]
pub struct Public {
  e : BigUint,
  n : BigUint
}
  impl Public {
    pub fn new(e: BigUint, n: BigUint) -> Public {
      Public {
        e: e,
        n: n
      }
    }

    fn encrypt_int(&self, pt : &BigUint) -> BigUint {
      pt.modpow(&self.e, &self.n)
    }

    pub fn encrypt_string(&self, pt : String) -> String {
      let mut out_blocks = Vec::new();
      let in_blocks : Vec<BigUint> = encoding::string_to_blocks(pt);
      for b in in_blocks {
        out_blocks.push(self.encrypt_int(&b));
      }
      encoding::encode_blocks(&out_blocks)
    }

    pub fn verify_sig(&self, sig: &BigUint, h : &BigUint) -> bool {
      let s = sig.modpow(&self.e, &self.n);
      if s == *h {true} else {false}
    }

    pub fn to_string(&self) -> String {
      let e = base64::encode(&self.e.to_bytes_le());
      let n = base64::encode(&self.n.to_bytes_le());
      base64::encode(e+":"+&n)
    }

    pub fn from_string(encoded: String) -> Public {
      let s = base64::decode(encoded).unwrap();
      let parts : Vec<&str> = str::from_utf8(&s).unwrap().split(":").collect();
      Public {
        e: BigUint::from_bytes_le(&base64::decode(parts[0]).unwrap()),
        n: BigUint::from_bytes_le(&base64::decode(parts[1]).unwrap())
      }
    }
  }