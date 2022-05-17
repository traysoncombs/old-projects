use num_bigint::{BigUint, BigInt};
use num_bigint::{ToBigInt, RandBigInt, ToBigUint};

pub fn gcd(a : BigUint, b: BigUint) -> BigUint {
  if a == 0.to_biguint().unwrap() {
    return b.clone();
  }
  return gcd(b % a.clone(), a.clone());
}

pub fn mod_inv(a: &mut BigUint, m: &mut BigUint) -> BigUint{ 
    let mut m0 = m.clone().to_bigint().unwrap(); 
    let mut y = 0.to_bigint().unwrap();
    let mut x = 1.to_bigint().unwrap(); 
  
    if *m == 1.to_biguint().unwrap() {
        return 0.to_biguint().unwrap();
    } 
  
    while *a > 1.to_biguint().unwrap() { 
        // q is quotient 
        let mut q = (&*a / &*m).to_bigint().unwrap(); 
        let mut t = m.clone().to_bigint().unwrap(); 
  
        // m is remainder now, process same as 
        // Euclid's algo 
        *m = &*a % &*m;
        *a = t.clone().to_biguint().unwrap(); 
        t = y.clone(); 
  
        // Update y and x 
        y = &x - &q * &y; 
        x = t.clone(); 
    } 
  
    // Make x positive 
    if x < 0.to_bigint().unwrap() {
        x += m0; 
    }
  
    return x.to_biguint().unwrap();
} 
