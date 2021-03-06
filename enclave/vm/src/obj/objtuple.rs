use crossbeam_utils::atomic::AtomicCell;
use std::fmt;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

use super::objiter;
use super::objsequence::get_item;
use super::objtype::PyClassRef;
use crate::function::OptionalArg;
use crate::pyhash;
use crate::pyobject::{
    IntoPyObject,
    PyArithmaticValue::{self, *},
    PyClassImpl, PyComparisonValue, PyContext, PyObjectRef, PyRef, PyResult, PyValue,
};
use crate::sequence;
use crate::vm::{ReprGuard, VirtualMachine};

/// tuple() -> empty tuple
/// tuple(iterable) -> tuple initialized from iterable's items
///
/// If the argument is a tuple, the return value is the same object.
#[pyclass]
pub struct PyTuple {
    elements: Vec<PyObjectRef>,
}

impl fmt::Debug for PyTuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: implement more informational, non-recursive Debug formatter
        f.write_str("tuple")
    }
}

impl From<Vec<PyObjectRef>> for PyTuple {
    fn from(elements: Vec<PyObjectRef>) -> Self {
        PyTuple { elements }
    }
}

impl PyValue for PyTuple {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.tuple_type()
    }
}

macro_rules! impl_intopyobj_tuple {
    ($(($T:ident, $idx:tt)),+) => {
        impl<$($T: IntoPyObject),*> IntoPyObject for ($($T,)*) {
            fn into_pyobject(self, vm: &VirtualMachine) -> PyResult {
                Ok(vm.ctx.new_tuple(vec![$(self.$idx.into_pyobject(vm)?),*]))
            }
        }
    };
}

impl_intopyobj_tuple!((A, 0));
impl_intopyobj_tuple!((A, 0), (B, 1));
impl_intopyobj_tuple!((A, 0), (B, 1), (C, 2));
impl_intopyobj_tuple!((A, 0), (B, 1), (C, 2), (D, 3));
impl_intopyobj_tuple!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
impl_intopyobj_tuple!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
impl_intopyobj_tuple!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));

impl PyTuple {
    pub(crate) fn fast_getitem(&self, idx: usize) -> PyObjectRef {
        self.elements[idx].clone()
    }

    pub fn as_slice(&self) -> &[PyObjectRef] {
        &self.elements
    }
}

pub type PyTupleRef = PyRef<PyTuple>;

pub(crate) fn get_value(obj: &PyObjectRef) -> &[PyObjectRef] {
    obj.payload::<PyTuple>().unwrap().as_slice()
}

#[pyimpl(flags(BASETYPE))]
impl PyTuple {
    #[inline]
    fn cmp<F>(&self, other: PyObjectRef, op: F, vm: &VirtualMachine) -> PyResult<PyComparisonValue>
    where
        F: Fn(&Vec<PyObjectRef>, &Vec<PyObjectRef>) -> PyResult<bool>,
    {
        let r = if let Some(other) = other.payload_if_subclass::<PyTuple>(vm) {
            Implemented(op(&self.elements, &other.elements)?)
        } else {
            NotImplemented
        };
        Ok(r)
    }

