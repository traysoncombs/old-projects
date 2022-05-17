mod algorithms;
mod private;
mod public;
mod encoding;
use num_primes::{Generator};
use num_bigint::{BigUint, BigInt};
use num_bigint::{ToBigInt, RandBigInt, ToBigUint};

fn get_random_num(end : &BigUint) -> BigUint {
  let mut rng = rand::thread_rng();
  return rng.gen_bigint_range(&1.to_bigint().unwrap(), &end.to_bigint().unwrap()).to_biguint().unwrap(); // might be wrong length here
}

#[derive(Debug)]
struct KeyPair {
  public: public::Public,
  private: private::Private
}
  impl KeyPair {
    fn new(bits: usize) -> KeyPair {
      let p = Generator::new_prime(bits);
      let q = Generator::new_prime(bits);
      let n = &p*&q;
      let mut phin = (&p-1.to_biguint().unwrap())*(&q-1.to_biguint().unwrap());
      let mut e : BigUint;
      loop {
        let possible_public = get_random_num(&phin);
        if algorithms::gcd(possible_public.clone(), phin.clone()) == 1.to_biguint().unwrap() {
          e = possible_public;
          break;
        } else {
          continue;
        }
      }
      let d = algorithms::mod_inv(&mut e.clone(), &mut phin.clone());
      KeyPair {
        public: public::Public::new(e, n.clone()),
        private: private::Private::new(d, n.clone()),
      }
    }
  }


fn main() {
  let keys = KeyPair::new(2048);
  let private = private::Private::from_string(keys.private.to_string());
  println!("private: {:?}", &private.to_string());
  let public = public::Public::from_string(keys.public.to_string());
  println!("public: {:?}", &public.to_string());
  let enc = public.encrypt_string("kkkfjkjfgf0000908hhhabc123av".to_string());
  println!("en: {}", &enc);
  let dec = private.decrypt_string(enc);
  println!("dec: {}", dec);
  
}