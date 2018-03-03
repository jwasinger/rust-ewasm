#![no_std]

use blake2_rfc::blake2b::Blake2b;
use blake2_rfc::_selftest_seq as selftest_seq;

// bn128 point addition copied from https://github.com/ethereumjs/rustbn.js

extern "C" {
    fn callDataCopy(resultOffset: *const u8, dataOffset: u32, length: u32);
    fn storageStore(keyOffset: *const u32, valueOffset: *const u8);
}

extern crate blake2_rfc;

#[no_mangle]
pub fn main() {
	const ZEROS: [u8; 4096] = [0; 4096];

	let mut state = Blake2b::new(64);
	for _ in 0..1048576 {
			state.update(&ZEROS);
	}

  let result: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
	assert_eq!(state.finalize().as_bytes(), &result);
}