    #[pymethod(name = "__lt__")]
    fn lt(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyComparisonValue> {
        self.cmp(other, |a, b| sequence::lt(vm, a, b), vm)
    }

    #[pymethod(name = "__gt__")]
    fn gt(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyComparisonValue> {
        self.cmp(other, |a, b| sequence::gt(vm, a, b), vm)
    }

    #[pymethod(name = "__ge__")]
    fn ge(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyComparisonValue> {
        self.cmp(other, |a, b| sequence::ge(vm, a, b), vm)
    }

    #[pymethod(name = "__le__")]
    fn le(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyComparisonValue> {
        self.cmp(other, |a, b| sequence::le(vm, a, b), vm)
    }

    #[pymethod(name = "__add__")]
    fn add(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyArithmaticValue<PyTuple> {
        if let Some(other) = other.payload_if_subclass::<PyTuple>(vm) {
            let elements: Vec<_> = self
                .elements
                .iter()
                .chain(other.as_slice().iter())
                .cloned()
                .collect();
            Implemented(elements.into())
        } else {
            NotImplemented
        }
    }

    #[pymethod(name = "__bool__")]
    fn bool(&self) -> bool {
        !self.elements.is_empty()
    }

    #[pymethod(name = "count")]
    fn count(&self, needle: PyObjectRef, vm: &VirtualMachine) -> PyResult<usize> {
        let mut count: usize = 0;
        for element in self.elements.iter() {
            if vm.identical_or_equal(element, &needle)? {
                count += 1;
            }
        }
        Ok(count)
    }

    #[pymethod(name = "__eq__")]
    fn eq(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyComparisonValue> {
        self.cmp(other, |a, b| sequence::eq(vm, a, b), vm)
    }

    #[pymethod(name = "__ne__")]
    fn ne(&self, other: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyComparisonValue> {
        Ok(self.eq(other, vm)?.map(|v| !v))
    }

    #[pymethod(name = "__hash__")]
    fn hash(&self, vm: &VirtualMachine) -> PyResult<pyhash::PyHash> {
        pyhash::hash_iter(self.elements.iter(), vm)
    }

    #[pymethod(name = "__iter__")]
    fn iter(zelf: PyRef<Self>) -> PyTupleIterator {
        PyTupleIterator {
            position: AtomicCell::new(0),
            tuple: zelf,
        }
    }

    #[pymethod(name = "__len__")]
    fn len(&self) -> usize {
        self.elements.len()
    }

    #[pymethod(name = "__repr__")]
    fn repr(zelf: PyRef<Self>, vm: &VirtualMachine) -> PyResult<String> {
        let s = if let Some(_guard) = ReprGuard::enter(zelf.as_object()) {
            let mut str_parts = Vec::with_capacity(zelf.elements.len());
            for elem in zelf.elements.iter() {
                let s = vm.to_repr(elem)?;
                str_parts.push(s.as_str().to_owned());
            }

            if str_parts.len() == 1 {
                format!("({},)", str_parts[0])
            } else {
                format!("({})", str_parts.join(", "))
            }
        } else {
            "(...)".to_owned()
        };
        Ok(s)
    }

    #[pymethod(name = "__mul__")]
    #[pymethod(name = "__rmul__")]
    fn mul(&self, counter: isize) -> PyTuple {
        let new_elements: Vec<_> = sequence::seq_mul(&self.elements, counter)
            .cloned()
            .collect();
        new_elements.into()
    }

    #[pymethod(name = "__getitem__")]
    fn getitem(zelf: PyRef<Self>, needle: PyObjectRef, vm: &VirtualMachine) -> PyResult {
        get_item(vm, zelf.as_object(), &zelf.elements, needle.clone())
    }

    #[pymethod(name = "index")]
    fn index(&self, needle: PyObjectRef, vm: &VirtualMachine) -> PyResult<usize> {
        for (index, element) in self.elements.iter().enumerate() {
            if vm.identical_or_equal(element, &needle)? {
                return Ok(index);
            }
        }
        Err(vm.new_value_error("tuple.index(x): x not in tuple".to_owned()))
    }

    #[pymethod(name = "__contains__")]
    fn contains(&self, needle: PyObjectRef, vm: &VirtualMachine) -> PyResult<bool> {
        for element in self.elements.iter() {
            if vm.identical_or_equal(element, &needle)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    #[pyslot]
    fn tp_new(
        cls: PyClassRef,
        iterable: OptionalArg<PyObjectRef>,
        vm: &VirtualMachine,
    ) -> PyResult<PyTupleRef> {
        let elements = if let OptionalArg::Present(iterable) = iterable {
            vm.extract_elements(&iterable)?
        } else {
            vec![]
        };

        PyTuple::from(elements).into_ref_with_type(vm, cls)
    }
}

#[pyclass]
#[derive(Debug)]
pub struct PyTupleIterator {
    position: AtomicCell<usize>,
    tuple: PyTupleRef,
}

impl PyValue for PyTupleIterator {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.tupleiterator_type()
    }
}

#[pyimpl]
impl PyTupleIterator {
    #[pymethod(name = "__next__")]
    fn next(&self, vm: &VirtualMachine) -> PyResult {
        let pos = self.position.fetch_add(1);
        if let Some(obj) = self.tuple.as_slice().get(pos) {
            Ok(obj.clone())
        } else {
            Err(objiter::new_stop_iteration(vm))
        }
    }

    #[pymethod(name = "__iter__")]
    fn iter(zelf: PyRef<Self>) -> PyRef<Self> {
        zelf
    }
}

pub fn init(context: &PyContext) {
    let tuple_type = &context.types.tuple_type;
    PyTuple::extend_class(context, tuple_type);

    PyTupleIterator::extend_class(context, &context.types.tupleiterator_type);
}
