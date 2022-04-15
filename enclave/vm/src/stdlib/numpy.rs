use crate::obj::objiter;
use crate::obj::objstr::PyStringRef;
use crate::function::{Args, KwArgs, OptionalArg, PyFuncArgs};

use crate::obj::{objbool, objtype::PyClassRef};
use crate::pyobject::{IdProtocol, PyClassImpl, PyObjectRef, PyRef, PyResult, PyValue};
use crate::VirtualMachine;
use crate::obj::objint::{PyInt, PyIntRef};

use num_bigint::BigInt;
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

#[pyclass(name = "ndarray")]
pub struct ndarray {

}
#[pyimpl]
impl ndarray{


    
}

fn array1(size: OptionalArg<usize>,vm: &VirtualMachine) -> PyInt{
    PyInt::new(1)
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;
    py_module!(vm, "numpy", {
        "array" => ctx.new_function(array1)

    })
}
