use num_bigint::BigUint;
use num_bigint::{BigInt, ToBigInt};
use std::ops::Add;
use std::ops::Mul;
use crate::algorithms::mod_inv;
use crate::algorithms::euclid_mod;
use std::fmt;
use crate::utils::{bigint_to_string, bigint_from_string};
use std::time::SystemTime;
use crate::utils;
use crate::curve::Curve;


#[derive(std::clone::Clone)]
pub struct Point<'a> {
  pub x: BigInt,
  pub y: BigInt,
  pub curve: &'a Curve
}
  impl<'a> Point<'a> {
    pub fn new(x: BigInt, y: BigInt, curve: &Curve) -> Point {
      Point {
        x: x,
        y: y,
        curve: curve
      }
    }

    pub fn on_curve(&self) -> bool {
      self.curve.on_curve(&self)
    }

    pub fn to_string(&self) -> String {
      base64::encode(bigint_to_string(&self.x) + ":" + &bigint_to_string(&self.y) + ":" + &self.curve.to_string().to_string())
    }

    pub fn from_string(s: String, curv: &'a mut Curve) -> Point<'a> {
      let s = String::from_utf8(base64::decode(s).unwrap()).unwrap();
      let c : Vec<&str> = s.split(":").collect();
      Point {
        x: utils::bigint_from_string(c[0].to_string()),
        y: utils::bigint_from_string(c[1].to_string()),
        curve: curv.populate_from_string(c[2].to_string()),
      }
    }
  }

  impl Add for Point<'_> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
      if &self.y == &(-&other.y) || &(-&self.y) == &other.y{
        panic!("Points are negative of each other!!!");
      }
      let mut s : BigInt;
      if self.x == other.x && self.y == other.y {
        s = euclid_mod(&((3*self.x.pow(2) + &self.curve.a) * mod_inv(&(2*&self.y), &self.curve.p)), &self.curve.p);
      } else {
        s = euclid_mod(&((&other.y-&self.y) * mod_inv(&(&other.x-&self.x), &self.curve.p)), &self.curve.p);
        //println!("points are distinct: {:?}", &s);
        //println!("part  1: {}", (&other.y-&self.y));
        //println!("part  2: {}", mod_inv(&(&other.x-&self.y), &self.curve.p));
        //println!("part  2 args: {}, {}", &other.x-&self.x,  &self.curve.p);
      }
      let x3 = euclid_mod(&(s.pow(2) - &self.x - &other.x), &self.curve.p); // euclid_mod is % but better, and more cancer
      Point {
        x: x3.clone(),
        y: euclid_mod(&(s*(&self.x-x3)-self.y), &self.curve.p),
        curve: self.curve
      }
    }
  }

  impl Mul<BigInt> for Point<'_> {
    type Output = Self;

    fn mul(self, scalar: BigInt) -> Self {
      
      let mut addend = self.clone();
      let mut result : Option<Point> = None;
      let mut n : BigInt = scalar.clone();
      let mut first = true; // yikes
      while &n != &0.to_bigint().unwrap() {
        let now = SystemTime::now();
        //println!("bits: {:b}", &n);
        if (&n & 0b1.to_bigint().unwrap()) == 1.to_bigint().unwrap() {
          println!("bit is one");
          result = if result.is_none() {Some(addend.clone())} else {Some(result.clone().unwrap()+addend.clone())};
          //println!("{}", result.clone().unwrap());
        }
        addend = addend.clone() + addend.clone();
        n >>= 1;
        println!("one iter: {:?}", now.elapsed());
      }
      result.unwrap()
    }
  }

  impl fmt::Display for Point<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", &self.x, &self.y)
    }
  }

  