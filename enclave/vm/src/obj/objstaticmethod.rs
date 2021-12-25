use super::objtype::PyClassRef;
use crate::function::OptionalArg;
use crate::pyobject::{PyClassImpl, PyContext, PyObjectRef, PyRef, PyResult, PyValue};
use crate::slots::SlotDescriptor;
use crate::vm::VirtualMachine;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
#[pyclass(name = "staticmethod")]
#[derive(Clone, Debug)]
pub struct PyStaticMethod {
    pub callable: PyObjectRef,
}
pub type PyStaticMethodRef = PyRef<PyStaticMethod>;

impl PyValue for PyStaticMethod {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.staticmethod_type()
    }
}

impl SlotDescriptor for PyStaticMethod {
    fn descr_get(
        vm: &VirtualMachine,
        zelf: PyObjectRef,
        _obj: Option<PyObjectRef>,
        _cls: OptionalArg<PyObjectRef>,
    ) -> PyResult {
        let zelf = Self::_zelf(zelf, vm)?;
        Ok(zelf.callable.clone())
    }
}

#[pyimpl(with(SlotDescriptor), flags(BASETYPE))]
impl PyStaticMethod {
    pub fn new(callable: PyObjectRef) -> Self {
        Self { callable }
    }

    #[pyslot]
    fn tp_new(
        cls: PyClassRef,
        callable: PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyStaticMethodRef> {
        PyStaticMethod {
            callable: callable.clone(),
        }
        .into_ref_with_type(vm, cls)
    }
}

pub fn init(context: &PyContext) {
    PyStaticMethod::extend_class(context, &context.types.staticmethod_type);
}
