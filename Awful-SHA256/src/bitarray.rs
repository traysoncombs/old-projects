use std::ops;
use std::convert::TryInto;
use std::borrow::Borrow;

#[derive(Debug)]
#[derive(Clone)]
pub struct BitArray { // This is very wrong, everything should be done using 8 bits not 32, but this is easier and im lazy.
  pub array: Vec<char>
} impl BitArray {

  pub fn zero_filled() -> BitArray {
    BitArray {
      array: vec!['0'; 32]
    }
  }

  pub fn from_str(data: String) -> BitArray {
    //let mut padding = vec!['0'; (32-data.len())];
    //padding.append(&mut data.chars().collect());
    BitArray {
      array: data.chars().collect()
    }
  }

  pub fn from_int(data: u32) -> BitArray {
    BitArray {
      array: prepend_zeros(data)
    }
  }

  pub fn to_int(&self) -> u32 {
    let bin_digits : String = (&self.array).into_iter().collect();
    u32::from_str_radix(&bin_digits, 2).unwrap()
  } 

  pub fn to_vec(&self, int: u32) -> Vec<char> {
    prepend_zeros(int)
  }

  pub fn rotate_right(&mut self, pos: u32) -> Self{
    self.array = self.to_vec(self.to_int().rotate_right(pos));
    self.clone()
  }

  pub fn rotate_left(&mut self, pos: u32) -> Self {
    self.array = self.to_vec(self.to_int().rotate_left(pos));
    self.clone()
  }

  pub fn len(&self) -> usize {
    self.array.len()
  }

  pub fn byte_len(&self) -> usize { // this is wrong because we are doing things with 32 bits and trying to find length in 8 bits
    let mut sum = 0;
    for chunk in self.array.chunks(8) {
      if chunk.contains(&'1') {
        sum += 8;
      }
    }
    sum
  }
}

impl ops::Shr<usize> for BitArray {
  type Output = Self;

  fn shr(mut self, _rhs: usize) -> Self {
    self.array = self.to_vec(self.to_int() >> _rhs);
    self.clone()
  }
}

impl ops::Shl<usize> for BitArray {
  type Output = Self;

  fn shl(mut self, _rhs: usize) -> Self {
    self.array = self.to_vec(self.to_int() << _rhs);
    self.clone()
  }
}

impl ops::Add<BitArray> for BitArray {
  type Output = Self;

  fn add(self, _rhs: BitArray) -> Self {
    BitArray::from_int((self.to_int() + _rhs.to_int()) % 2^32)
  }
}

impl ops::BitXor<BitArray> for BitArray {
  type Output = Self;

  fn bitxor(self, _rhs: BitArray) -> Self {
    BitArray::from_int(self.to_int() ^ _rhs.to_int())
  }
}

impl ops::BitAnd<BitArray> for BitArray {
  type Output = Self;

  fn bitand(self, _rhs: BitArray) -> Self {
    BitArray::from_int(self.to_int() & _rhs.to_int())
  }
}
#[derive(Debug)]
#[derive(Clone)]
pub struct DoubleBitArray {
  pub arrays: Vec<BitArray>
} impl DoubleBitArray {

  pub fn new() -> DoubleBitArray {
    DoubleBitArray {
      arrays: vec![BitArray {array: Vec::new()}]
    }
  }

  pub fn from_str(data: String) -> DoubleBitArray{
    DoubleBitArray {
      arrays: vec![BitArray::from_str(data)]
    }
  }

  pub fn from_int(data: u32) -> DoubleBitArray{
    DoubleBitArray {
      arrays: vec![BitArray::from_int(data)]
    }
  }

  pub fn append_char(&mut self, data: char) {
    let bitarray = &self.arrays[self.arrays.len()-1];
    if bitarray.len() <= 31 {
      let arrays_len = self.arrays.len();
      self.arrays[arrays_len-1].array.push(data);
    } else {
      self.arrays.push(BitArray::from_str(char::to_string(&data)));
    }
  }

  pub fn push_array(&mut self, data: BitArray) {
    self.arrays.push(data);
  }

  pub fn len(&self) -> usize {
    let mut sum = 0;
    for arr in self.arrays.iter() {
      sum += arr.len();
    }
    sum
  }

  pub fn byte_len(&self) -> usize {
    let mut sum = 0;
    for arr in self.arrays.iter() { 
      sum += arr.byte_len();
    }
    sum
  }
}

pub fn prepend_zeros(data: u32) -> Vec<char> {
  let prefixed_bits : Vec<char> = format!("{:#034b}", data).chars().collect();
  (&prefixed_bits[2..34]).to_vec()
}