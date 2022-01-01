use parity_wasm::elements::{Section, deserialize_file};
use parity_wasm::elements::RelocationEntry;

//#[test]
pub fn reloc_section() {
	let module =
		deserialize_file("./res/cases/v1/relocatable.wasm").expect("Module should be deserialized")
		.parse_reloc().expect("Reloc section should be deserialized");
	let mut found = false;
	for section in module.sections() {
		match *section {
			Section::Reloc(ref reloc_section) => {
				assert_eq!(vec![
					RelocationEntry::MemoryAddressSleb { offset: 4, index: 0, addend: 0 },
					RelocationEntry::FunctionIndexLeb { offset: 12, index: 0 },
				], reloc_section.entries());
				found = true
			},
			_ => { }
		}
	}
	assert!(found, "There should be a reloc section in relocatable.wasm");
}
