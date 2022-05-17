use num_bigint::{BigUint, BigInt};
use num_bigint::{ToBigInt, RandBigInt, ToBigUint};

pub fn mod_inv(a: &BigInt, m: &BigInt) -> BigInt{ 
    let mut m0 = m.clone(); 
    let mut a0 = a.clone();
    let mut y = 0.to_bigint().unwrap();
    let mut x = 1.to_bigint().unwrap(); 
  
    if m0 == 1.to_bigint().unwrap() {
        return 0.to_bigint().unwrap();
    } 
  
    while a0 > 1.to_bigint().unwrap() { 
        // q is quotient 
        let mut q = &a0 / &m0; 
        let mut t = m0.clone(); 
  
        // m is remainder now, process same as 
        // Euclid's algo 
        m0 = euclid_mod(&a0, &m0); // dum
        a0 = t.clone(); 
        t = y.clone(); 
  
        // Update y and x 
        y = &x - &q * &y; 
        x = t.clone(); 
    }

    if x < 0.to_bigint().unwrap() {
        x += m; 
    }
    
    return x;
}

pub fn euclid_mod(a: &BigInt, b: &BigInt) -> BigInt {
  ((a % b) + b) % b
}

/*pub fn mod_inv(a: &BigInt, module: &BigInt) -> BigInt {
  let mut m = module.clone();
  let mut n = a.clone();
  let mut x = 0.to_bigint().unwrap();
  let mut y = 1.to_bigint().unwrap();
  //let mut mn = (module, a);
  //let mut xy = (0.to_bigint().unwrap(), 1.to_bigint().unwrap());
 
  while n != 0.to_bigint().unwrap() {
    x = y.clone();
    y = x.clone() - (m.clone() / n.clone()) * y.clone();
    m = n.clone();
    n = (m.clone() % n.clone());
    //xy = (y, x - (m / n) * y);
    //mn = (n, m % n);
  }
  x
}*/