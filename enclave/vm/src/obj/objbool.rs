use num_bigint::Sign;
use num_traits::Zero;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

use crate::function::PyFuncArgs;
use crate::pyobject::{
    IdProtocol, IntoPyObject, PyClassImpl, PyContext, PyObjectRef, PyResult, TryFromObject,
    TypeProtocol,
};
use crate::vm::VirtualMachine;

use super::objint::PyInt;
use super::objstr::PyStringRef;
use super::objtype;

impl IntoPyObject for bool {
    fn into_pyobject(self, vm: &VirtualMachine) -> PyResult {
        Ok(vm.ctx.new_bool(self))
    }
}

impl TryFromObject for bool {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<bool> {
        if objtype::isinstance(&obj, &vm.ctx.int_type()) {
            Ok(get_value(&obj))
        } else {
            Err(vm.new_type_error(format!("Expected type bool, not {}", obj.class().name)))
        }
    }
}

/// Convert Python bool into Rust bool.
pub fn boolval(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<bool> {
    if obj.is(&vm.ctx.true_value) {
        return Ok(true);
    }
    if obj.is(&vm.ctx.false_value) {
        return Ok(false);
    }
    let rs_bool = match vm.get_method(obj.clone(), "__bool__") {
        Some(method_or_err) => {
            // If descriptor returns Error, propagate it further
            let method = method_or_err?;
            let bool_obj = vm.invoke(&method, PyFuncArgs::default())?;
            if !objtype::isinstance(&bool_obj, &vm.ctx.bool_type()) {
                return Err(vm.new_type_error(format!(
                    "__bool__ should return bool, returned type {}",
                    bool_obj.class().name
                )));
            }

            get_value(&bool_obj)
        }
        None => match vm.get_method(obj.clone(), "__len__") {
            Some(method_or_err) => {
                let method = method_or_err?;
                let bool_obj = vm.invoke(&method, PyFuncArgs::default())?;
                match bool_obj.payload::<PyInt>() {
                    Some(int_obj) => {
                        let len_val = int_obj.as_bigint();
                        if len_val.sign() == Sign::Minus {
                            return Err(
                                vm.new_value_error("__len__() should return >= 0".to_owned())
                            );
                        }

                        !len_val.is_zero()
                    }
                    None => {
                        return Err(vm.new_type_error(format!(
                            "'{}' object cannot be interpreted as an integer",
                            bool_obj.class().name
                        )))
                    }
                }
            }
            None => true,
        },
    };
    Ok(rs_bool)
}

/// bool(x) -> bool
///
/// Returns True when the argument x is true, False otherwise.
/// The builtins True and False are the only two instances of the class bool.
/// The class bool is a subclass of the class int, and cannot be subclassed.
#[pyclass]
struct PyBool;

#[pyimpl]
impl PyBool {
    #[pymethod(magic)]
    fn repr(zelf: bool) -> String {
        if zelf { "True" } else { "False" }.to_owned()
    }

    #[pymethod(magic)]
    fn format(
        obj: PyObjectRef,
        format_spec: PyStringRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyStringRef> {
        if format_spec.as_str().is_empty() {
            vm.to_str(&obj)
        } else {
            Err(vm.new_type_error("unsupported format string passed to bool.__format__".to_owned()))
        }
    }

    #[pymethod(name = "__ror__")]
    #[pymethod(magic)]
    fn or(lhs: PyObjectRef, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult {
        if objtype::isinstance(&lhs, &vm.ctx.bool_type())
            && objtype::isinstance(&rhs, &vm.ctx.bool_type())
        {
            let lhs = get_value(&lhs);
            let rhs = get_value(&rhs);
            (lhs || rhs).into_pyobject(vm)
        } else {
            get_py_int(&lhs).or(rhs.clone(), vm).into_pyobject(vm)
        }
    }

    #[pymethod(name = "__rand__")]
    #[pymethod(magic)]
    fn and(lhs: PyObjectRef, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult {
        if objtype::isinstance(&lhs, &vm.ctx.bool_type())
            && objtype::isinstance(&rhs, &vm.ctx.bool_type())
        {
            let lhs = get_value(&lhs);
            let rhs = get_value(&rhs);
            (lhs && rhs).into_pyobject(vm)
        } else {
            get_py_int(&lhs).and(rhs.clone(), vm).into_pyobject(vm)
        }
    }

    #[pymethod(name = "__rxor__")]
    #[pymethod(magic)]
    fn xor(lhs: PyObjectRef, rhs: PyObjectRef, vm: &VirtualMachine) -> PyResult {
        if objtype::isinstance(&lhs, &vm.ctx.bool_type())
            && objtype::isinstance(&rhs, &vm.ctx.bool_type())
        {
            let lhs = get_value(&lhs);
            let rhs = get_value(&rhs);
            (lhs ^ rhs).into_pyobject(vm)
        } else {
            get_py_int(&lhs).xor(rhs.clone(), vm).into_pyobject(vm)
        }
    }

    #[pyslot]
    fn tp_new(vm: &VirtualMachine, args: PyFuncArgs) -> PyResult {
        arg_check!(
            vm,
            args,
            required = [(_zelf, Some(vm.ctx.type_type()))],
            optional = [(val, None)]
        );
        let value = match val {
            Some(val) => boolval(vm, val.clone())?,
            None => false,
        };
        Ok(vm.new_bool(value))
    }
}

pub(crate) fn init(context: &PyContext) {
    PyBool::extend_class(context, &context.types.bool_type);
}

pub fn not(vm: &VirtualMachine, obj: &PyObjectRef) -> PyResult<bool> {
    if objtype::isinstance(obj, &vm.ctx.bool_type()) {
        let value = get_value(obj);
        Ok(!value)
    } else {
        Err(vm.new_type_error(format!("Can only invert a bool, on {:?}", obj)))
    }
}

// Retrieve inner int value:
pub fn get_value(obj: &PyObjectRef) -> bool {
    !obj.payload::<PyInt>().unwrap().as_bigint().is_zero()
}

pub fn get_py_int(obj: &PyObjectRef) -> &PyInt {
    &obj.payload::<PyInt>().unwrap()
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct IntoPyBool {
    value: bool,
}

impl IntoPyBool {
    pub const TRUE: IntoPyBool = IntoPyBool { value: true };
    pub const FALSE: IntoPyBool = IntoPyBool { value: false };

    pub fn to_bool(self) -> bool {
        self.value
    }
}

impl TryFromObject for IntoPyBool {
    fn try_from_object(vm: &VirtualMachine, obj: PyObjectRef) -> PyResult<Self> {
        Ok(IntoPyBool {
            value: boolval(vm, obj)?,
        })
    }
}
