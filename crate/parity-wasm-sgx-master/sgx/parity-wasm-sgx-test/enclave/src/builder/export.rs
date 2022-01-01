use parity_wasm::builder::export;

//#[test]
pub fn example() {
	let entry = export().field("memory").internal().memory(0).build();
	assert_eq!(entry.field(), "memory");
}
