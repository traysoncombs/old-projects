use crate::algorithms;
use num_bigint::{BigUint, BigInt};
use num_bigint::{ToBigInt, RandBigInt, ToBigUint};
use num_traits::pow::Pow;
use std::convert::TryInto;
use num_traits::cast::ToPrimitive;
use std::str;
use base64;

const BLOCK_SIZE : u32 = 3;
const MULT : u32 = 255;


pub fn string_to_blocks(msg: String) -> Vec<BigUint> {
  let mut ret : Vec<BigUint> = Vec::new();
  let msg_split : Vec<char> = msg.chars().collect();
  let mut block = String::new();
  for (i,m) in msg_split.iter().enumerate() {
    block.push(*m);
    if i == msg_split.len()-1 {
      while block.len() < 3 {
        block.push('\0');
      }
    }
    if (block.len() == 3) {
      ret.push(block_to_int(block));
      block = String::new();
    }
    
  }
  ret
}

pub fn encode_blocks(blocks: &Vec<BigUint>) -> String {
  let mut ret = String::new();
  for (i,b) in blocks.iter().enumerate() {
    let bytes = base64::encode(b.to_bytes_le());
    ret += &bytes;
    if i != blocks.len()-1 {
      ret.push(':');
    }
  }
  ret
}

pub fn decode_string(s : String) -> Vec<BigUint> {
  let blocks : Vec<&str> = s.split(":").collect();
  let mut ret = Vec::new();
  for b in blocks {
    ret.push(BigUint::from_bytes_le(&base64::decode(b).unwrap()));
  }
  ret
}

pub fn block_to_int(msg: String) -> BigUint {
  let msg_blocks = msg.as_bytes();
  println!("msg: {:?}", msg_blocks);
  let mut sum : u32 = 0;
  for e in 0..BLOCK_SIZE {
    sum += MULT.pow(e) * (msg_blocks[2-e as usize] as u32);
  }
  sum.to_biguint().unwrap()
}

pub fn int_to_block(num: BigUint) -> String {
  let mut decoded : Vec<u8> = Vec::new();
  let mut m = num.to_u32().unwrap();
  for e in (0..BLOCK_SIZE).rev() {
    decoded.push((m/MULT.pow(e)) as u8);
    m = m % MULT.pow(e);
  }
  decoded.push((m % MULT) as u8);
  str::from_utf8(&decoded[..]).unwrap().to_string()
}

pub fn blocks_to_string(blocks: &Vec<BigUint>) -> String {
  let mut ret : Vec<String> = Vec::new();
  for b in blocks {
    let s = int_to_block(b.clone());
    let mut f = String::new();
    for i in s.chars() {
      if i == '\0' {
        break;
      }
      f.push(i);
    }
    ret.push(f);
  }
  ret.join("")
}