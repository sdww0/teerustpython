use parity_wasm::elements;
use parity_wasm::builder::module;

//#[test]
pub fn smoky() {
	let module = module().build();
	assert_eq!(module.sections().len(), 0);
}

//#[test]
pub fn functions() {
	let module = module()
		.function()
			.signature().param().i32().build()
			.body().build()
			.build()
		.build();

	assert_eq!(module.type_section().expect("type section to exist").types().len(), 1);
	assert_eq!(module.function_section().expect("function section to exist").entries().len(), 1);
	assert_eq!(module.code_section().expect("code section to exist").bodies().len(), 1);
}

//#[test]
pub fn export() {
	let module = module()
		.export().field("call").internal().func(0).build()
		.build();

	assert_eq!(module.export_section().expect("export section to exist").entries().len(), 1);
}

//#[test]
pub fn global() {
	let module = module()
		.global().value_type().i64().mutable().init_expr(elements::Instruction::I64Const(5)).build()
		.build();

	assert_eq!(module.global_section().expect("global section to exist").entries().len(), 1);
}

//#[test]
pub fn data() {
	let module = module()
		.data()
			.offset(elements::Instruction::I32Const(16))
			.value(vec![0u8, 15, 10, 5, 25])
			.build()
		.build();

	assert_eq!(module.data_section().expect("data section to exist").entries().len(), 1);
}

//#[test]
pub fn reuse_types() {
	let module = module()
		.function()
			.signature().param().i32().build()
			.body().build()
			.build()
		.function()
			.signature().param().i32().build()
			.body().build()
			.build()
		.build();

	assert_eq!(module.type_section().expect("type section failed").types().len(), 1);
}
