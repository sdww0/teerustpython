use super::objiter;
use super::objtype::PyClassRef;
use crate::function::Args;
use crate::pyobject::{PyClassImpl, PyContext, PyObjectRef, PyRef, PyResult, PyValue};
use crate::vm::VirtualMachine;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
pub type PyZipRef = PyRef<PyZip>;

#[pyclass]
#[derive(Debug)]
pub struct PyZip {
    iterators: Vec<PyObjectRef>,
}

impl PyValue for PyZip {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.zip_type()
    }
}

#[pyimpl(flags(BASETYPE))]
impl PyZip {
    #[pyslot]
    fn tp_new(cls: PyClassRef, iterables: Args, vm: &VirtualMachine) -> PyResult<PyZipRef> {
        let iterators = iterables
            .into_iter()
            .map(|iterable| objiter::get_iter(vm, &iterable))
            .collect::<Result<Vec<_>, _>>()?;
        PyZip { iterators }.into_ref_with_type(vm, cls)
    }

    #[pymethod(name = "__next__")]
    fn next(&self, vm: &VirtualMachine) -> PyResult {
        if self.iterators.is_empty() {
            Err(objiter::new_stop_iteration(vm))
        } else {
            let next_objs = self
                .iterators
                .iter()
                .map(|iterator| objiter::call_next(vm, iterator))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(vm.ctx.new_tuple(next_objs))
        }
    }

    #[pymethod(name = "__iter__")]
    fn iter(zelf: PyRef<Self>) -> PyRef<Self> {
        zelf
    }
}

pub fn init(context: &PyContext) {
    PyZip::extend_class(context, &context.types.zip_type);
}
