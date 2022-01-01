use parity_wasm::builder::import;

//#[test]
pub fn example() {
	let entry = import().module("env").field("memory").external().memory(256, Some(256)).build();

	assert_eq!(entry.module(), "env");
	assert_eq!(entry.field(), "memory");
}
