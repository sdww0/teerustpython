use super::objtype::PyClassRef;
use crate::function::{OptionalArg, PyFuncArgs};
use crate::pyobject::{
    IdProtocol, PyClassImpl, PyContext, PyObject, PyObjectPayload, PyObjectRef, PyRef, PyResult,
    PyValue, TypeProtocol,
};
use crate::slots::SlotCall;
use crate::vm::VirtualMachine;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

use crate::pyhash::PyHash;
use crossbeam_utils::atomic::AtomicCell;
use std::sync::{Arc, Weak};

#[pyclass]
#[derive(Debug)]
pub struct PyWeak {
    referent: Weak<PyObject<dyn PyObjectPayload>>,
    hash: AtomicCell<Option<PyHash>>,
}

impl PyWeak {
    pub fn downgrade(obj: &PyObjectRef) -> PyWeak {
        PyWeak {
            referent: Arc::downgrade(obj),
            hash: AtomicCell::new(None),
        }
    }

    pub fn upgrade(&self) -> Option<PyObjectRef> {
        self.referent.upgrade()
    }
}

impl PyValue for PyWeak {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.weakref_type()
    }
}

pub type PyWeakRef = PyRef<PyWeak>;

impl SlotCall for PyWeak {
    fn call(&self, args: PyFuncArgs, vm: &VirtualMachine) -> PyResult {
        args.bind::<()>(vm)?;
        Ok(self.referent.upgrade().unwrap_or_else(|| vm.get_none()))
    }
}

#[pyimpl(with(SlotCall), flags(BASETYPE))]
impl PyWeak {
    // TODO callbacks
    #[pyslot]
    fn tp_new(
        cls: PyClassRef,
        referent: PyObjectRef,
        _callback: OptionalArg<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult<PyRef<Self>> {
        PyWeak::downgrade(&referent).into_ref_with_type(vm, cls)
    }

    #[pymethod(magic)]
    fn hash(&self, vm: &VirtualMachine) -> PyResult<PyHash> {
        match self.hash.load() {
            Some(hash) => Ok(hash),
            None => {
                let obj = self
                    .upgrade()
                    .ok_or_else(|| vm.new_type_error("weak object has gone away".to_owned()))?;
                let hash = vm._hash(&obj)?;
                self.hash.store(Some(hash));
                Ok(hash)
            }
        }
    }

    #[pymethod(magic)]
    fn eq(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult {
        if let Some(other) = other.payload_if_subclass::<Self>(vm) {
            self.upgrade()
                .and_then(|s| other.upgrade().map(|o| (s, o)))
                .map_or(Ok(false), |(a, b)| vm.bool_eq(a, b))
                .map(|b| vm.new_bool(b))
        } else {
            Ok(vm.ctx.not_implemented())
        }
    }

    #[pymethod(magic)]
    fn repr(zelf: PyRef<Self>) -> String {
        let id = zelf.get_id();
        if let Some(o) = zelf.upgrade() {
            format!(
                "<weakref at {}; to '{}' at {}>",
                id,
                o.class().name,
                o.get_id(),
            )
        } else {
            format!("<weakref at {}; dead>", id)
        }
    }
}

pub fn init(context: &PyContext) {
    PyWeak::extend_class(context, &context.types.weakref_type);
}
