use num_bigint::{BigInt, ToBigInt};
use crate::point::Point;
use crate::algorithms::{mod_inv, euclid_mod};
use std::fmt;
use base64;

struct PublicKey<'a> {
  p: Point<'a>
}

pub struct Signature {
  r: BigInt,
  s: BigInt
}
  impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r: {}, s: {}", &self.r, &self.s)
    }
  }

pub struct KeyPair<'a> {
  pub private: BigInt,
  pub public: Point<'a>
}
  impl KeyPair<'_> {
    pub fn sign(&self, h: &BigInt) -> Signature {
      let r = self.public.x.clone();
      let s = euclid_mod(&(mod_inv(&self.private, &self.public.curve.p) * (h.clone() + r.clone() * self.private.clone())), &self.public.curve.p);
      Signature {
        r: r,
        s: s
      }
    }
  }