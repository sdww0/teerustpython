use std::prelude::v1::*;
use parity_wasm::elements::{
	deserialize_buffer, deserialize_file, ValueType, InitExpr, DataSegment,
	serialize, ElementSegment, Instructions, BlockType, Local, FuncBody,
};
use parity_wasm::elements::{Section, TypeSection, Type, DataSection, ElementSection, CodeSection};

//#[test]
pub fn import_section() {
	let module = deserialize_file("./res/cases/v1/test5.wasm").expect("Should be deserialized");
	let mut found = false;
	for section in module.sections() {
		match section {
			&Section::Import(ref import_section) => {
				assert_eq!(25, import_section.entries().len());
				found = true
			},
			_ => { }
		}
	}
	assert!(found, "There should be import section in test5.wasm");
}

fn functions_test_payload() -> &'static [u8] {
	&[
		// functions section id
		0x03u8,
		// functions section length
		0x87, 0x80, 0x80, 0x80, 0x0,
		// number of functions
		0x04,
		// type reference 1
		0x01,
		// type reference 2
		0x86, 0x80, 0x00,
		// type reference 3
		0x09,
		// type reference 4
		0x33
	]
}

//#[test]
pub fn fn_section_detect() {
	let section: Section =
		deserialize_buffer(functions_test_payload()).expect("section to be deserialized");

	match section {
		Section::Function(_) => {},
		_ => {
			panic!("Payload should be recognized as functions section")
		}
	}
}

//#[test]
pub fn fn_section_number() {
	let section: Section =
		deserialize_buffer(functions_test_payload()).expect("section to be deserialized");

	match section {
		Section::Function(fn_section) => {
			assert_eq!(4, fn_section.entries().len(), "There should be 4 functions total");
		},
		_ => {
			// will be catched by dedicated test
		}
	}
}

//#[test]
pub fn fn_section_ref() {
	let section: Section =
		deserialize_buffer(functions_test_payload()).expect("section to be deserialized");

	match section {
		Section::Function(fn_section) => {
			assert_eq!(6, fn_section.entries()[1].type_ref());
		},
		_ => {
			// will be catched by dedicated test
		}
	}
}

fn types_test_payload() -> &'static [u8] {
	&[
		// section length
		11,

		// 2 functions
		2,
		// func 1, form =1
		0x60,
		// param_count=1
		1,
			// first param
			0x7e, // i64
		// no return params
		0x00,

		// func 2, form=1
		0x60,
		// param_count=2
		2,
			// first param
			0x7e,
			// second param
			0x7d,
		// return param (is_present, param_type)
		0x01, 0x7e
	]
}

//#[test]
pub fn type_section_len() {
	let type_section: TypeSection =
		deserialize_buffer(types_test_payload()).expect("type_section be deserialized");

	assert_eq!(type_section.types().len(), 2);
}

//#[test]
pub fn type_section_infer() {
	let type_section: TypeSection =
		deserialize_buffer(types_test_payload()).expect("type_section be deserialized");

	let t1 = match &type_section.types()[1] {
		&Type::Function(ref func_type) => func_type
	};

	assert_eq!(Some(ValueType::I64), t1.return_type());
	assert_eq!(2, t1.params().len());
}

fn export_payload() -> &'static [u8] {
	&[
		// section id
		0x07,
		// section length
		28,
		// 6 entries
		6,
		// func "A", index 6
		// [name_len(1-5 bytes), name_bytes(name_len, internal_kind(1byte), internal_index(1-5 bytes)])
		0x01, 0x41,  0x01, 0x86, 0x80, 0x00,
		// func "B", index 8
		0x01, 0x42,  0x01, 0x86, 0x00,
		// func "C", index 7
		0x01, 0x43,  0x01, 0x07,
		// memory "D", index 0
		0x01, 0x44,  0x02, 0x00,
		// func "E", index 1
		0x01, 0x45,  0x01, 0x01,
		// func "F", index 2
		0x01, 0x46,  0x01, 0x02
	]
}


//#[test]
pub fn export_detect() {
	let section: Section =
		deserialize_buffer(export_payload()).expect("section to be deserialized");

	match section {
		Section::Export(_) => {},
		_ => {
			panic!("Payload should be recognized as export section")
		}
	}
}

