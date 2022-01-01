use std::prelude::v1::*;

use parity_wasm::elements::{deserialize_buffer, Serialize};
use parity_wasm::elements::{CountedList, VarInt7, VarUint32, VarInt32, VarInt64, VarUint64};
use parity_wasm::elements::Error;

fn varuint32_ser_test(val: u32, expected: Vec<u8>) {
	let mut buf = Vec::new();
	let v1: VarUint32 = val.into();
	v1.serialize(&mut buf).expect("to be serialized ok");
	assert_eq!(expected, buf);
}

fn varuint32_de_test(dt: Vec<u8>, expected: u32) {
	let val: VarUint32 = deserialize_buffer(&dt).expect("buf to be serialized");
	assert_eq!(expected, val.into());
}

fn varuint32_serde_test(dt: Vec<u8>, val: u32) {
	varuint32_de_test(dt.clone(), val);
	varuint32_ser_test(val, dt);
}

fn varint32_ser_test(val: i32, expected: Vec<u8>) {
	let mut buf = Vec::new();
	let v1: VarInt32 = val.into();
	v1.serialize(&mut buf).expect("to be serialized ok");
	assert_eq!(expected, buf);
}

fn varint32_de_test(dt: Vec<u8>, expected: i32) {
	let val: VarInt32 = deserialize_buffer(&dt).expect("buf to be serialized");
	assert_eq!(expected, val.into());
}

fn varint32_serde_test(dt: Vec<u8>, val: i32) {
	varint32_de_test(dt.clone(), val);
	varint32_ser_test(val, dt);
}

fn varuint64_ser_test(val: u64, expected: Vec<u8>) {
	let mut buf = Vec::new();
	let v1: VarUint64 = val.into();
	v1.serialize(&mut buf).expect("to be serialized ok");
	assert_eq!(expected, buf);
}

fn varuint64_de_test(dt: Vec<u8>, expected: u64) {
	let val: VarUint64 = deserialize_buffer(&dt).expect("buf to be serialized");
	assert_eq!(expected, val.into());
}

fn varuint64_serde_test(dt: Vec<u8>, val: u64) {
	varuint64_de_test(dt.clone(), val);
	varuint64_ser_test(val, dt);
}

fn varint64_ser_test(val: i64, expected: Vec<u8>) {
	let mut buf = Vec::new();
	let v1: VarInt64 = val.into();
	v1.serialize(&mut buf).expect("to be serialized ok");
	assert_eq!(expected, buf);
}

fn varint64_de_test(dt: Vec<u8>, expected: i64) {
	let val: VarInt64 = deserialize_buffer(&dt).expect("buf to be serialized");
	assert_eq!(expected, val.into());
}

fn varint64_serde_test(dt: Vec<u8>, val: i64) {
	varint64_de_test(dt.clone(), val);
	varint64_ser_test(val, dt);
}

//#[test]
pub fn varuint32_0() {
	varuint32_serde_test(vec![0u8; 1], 0);
}

//#[test]
pub fn varuint32_1() {
	varuint32_serde_test(vec![1u8; 1], 1);
}

//#[test]
pub fn varuint32_135() {
	varuint32_serde_test(vec![135u8, 0x01], 135);
}

//#[test]
pub fn varuint32_8192() {
	varuint32_serde_test(vec![0x80, 0x40], 8192);
}

//#[test]
pub fn varint32_8192() {
	varint32_serde_test(vec![0x80, 0xc0, 0x00], 8192);
}

//#[test]
pub fn varint32_neg_8192() {
	varint32_serde_test(vec![0x80, 0x40], -8192);
}

//#[test]
pub fn varuint64_0() {
	varuint64_serde_test(vec![0u8; 1], 0);
}

//#[test]
pub fn varuint64_1() {
	varuint64_serde_test(vec![1u8; 1], 1);
}

//#[test]
pub fn varuint64_135() {
	varuint64_serde_test(vec![135u8, 0x01], 135);
}

