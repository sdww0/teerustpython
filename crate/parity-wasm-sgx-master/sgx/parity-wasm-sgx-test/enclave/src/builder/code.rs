use parity_wasm::builder::{signatures, function};
use parity_wasm::elements;

//#[test]
pub fn example() {
	let result = signatures()
		.type_ref().val(1).build()
		.build();

	assert_eq!(result.entries().len(), 1);

	let result = signatures()
		.signature()
			.param().i32()
			.param().i32()
			.return_type().i64()
			.build()
		.bind();

	assert_eq!(result.len(), 1);
}

//#[test]
pub fn func_example() {
	let func = function()
		.signature()
			.param().i32()
			.return_type().i32()
			.build()
		.body()
			.with_instructions(elements::Instructions::empty())
			.build()
		.build();

	assert_eq!(func.code.locals().len(), 0);
	assert_eq!(func.code.code().elements().len(), 1);
}