fn code_payload() -> &'static [u8] {
	&[
		// sectionid
		0x0Au8,
		// section length, 32
		0x20,
		// body count
		0x01,
		// body 1, length 30
		0x1E,
		0x01, 0x01, 0x7F, // local i32 (one collection of length one of type i32)
		0x02, 0x7F, // block i32
			0x23, 0x00, // get_global 0
			0x21, 0x01, // set_local 1
			0x23, 0x00, // get_global 0
			0x20, 0x00, // get_local 0
			0x6A,       // i32.add
			0x24, 0x00, // set_global 0
			0x23, 0x00, // get_global 0
			0x41, 0x0F, // i32.const 15
			0x6A,       // i32.add
			0x41, 0x70, // i32.const -16
			0x71,       // i32.and
			0x24, 0x00, // set_global 0
			0x20, 0x01, // get_local 1
		0x0B,
		0x0B,
	]
}

//#[test]
pub fn code_detect() {

	let section: Section =
		deserialize_buffer(code_payload()).expect("section to be deserialized");

	match section {
		Section::Code(_) => {},
		_ => {
			panic!("Payload should be recognized as a code section")
		}
	}
}

fn data_payload() -> &'static [u8] {
	&[
		0x0bu8,  // section id
		20,      // 20 bytes overall
		0x01,    // number of segments
		0x00,    // index
		0x0b,    // just `end` op
		0x10,
		// 16x 0x00
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00
	]
}

//#[test]
pub fn data_section_ser() {
	let data_section = DataSection::with_entries(
		vec![DataSegment::new(0u32, Some(InitExpr::empty()), vec![0u8; 16])]
	);

	let buf = serialize(data_section).expect("Data section to be serialized");

	assert_eq!(buf, vec![
		20u8, // 19 bytes overall
		0x01, // number of segments
		0x00, // index
		0x0b, // just `end` op
		16,   // value of length 16
		0x00, 0x00, 0x00, 0x00, // 16x 0x00 as in initialization
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00,
		0x00, 0x00, 0x00, 0x00
	]);
}

//#[test]
pub fn data_section_detect() {
	let section: Section =
		deserialize_buffer(data_payload()).expect("section to be deserialized");

	match section {
		Section::Data(_) => {},
		_ => {
			panic!("Payload should be recognized as a data section")
		}
	}
}

//#[test]
pub fn element_section_ser() {
	let element_section = ElementSection::with_entries(
		vec![ElementSegment::new(0u32, Some(InitExpr::empty()), vec![0u32; 4])]
	);

	let buf = serialize(element_section).expect("Element section to be serialized");

	assert_eq!(buf, vec![
		08u8, // 8 bytes overall
		0x01, // number of segments
		0x00, // index
		0x0b, // just `end` op
		0x04, // 4 elements
		0x00, 0x00, 0x00, 0x00 // 4x 0x00 as in initialization
	]);
}

//#[test]
pub fn code_section_ser() {
	use parity_wasm::elements::Instruction::*;

	let code_section = CodeSection::with_bodies(
		vec![
			FuncBody::new(
				vec![Local::new(1, ValueType::I32)],
				Instructions::new(vec![
					Block(BlockType::Value(ValueType::I32)),
					GetGlobal(0),
					End,
					End,
				])
			)
		]);

	let buf = serialize(code_section).expect("Code section to be serialized");

	assert_eq!(buf, vec![
		11u8,            // 11 bytes total section size
		0x01,            // 1 function
		  9,             //   function #1 total code size
		  1,             //   1 local variable declaration
		  1,             //      amount of variables
		  0x7f,          //      type of variable (7-bit, -0x01), negative
		  0x02,          //   block
			0x7f,        //      block return type (7-bit, -0x01), negative
			0x23, 0x00,  //      get_global(0)
			0x0b,        //   block end
		0x0b,            // function end
	]);
}

//#[test]
pub fn start_section() {
	let section: Section = deserialize_buffer(&[08u8, 01u8, 00u8]).expect("Start section to deserialize");
	if let Section::Start(_) = section {
	} else {
		panic!("Payload should be a start section");
	}

	let serialized = serialize(section).expect("Start section to successfully serializen");

	assert_eq!(serialized, vec![08u8, 01u8, 00u8]);
}
