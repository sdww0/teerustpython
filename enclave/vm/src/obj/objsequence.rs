use std::ops::Range;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Signed, ToPrimitive, Zero};

use super::objint::{PyInt, PyIntRef};
use super::objlist::PyList;
use super::objnone::PyNone;
use super::objslice::{PySlice, PySliceRef};
use super::objtuple::PyTuple;
use crate::function::OptionalArg;
use crate::pyobject::{PyObject, PyObjectRef, PyResult, TryFromObject, TypeProtocol};
use crate::vm::VirtualMachine;

pub trait PySliceableSequence {
    type Sliced;

    fn do_slice(&self, range: Range<usize>) -> Self::Sliced;
    fn do_slice_reverse(&self, range: Range<usize>) -> Self::Sliced;
    fn do_stepped_slice(&self, range: Range<usize>, step: usize) -> Self::Sliced;
    fn do_stepped_slice_reverse(&self, range: Range<usize>, step: usize) -> Self::Sliced;
    fn empty() -> Self::Sliced;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn get_pos(&self, p: isize) -> Option<usize> {
        get_pos(p, self.len())
    }

    fn get_slice_pos(&self, slice_pos: &BigInt) -> usize {
        get_slice_pos(slice_pos, self.len())
    }

    fn get_slice_range(&self, start: &Option<BigInt>, stop: &Option<BigInt>) -> Range<usize> {
        let start = start.as_ref().map(|x| self.get_slice_pos(x)).unwrap_or(0);
        let stop = stop
            .as_ref()
            .map(|x| self.get_slice_pos(x))
            .unwrap_or_else(|| self.len());

        start..stop
    }

    fn get_slice_items(&self, vm: &VirtualMachine, slice: &PySlice) -> PyResult<Self::Sliced> {
        let start = slice.start_index(vm)?;
        let stop = slice.stop_index(vm)?;
        let step = slice.step_index(vm)?.unwrap_or_else(BigInt::one);
        if step.is_zero() {
            Err(vm.new_value_error("slice step cannot be zero".to_owned()))
        } else if step.is_positive() {
            let range = self.get_slice_range(&start, &stop);
            if range.start < range.end {
                #[allow(clippy::range_plus_one)]
                match step.to_i32() {
                    Some(1) => Ok(self.do_slice(range)),
                    Some(num) => Ok(self.do_stepped_slice(range, num as usize)),
                    None => Ok(self.do_slice(range.start..range.start + 1)),
                }
            } else {
                Ok(Self::empty())
            }
        } else {
            // calculate the range for the reverse slice, first the bounds needs to be made
            // exclusive around stop, the lower number
            let start = start.as_ref().map(|x| {
                if *x == (-1).to_bigint().unwrap() {
                    self.len() + BigInt::one() //.to_bigint().unwrap()
                } else {
                    x + 1
                }
            });
            let stop = stop.as_ref().map(|x| {
                if *x == (-1).to_bigint().unwrap() {
                    self.len().to_bigint().unwrap()
                } else {
                    x + 1
                }
            });
            let range = self.get_slice_range(&stop, &start);
            if range.start < range.end {
                match (-step).to_i32() {
                    Some(1) => Ok(self.do_slice_reverse(range)),
                    Some(num) => Ok(self.do_stepped_slice_reverse(range, num as usize)),
                    None => Ok(self.do_slice(range.end - 1..range.end)),
                }
            } else {
                Ok(Self::empty())
            }
        }
    }
}

impl<T: Clone> PySliceableSequence for [T] {
    type Sliced = Vec<T>;

    fn do_slice(&self, range: Range<usize>) -> Self::Sliced {
        self[range].to_vec()
    }

    fn do_slice_reverse(&self, range: Range<usize>) -> Self::Sliced {
        let mut slice = self[range].to_vec();
        slice.reverse();
        slice
    }

    fn do_stepped_slice(&self, range: Range<usize>, step: usize) -> Self::Sliced {
        self[range].iter().step_by(step).cloned().collect()
    }

    fn do_stepped_slice_reverse(&self, range: Range<usize>, step: usize) -> Self::Sliced {
        self[range].iter().rev().step_by(step).cloned().collect()
    }

