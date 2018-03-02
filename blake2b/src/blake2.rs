//#![no_std]

use blake2_rfc::blake2b::Blake2b;
use blake2_rfc::_selftest_seq as selftest_seq;

use std::iter::repeat;
use std::vec::Vec;


extern crate data_encoding;
use self::data_encoding::HEXUPPER;

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
	assert_eq!(&state.finalize(), &HEXUPPER.decode(
			b"745572CA5756F9104329ED543735FC11904F0C18C4DF8ADF930F22D07F3094919A519FF34FD240AE3F5D5B4C8042225C109FB951036FDC99E7D2CD0C1D36B267")
			.unwrap()[..]);
}