//#[test]
pub fn varuint64_8192() {
	varuint64_serde_test(vec![0x80, 0x40], 8192);
}

//#[test]
pub fn varint64_8192() {
	varint64_serde_test(vec![0x80, 0xc0, 0x00], 8192);
}

//#[test]
pub fn varint64_neg_8192() {
	varint64_serde_test(vec![0x80, 0x40], -8192);
}

//#[test]
pub fn varint64_min() {
	varint64_serde_test(
		vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x7f],
		-9223372036854775808,
	);
}

//#[test]
pub fn varint64_bad_extended() {
	let res = deserialize_buffer::<VarInt64>(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x6f][..]);
	assert!(res.is_err());
}

//#[test]
pub fn varint32_bad_extended() {
	let res = deserialize_buffer::<VarInt32>(&[0x80, 0x80, 0x80, 0x80, 0x6f][..]);
	assert!(res.is_err());
}

//#[test]
pub fn varint32_bad_extended2() {
	let res = deserialize_buffer::<VarInt32>(&[0x80, 0x80, 0x80, 0x80, 0x41][..]);
	assert!(res.is_err());
}

//#[test]
pub fn varint64_max() {
	varint64_serde_test(
		vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00],
		9223372036854775807,
	);
}

//#[test]
pub fn varint64_too_long() {
	assert!(
		deserialize_buffer::<VarInt64>(
			&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00][..],
		).is_err()
	);
}

//#[test]
pub fn varint32_too_long() {
	assert!(
		deserialize_buffer::<VarInt32>(
			&[0xff, 0xff, 0xff, 0xff, 0xff, 0x00][..],
		).is_err()
	);
}

//#[test]
pub fn varuint64_too_long() {
	assert!(
		deserialize_buffer::<VarUint64>(
			&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00][..],
		).is_err()
	);
}

//#[test]
pub fn varuint32_too_long() {
	assert!(
		deserialize_buffer::<VarUint32>(
			&[0xff, 0xff, 0xff, 0xff, 0xff, 0x00][..],
		).is_err()
	);
}

//#[test]
pub fn varuint32_too_long_trailing() {
	assert!(
		deserialize_buffer::<VarUint32>(
			&[0xff, 0xff, 0xff, 0xff, 0x7f][..],
		).is_err()
	);
}

//#[test]
pub fn varuint64_too_long_trailing() {
	assert!(
		deserialize_buffer::<VarUint64>(
			&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x04][..],
		).is_err()
	);
}

//#[test]
pub fn varint32_min() {
	varint32_serde_test(
		vec![0x80, 0x80, 0x80, 0x80, 0x78],
		-2147483648,
	);
}

//#[test]
pub fn varint7_invalid() {
	match deserialize_buffer::<VarInt7>(&[240]) {
		Err(Error::InvalidVarInt7(_)) => {},
		_ => panic!("Should be invalid varint7 error!")
	}
}

//#[test]
pub fn varint7_neg() {
	assert_eq!(-0x10i8, deserialize_buffer::<VarInt7>(&[0x70]).expect("fail").into());
}

//#[test]
pub fn varuint32_too_long_nulled() {
	match deserialize_buffer::<VarUint32>(
		&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x78]
	) {
		Err(Error::InvalidVarUint32) => {},
		_ => panic!("Should be invalid varuint32"),
	}
}

//#[test]
pub fn varint32_max() {
	varint32_serde_test(
		vec![0xff, 0xff, 0xff, 0xff, 0x07],
		2147483647,
	);
}


//#[test]
pub fn counted_list() {
	let payload = [
		133u8, //(128+5), length is 5
			0x80, 0x80, 0x80, 0x0, // padding
		0x01,
		0x7d,
		0x05,
		0x07,
		0x09,
	];

	let list: CountedList<VarInt7> =
		deserialize_buffer(&payload).expect("type_section be deserialized");

	let vars = list.into_inner();
	assert_eq!(5, vars.len());
	let v3: i8 = (*vars.get(1).unwrap()).into();
	assert_eq!(-0x03i8, v3);
}
