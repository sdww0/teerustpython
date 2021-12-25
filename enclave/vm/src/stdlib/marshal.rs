use crate::bytecode;
use crate::obj::objbytes::{PyBytes, PyBytesRef};
use crate::obj::objcode::{PyCode, PyCodeRef};
use crate::pyobject::{PyObjectRef, PyResult};
use crate::vm::VirtualMachine;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

fn marshal_dumps(co: PyCodeRef) -> PyBytes {
    PyBytes::new(co.code.to_bytes())
}

fn marshal_loads(code_bytes: PyBytesRef, vm: &VirtualMachine) -> PyResult<PyCode> {
    let code = bytecode::CodeObject::from_bytes(&code_bytes)
        .map_err(|_| vm.new_value_error("Couldn't deserialize python bytecode".to_owned()))?;
    Ok(PyCode { code })
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;

    py_module!(vm, "marshal", {
        "loads" => ctx.new_function(marshal_loads),
        "dumps" => ctx.new_function(marshal_dumps),
    })
}
