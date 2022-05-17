use crate::encoding;

use num_bigint::{BigUint, BigInt};
use num_bigint::{ToBigInt, RandBigInt, ToBigUint};
use std::str;

use base64;

#[derive(Debug)]
pub struct Private {
  d : BigUint,
  n : BigUint
}
  impl Private {
     pub fn new(d: BigUint, n: BigUint) -> Private{
      Private {
        d: d,
        n: n
      }
    }

    fn decrypt_int(&self, ct : &BigUint) -> BigUint {
        return ct.modpow(&self.d, &self.n);
    }

    pub fn decrypt_string(&self, ct: String) -> String {
      let blocks : Vec<BigUint> = encoding::decode_string(ct);
      let mut out_string = Vec::new();
      for b in blocks {
        out_string.push(encoding::int_to_block(self.decrypt_int(&b)));
      }
      out_string.join("")
    }

    pub fn sign(&self, msg: &BigUint) -> BigUint {
      return msg.modpow(&self.d, &self.n);
    }

    pub fn to_string(&self) -> String {
      let d = base64::encode(&self.d.to_bytes_le());
      let n = base64::encode(&self.n.to_bytes_le());
      base64::encode(d+":"+&n)
    }

    pub fn from_string(encoded: String) -> Private {
      let s = base64::decode(encoded).unwrap();
      let parts : Vec<&str> = str::from_utf8(&s).unwrap().split(":").collect();
      Private {
        d: BigUint::from_bytes_le(&base64::decode(parts[0]).unwrap()),
        n: BigUint::from_bytes_le(&base64::decode(parts[1]).unwrap())
      }
    }
  }