use std::prelude::v1::*;
use std::panic;
use sgx_tunittest::*;
use parity_wasm::elements::{deserialize_file, serialize, deserialize_buffer, Section};
use parity_wasm::elements::Module;

//#[test]
pub fn hello() {
	let module = deserialize_file("./res/cases/v1/hello.wasm").expect("Should be deserialized");

	assert_eq!(module.version(), 1);
	assert_eq!(module.sections().len(), 8);
}

//#[test]
pub fn serde() {
	let module = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	let buf = serialize(module).expect("serialization to succeed");

	let module_new: Module = deserialize_buffer(&buf).expect("deserialization to succeed");
	let module_old = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");

	assert_eq!(module_old.sections().len(), module_new.sections().len());
}

//#[test]
pub fn serde_type() {
	let mut module = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	module.sections_mut().retain(|x| {
		if let &Section::Type(_) = x { true } else { false }
	});

	let buf = serialize(module).expect("serialization to succeed");

	let module_new: Module = deserialize_buffer(&buf).expect("deserialization to succeed");
	let module_old = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	assert_eq!(
		module_old.type_section().expect("type section exists").types().len(),
		module_new.type_section().expect("type section exists").types().len(),
		"There should be equal amount of types before and after serialization"
	);
}

//#[test]
pub fn serde_import() {
	let mut module = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	module.sections_mut().retain(|x| {
		if let &Section::Import(_) = x { true } else { false }
	});

	let buf = serialize(module).expect("serialization to succeed");

	let module_new: Module = deserialize_buffer(&buf).expect("deserialization to succeed");
	let module_old = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	assert_eq!(
		module_old.import_section().expect("import section exists").entries().len(),
		module_new.import_section().expect("import section exists").entries().len(),
		"There should be equal amount of import entries before and after serialization"
	);
}

//#[test]
pub fn serde_code() {
	let mut module = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	module.sections_mut().retain(|x| {
		if let &Section::Code(_) = x { return true }
		if let &Section::Function(_) = x { true } else { false }
	});

	let buf = serialize(module).expect("serialization to succeed");

	let module_new: Module = deserialize_buffer(&buf).expect("deserialization to succeed");
	let module_old = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	assert_eq!(
		module_old.code_section().expect("code section exists").bodies().len(),
		module_new.code_section().expect("code section exists").bodies().len(),
		"There should be equal amount of function bodies before and after serialization"
	);
}

//#[test]
pub fn const_() {
	use parity_wasm::elements::Instruction::*;

	let module = deserialize_file("./res/cases/v1/const.wasm").expect("Should be deserialized");
	let func = &module.code_section().expect("Code section to exist").bodies()[0];
	assert_eq!(func.code().elements().len(), 20);

	assert_eq!(I64Const(9223372036854775807), func.code().elements()[0]);
	assert_eq!(I64Const(-9223372036854775808), func.code().elements()[1]);
	assert_eq!(I64Const(-1152894205662152753), func.code().elements()[2]);
	assert_eq!(I64Const(-8192), func.code().elements()[3]);
	assert_eq!(I32Const(1024), func.code().elements()[4]);
	assert_eq!(I32Const(2048), func.code().elements()[5]);
	assert_eq!(I32Const(4096), func.code().elements()[6]);
	assert_eq!(I32Const(8192), func.code().elements()[7]);
	assert_eq!(I32Const(16384), func.code().elements()[8]);
	assert_eq!(I32Const(32767), func.code().elements()[9]);
	assert_eq!(I32Const(-1024), func.code().elements()[10]);
	assert_eq!(I32Const(-2048), func.code().elements()[11]);
	assert_eq!(I32Const(-4096), func.code().elements()[12]);
	assert_eq!(I32Const(-8192), func.code().elements()[13]);
	assert_eq!(I32Const(-16384), func.code().elements()[14]);
	assert_eq!(I32Const(-32768), func.code().elements()[15]);
	assert_eq!(I32Const(-2147483648), func.code().elements()[16]);
	assert_eq!(I32Const(2147483647), func.code().elements()[17]);
}

//#[test]
pub fn store() {
	use parity_wasm::elements::Instruction::*;

	let module = deserialize_file("./res/cases/v1/offset.wasm").expect("Should be deserialized");
	let func = &module.code_section().expect("Code section to exist").bodies()[0];

	assert_eq!(func.code().elements().len(), 5);
	assert_eq!(I64Store(0, 32), func.code().elements()[2]);
}

//#[test]
pub fn peek() {
	use parity_wasm::elements::peek_size;

	let module = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	let mut buf = serialize(module).expect("serialization to succeed");

	buf.extend_from_slice(&[1, 5, 12, 17]);

	assert_eq!(peek_size(&buf), buf.len() - 4);
}


//#[test]
pub fn peek_2() {
	use parity_wasm::elements::peek_size;

	let module = deserialize_file("./res/cases/v1/offset.wasm").expect("Should be deserialized");
	let mut buf = serialize(module).expect("serialization to succeed");

	buf.extend_from_slice(&[0, 0, 0, 0, 0, 1, 5, 12, 17]);

	assert_eq!(peek_size(&buf), buf.len() - 9);
}

//#[test]
pub fn peek_3() {
	use parity_wasm::elements::peek_size;

	let module = deserialize_file("./res/cases/v1/peek_sample.wasm").expect("Should be deserialized");
	let buf = serialize(module).expect("serialization to succeed");

	assert_eq!(peek_size(&buf), buf.len());
}

//#[test]
pub fn module_default_round_trip() {
	let module1 = Module::default();
	let buf = serialize(module1).expect("Serialization should succeed");

	let module2: Module = deserialize_buffer(&buf).expect("Deserialization should succeed");
	assert_eq!(Module::default().magic, module2.magic);
}

//#[test]
pub fn names() {
	let module = deserialize_file("./res/cases/v1/with_names.wasm")
		.expect("Should be deserialized")
		.parse_names()
		.expect("Names to be parsed");

	let mut found_section = false;
	for section in module.sections() {
		match *section {
			Section::Name(ref name_section) => {
				let function_name_subsection = name_section
					.functions()
					.expect("function_name_subsection should be present");
				assert_eq!(
					function_name_subsection.names().get(0).expect("Should be entry #0"),
					"elog"
				);
				assert_eq!(
					function_name_subsection.names().get(11).expect("Should be entry #0"),
					"_ZN48_$LT$pwasm_token_contract..Endpoint$LT$T$GT$$GT$3new17hc3ace6dea0978cd9E"
				);

				found_section = true;
			},
			_ => {},
		}
	}

	assert!(found_section, "Name section should be present in dedicated example");
}

//#[test]
//#[should_panic]
pub fn wrong_varuint1_case() {
    should_panic!(
	deserialize_file("./res/cases/v1/varuint1_1.wasm")
		.expect("Maybe shouldn't be deserialized"));
}


//#[test]
pub fn memory_space() {
	let module = deserialize_file("./res/cases/v1/two-mems.wasm").expect("failed to deserialize");
	assert_eq!(module.memory_space(), 2);
}
