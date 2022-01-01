use parity_wasm::builder::global;
use parity_wasm::elements;

//#[test]
pub fn example() {
	let entry = global().value_type().i32().build();
	assert_eq!(entry.global_type().content_type(), elements::ValueType::I32);
	assert_eq!(entry.global_type().is_mutable(), false);
}