    #[inline(always)]
    fn empty() -> Self::Sliced {
        Vec::new()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

pub enum SequenceIndex {
    Int(isize),
    Slice(PySliceRef),
}

impl TryFromObject for SequenceIndex {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        match_class!(match obj {
            i @ PyInt => i
                .as_bigint()
                .to_isize()
                .map(SequenceIndex::Int)
                .ok_or_else(|| vm
                    .new_index_error("cannot fit 'int' into an index-sized integer".to_owned())),
            s @ PySlice => Ok(SequenceIndex::Slice(s)),
            obj => Err(vm.new_type_error(format!(
                "sequence indices be integers or slices, not {}",
                obj.class(),
            ))),
        })
    }
}

/// Get the index into a sequence like type. Get it from a python integer
/// object, accounting for negative index, and out of bounds issues.
pub fn get_sequence_index(vm: &VirtualMachine, index: &PyIntRef, length: usize) -> PyResult<usize> {
    if let Some(value) = index.as_bigint().to_i64() {
        if value < 0 {
            let from_end: usize = -value as usize;
            if from_end > length {
                Err(vm.new_index_error("Index out of bounds!".to_owned()))
            } else {
                let index = length - from_end;
                Ok(index)
            }
        } else {
            let index = value as usize;
            if index >= length {
                Err(vm.new_index_error("Index out of bounds!".to_owned()))
            } else {
                Ok(index)
            }
        }
    } else {
        Err(vm.new_index_error("cannot fit 'int' into an index-sized integer".to_owned()))
    }
}

pub fn get_pos(p: isize, len: usize) -> Option<usize> {
    let neg = p.is_negative();
    let p = p.abs().to_usize()?;
    if neg {
        len.checked_sub(p)
    } else if p >= len {
        None
    } else {
        Some(p)
    }
}

pub fn get_slice_pos(slice_pos: &BigInt, len: usize) -> usize {
    if let Some(pos) = slice_pos.to_isize() {
        if let Some(index) = get_pos(pos, len) {
            // within bounds
            return index;
        }
    }

    if slice_pos.is_negative() {
        // slice past start bound, round to start
        0
    } else {
        // slice past end bound, round to end
        len
    }
}

pub fn get_slice_range(start: &Option<BigInt>, stop: &Option<BigInt>, len: usize) -> Range<usize> {
    let start = start.as_ref().map_or(0, |x| get_slice_pos(x, len));
    let stop = stop.as_ref().map_or(len, |x| get_slice_pos(x, len));

    start..stop
}

pub fn get_item(
    vm: &VirtualMachine,
    sequence: &PyObjectRef,
    elements: &[PyObjectRef],
    subscript: PyObjectRef,
) -> PyResult {
    if let Some(i) = subscript.payload::<PyInt>() {
        return match i.as_bigint().to_isize() {
            Some(value) => {
                if let Some(pos_index) = get_pos(value, elements.len()) {
                    let obj = elements[pos_index].clone();
                    Ok(obj)
                } else {
                    Err(vm.new_index_error("Index out of bounds!".to_owned()))
                }
            }
            None => {
                Err(vm.new_index_error("cannot fit 'int' into an index-sized integer".to_owned()))
            }
        };
    }

    if let Some(slice) = subscript.payload::<PySlice>() {
        if sequence.payload::<PyList>().is_some() {
            Ok(PyObject::new(
                PyList::from(elements.get_slice_items(vm, slice)?),
                sequence.class(),
                None,
            ))
        } else if sequence.payload::<PyTuple>().is_some() {
            Ok(PyObject::new(
                PyTuple::from(elements.get_slice_items(vm, slice)?),
                sequence.class(),
                None,
            ))
        } else {
            panic!("sequence get_item called for non-sequence")
        }
    } else {
        Err(vm.new_type_error(format!(
            "indexing type {:?} with index {:?} is not supported (yet?)",
            sequence, subscript
        )))
    }
}

//Check if given arg could be used with PySliceableSequence.get_slice_range()
pub fn is_valid_slice_arg(
    arg: OptionalArg<PyObjectRef>,
    vm: &VirtualMachine,
) -> PyResult<Option<BigInt>> {
    if let OptionalArg::Present(value) = arg {
        match_class!(match value {
            i @ PyInt => Ok(Some(i.as_bigint().clone())),
            _obj @ PyNone => Ok(None),
            _ => Err(vm.new_type_error(
                "slice indices must be integers or None or have an __index__ method".to_owned()
            )), // TODO: check for an __index__ method
        })
    } else {
        Ok(None)
    }
}

pub fn opt_len(obj: &PyObjectRef, vm: &VirtualMachine) -> Option<PyResult<usize>> {
    vm.get_method(obj.clone(), "__len__").map(|len| {
        let len = vm.invoke(&len?, vec![])?;
        let len = len
            .payload_if_subclass::<PyInt>(vm)
            .ok_or_else(|| {
                vm.new_type_error(format!(
                    "'{}' object cannot be interpreted as an integer",
                    len.class().name
                ))
            })?
            .as_bigint();
        if len.is_negative() {
            return Err(vm.new_value_error("__len__() should return >= 0".to_owned()));
        }
        len.to_usize().ok_or_else(|| {
            vm.new_overflow_error("cannot fit __len__() result into usize".to_owned())
        })
    })
}

pub fn len(obj: &PyObjectRef, vm: &VirtualMachine) -> PyResult<usize> {
    opt_len(obj, vm).unwrap_or_else(|| {
        Err(vm.new_type_error(format!(
            "object of type '{}' has no len()",
            obj.class().name
        )))
    })
}
