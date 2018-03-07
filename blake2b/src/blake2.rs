#![no_std]

use blake2_rfc::blake2b::Blake2b;
use blake2_rfc::_selftest_seq as selftest_seq;

// bn128 point addition copied from https://github.com/ethereumjs/rustbn.js

extern "C" {
  fn storageStore(keyOffset: *const u32, valueOffset: *const u32);
  fn printMemHex(start:  *const u8, len: u32);
}

extern crate blake2_rfc;

fn as_u32_be(array: &[u8; 4]) -> u32 {
	((array[0] as u32) << 24) +
	((array[1] as u32) << 16) +
	((array[2] as u32) <<  8) +
	((array[3] as u32) <<  0)
}

#[no_mangle]
pub fn main() {
	const ZEROS: [u8; 256] = [0; 256];

  let mut dst = [0, 0, 0, 0];
  let sstore_key: [u32;8] = [0;8];
  let raw_sstore_key = &sstore_key as *const u32;
  let vl: u32 = 0;

  unsafe {
     storageStore(raw_sstore_key, &vl);
  }

	let mut state = Blake2b::new(64);
	for _ in 0..4 {
    state.update(&ZEROS);
	}

  dst.copy_from_slice(&state.finalize().as_bytes()[0..4]);
  let a: [u8; 4] = [dst[0], dst[1], dst[2], dst[3]];
  let value: u32 = as_u32_be(&a);

  let result: [u8; 4] = [0xb4, 0xb7, 0x2b, 0x45];

  unsafe {
    printMemHex(&a as *const u8, 4);
    //printMemHex(0, 4);
  }
	assert_eq!(result[0], a[0]);
	assert_eq!(result[1], a[1]);
	assert_eq!(result[2], a[2]);
	assert_eq!(result[3], a[3]);

  unsafe {
     storageStore(raw_sstore_key, &value);
  }
}
