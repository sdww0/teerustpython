use std::prelude::v1::*;

use parity_wasm::elements::{Instruction, Instructions};

//#[test]
pub fn ifelse() {
	// see if-else.wast/if-else.wasm
	let instruction_list = parity_wasm::deserialize_buffer::<Instructions>(&[0x04, 0x7F, 0x41, 0x05, 0x05, 0x41, 0x07, 0x0B, 0x0B])
		.expect("valid hex of if instruction");
	let instructions = instruction_list.elements();
	match &instructions[0] {
		&Instruction::If(_) => (),
		_ => panic!("Should be deserialized as if instruction"),
	}
	let before_else = instructions.iter().skip(1)
		.take_while(|op| match **op { Instruction::Else => false, _ => true }).count();
	let after_else = instructions.iter().skip(1)
		.skip_while(|op| match **op { Instruction::Else => false, _ => true })
		.take_while(|op| match **op { Instruction::End => false, _ => true })
		.count()
		- 1; // minus Instruction::Else itself
	assert_eq!(before_else, after_else);
}

//#[test]
pub fn display() {
	let instruction = Instruction::GetLocal(0);
	assert_eq!("get_local 0", format!("{}", instruction));

	let instruction = Instruction::F64Store(0, 24);
	assert_eq!("f64.store offset=24", format!("{}", instruction));

	let instruction = Instruction::I64Store(0, 0);
	assert_eq!("i64.store", format!("{}", instruction));
}

//#[test]
pub fn size_off() {
	assert!(::std::mem::size_of::<Instruction>() <= 24);
}
