use std::prelude::v1::*;
use parity_wasm::elements::*;

// A helper function for the tests. Serialize a section, deserialize it,
// and make sure it matches the original.
fn serialize_test(original: NameSection) -> Vec<u8> {
	let mut buffer = vec![];
	original
		.serialize(&mut buffer)
		.expect("serialize error");
	buffer
	// todo: add deserialization to this test
}

//#[test]
pub fn serialize_module_name() {
	let module_name_subsection = ModuleNameSubsection::new("my_mod");
	let original = NameSection::new(Some(module_name_subsection), None, None);
	serialize_test(original.clone());
}

//#[test]
pub fn serialize_function_names() {
	let mut function_name_subsection = FunctionNameSubsection::default();
	function_name_subsection.names_mut().insert(0, "hello_world".to_string());
	let name_section = NameSection::new(None, Some(function_name_subsection), None);
	serialize_test(name_section);
}

//#[test]
pub fn serialize_local_names() {
	let mut local_name_subsection = LocalNameSubsection::default();
	let mut locals = NameMap::default();
	locals.insert(0, "msg".to_string());
	local_name_subsection.local_names_mut().insert(0, locals);

	let name_section = NameSection::new(None, None, Some(local_name_subsection));
	serialize_test(name_section);
}

//#[test]
pub fn serialize_all_subsections() {
	let module_name_subsection = ModuleNameSubsection::new("ModuleNameSubsection");

	let mut function_name_subsection = FunctionNameSubsection::default();
	function_name_subsection.names_mut().insert(0, "foo".to_string());
	function_name_subsection.names_mut().insert(1, "bar".to_string());

	let mut local_name_subsection = LocalNameSubsection::default();
	let mut locals = NameMap::default();
	locals.insert(0, "msg1".to_string());
	locals.insert(1, "msg2".to_string());
	local_name_subsection.local_names_mut().insert(0, locals);

	let name_section = NameSection::new(Some(module_name_subsection), Some(function_name_subsection), Some(local_name_subsection));
	serialize_test(name_section);
}

//#[test]
pub fn deserialize_local_names() {
	let module = deserialize_file("./res/cases/v1/names_with_imports.wasm")
		.expect("Should be deserialized")
		.parse_names()
		.expect("Names to be parsed");

	let name_section = module.names_section().expect("name_section should be present");
	let local_names = name_section.locals().expect("local_name_section should be present");

	let locals = local_names.local_names().get(0).expect("entry #0 should be present");
	assert_eq!(
		locals.get(0).expect("entry #0 should be present"),
		"abc"
	);

	let locals = local_names.local_names().get(1).expect("entry #1 should be present");
	assert_eq!(
		locals.get(0).expect("entry #0 should be present"),
		"def"
	);
}
