mod bitarray;

use self::bitarray::*;

const H0: u32 = 0x6a09e667;
const H1: u32 = 0xbb67ae85;
const H2: u32 = 0x3c6ef372;
const H3: u32 = 0xa54ff53a;
const H4: u32 = 0x510e527f;
const H5: u32 = 0x9b05688c;
const H6: u32 = 0x1f83d9ab;
const H7: u32 = 0x5be0cd19;
static ROUND_CONSTANTS: [u32; 64] = [0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2];

fn main() {
  let to_hash: u32 = 349850458;
  let mut bits = DoubleBitArray::from_str("01010110101010100101011010101010".to_string());
  pad(&mut bits);
  let schedule = create_message_schedule(&bits);

  //println!("{:?}", bits);
  println!("{:?}", schedule);
  println!("{}", bits.len());
}


fn pad(data: &mut DoubleBitArray) {
  let mut length = get_length(&data);
  data.append_char('1');
  if data.len() % 448 != 0 {
    for _ in 0..(448 - (data.len() % 448)){
      data.append_char('0');
    }
  }
  data.push_array(length.remove(0));
  data.push_array(length.remove(0));
}

fn get_length(data: &DoubleBitArray) -> Vec<BitArray> {
  let bin_arr: Vec<char> = format!("{:#066b}", data.byte_len()).chars().collect();
  let bin = (&bin_arr[2..66]).to_vec();
  let bits_one = BitArray::from_str((&bin[0..32]).into_iter().collect());
  let bits_two = BitArray::from_str((&bin[32..64]).into_iter().collect());
  vec![bits_one, bits_two]
}

fn chunk_loop(data: &DoubleBitArray) {
  for chunk in data.arrays.chunks(16) {

  }
}

fn create_message_schedule(data: &DoubleBitArray) -> DoubleBitArray {
  let mut schedule = data.clone();
  for _ in 0..48 {
    schedule.push_array(BitArray::zero_filled());
  }
  for i in 16..64 {
    let s0 = (schedule.arrays[i-15].rotate_right(7)) ^ (schedule.arrays[i-15].rotate_right(18)) ^ (schedule.arrays[i-15].clone() >> 3);
    let s1 = (schedule.arrays[i-2].rotate_right(17)) ^ (schedule.arrays[i-2].rotate_right(19)) ^ (schedule.arrays[i-2].clone() >> 10);
    schedule.arrays[i] = schedule.arrays[i-16].clone() + s0 + schedule.arrays[i-7].clone() + s1;
  }
  schedule
}

fn compression() {
  let a = H0;
  let b = H1;
  let c = H2;
  let d = H3;
  let e = H4;
  let f = H5;
  let g = H6;
  let h = H7;
  for i in 0..64 {
    
  }
}