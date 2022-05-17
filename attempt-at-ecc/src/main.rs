mod curve;
mod point;
mod algorithms;
mod keys;
mod utils;
use num_bigint::{BigInt, ToBigInt, RandBigInt};
use std::time::SystemTime;

fn main() {
  let mut rng = rand::thread_rng();
  //println!("key: {:?}", &r);
  let secp256k1 = curve::Curve::new_secp256k1();
  //let test = curve::Curve::new_test_curve();
  //println!("curve: {}", &test);
  //let p = secp256k1.new_point(BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(), BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap());
  //println!("p: {}", &p);
  let t = SystemTime::now();
  //println!("mult: {}", p*r);
  //println!("time elapsed: {:?}", t.elapsed().unwrap());
  let k = rng.gen_biguint(256).to_bigint().unwrap();
  let h = rng.gen_biguint(256).to_bigint().unwrap();
  let keys = secp256k1.new_keypair(&k);
  let signature = keys.sign(&h);
  println!("curve: {}", &secp256k1);
  println!("sig: {}", &signature);
  println!("key: {:?}", &k);
  let k_str = utils::bigint_to_string(&k);
  println!("key_str: {}",&k_str);
  println!("key: {:?}",utils::bigint_from_string(k_str));
  let curve_str = secp256k1.to_string();
  println!("Curve to str: {}", &curve_str);
  println!("Str to curve: {}", curve::Curve::from_string(curve_str));
  println!("Point: {}", keys.public.to_string());
}