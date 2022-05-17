use crate::point::Point;
use crate::algorithms::euclid_mod;
use crate::keys;
use num_bigint::BigUint;
use num_bigint::{BigInt, ToBigInt};
use std::fmt;
use crate::utils;

pub struct Curve {
  g_x: BigInt,
  g_y: BigInt,
  pub p: BigInt,
  pub a: i32,
  pub b: i32
}
  impl Curve {
    pub fn new_secp256k1() -> Curve {
      Curve {
        g_x: BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
        g_y: BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap(),
        p: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap(),
        a: 0,
        b: 7,
      }
    }

    pub fn new_empty() -> Curve {
      Curve {
        g_x: BigInt::parse_bytes(b"0", 16).unwrap(),
        g_y: BigInt::parse_bytes(b"0", 16).unwrap(),
        p: BigInt::parse_bytes(b"0", 16).unwrap(),
        a: 0,
        b: 0,
      }
    }

    pub fn new_test_curve() -> Curve {
      Curve {
        g_x: BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
        g_y: BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap(),
        p: 97.to_bigint().unwrap(),
        a: 2,
        b: 3
      }
    }

    pub fn on_curve(&self, p: &Point) -> bool {
      p.y.modpow(&2.to_bigint().unwrap(), &self.p) == euclid_mod(&(p.x.pow(3) + (&self.a*&p.x) + &self.b), &self.p)
    }

    pub fn new_point<T: ToBigInt, V: ToBigInt>(&self, x: T, y: V) -> Point {
      let p = Point::new(x.to_bigint().unwrap(), y.to_bigint().unwrap(), &self);
      assert!(self.on_curve(&p));
      p
    }

    pub fn new_keypair(&self, k: &BigInt) -> keys::KeyPair {
      let g = Point::new(self.g_x.clone(), self.g_y.clone(), &self);
      keys::KeyPair {
        private : k.clone(),
        public: g*k.clone()
      }
    }

    pub fn to_string(&self) -> String {
      base64::encode((utils::bigint_to_string(&self.g_x) + ":" + &utils::bigint_to_string(&self.g_y) + ":" + &utils::bigint_to_string(&self.p) + ":" + &self.a.to_string() + ":" + &self.b.to_string()))
    }

    pub fn from_string(s: String) -> Curve {
      let s = String::from_utf8(base64::decode(s).unwrap()).unwrap();
      let c : Vec<&str> = s.split(":").collect();
      Curve {
        g_x: utils::bigint_from_string(c[0].to_string()),
        g_y: utils::bigint_from_string(c[1].to_string()),
        p: utils::bigint_from_string(c[2].to_string()),
        a: c[3].parse::<i32>().unwrap(),
        b: c[4].parse::<i32>().unwrap(),
      }
    }

    pub fn populate_from_string(&mut self, s: String) -> &Self {
      let s = String::from_utf8(base64::decode(s).unwrap()).unwrap();
      let c : Vec<&str> = s.split(":").collect();
      self.g_x = utils::bigint_from_string(c[0].to_string());
      self.g_y = utils::bigint_from_string(c[1].to_string());
      self.p = utils::bigint_from_string(c[2].to_string());
      self.a = c[3].parse::<i32>().unwrap();
      self.b = c[4].parse::<i32>().unwrap();
      self
    }
  }

  impl fmt::Display for Curve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "g_x: {}, g_y: {}, p: {}", &self.g_x, &self.g_y, &self.p)
    }
  }
