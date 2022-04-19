use crate::obj::objiter;
use crate::obj::objstr::PyStringRef;
use crate::function::{Args, KwArgs, OptionalArg, PyFuncArgs};

use crate::obj::{objbool, objtype::PyClassRef};
use crate::VirtualMachine;
use crate::obj::objint::{PyInt, PyIntRef};
use crate::obj::objfloat::{PyFloat, PyFloatRef};
use crate::obj::objlist::{PyList, PyListRef,PyListIterator};
use crate::stdlib::array::PyArrayRef;
use crate::pyobject::{
    IdProtocol, PyArithmaticValue::*, PyClassImpl, PyComparisonValue, PyContext, PyIterable,
    PyObjectRef, PyRef, PyResult, PyValue, TryFromObject, TypeProtocol,
};
use ndarray::{s, Axis, Array, Array2, Array3, arr2, arr3};
use num_traits::ToPrimitive;

use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;
use std::println; 
use std::sync::Arc;

#[pyclass(name = "ndarray")]
#[derive(Debug)]
pub struct Pyndarray {
    value:u32,
    Ndarray: Array2<f64>
}

// pub type PyndarrayRef = PyRef<Pyndarray>;


impl PyValue for Pyndarray {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.ctx.ndarray_type()
    }
}

fn array1(inputArray: PyListRef,vm: &VirtualMachine) -> PyResult<Pyndarray>{

    /**
     * not implemented 
     * 0: initialized   
     * 1: int128
     * 2: float64
     */
    let mut eachLength : usize = 0;
    let mut length : usize = inputArray.borrow_elements().len();
    for elem1 in inputArray.borrow_elements().iter() {

        match PyListRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                if(eachLength==0){
                    eachLength = result.borrow_elements().len();
                }
                if(eachLength!=result.borrow_elements().len()){
                    return Err(vm.new_value_error("it seems that the length of some input list are not the same".to_owned()))
                }
            },
            _ => {
                return Err(vm.new_not_implemented_error("sorry, but the ndarry just support two dimension array".to_owned()))
            }
        }
    }
    let mut array = Array::ones((length,eachLength));
    let mut rowIndex = 0;
    let mut columnIndex = 0;
    for elem1 in inputArray.borrow_elements().iter() {
        match PyListRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                for elem2 in result.borrow_elements().iter(){
                    match PyIntRef::try_from_object(vm, elem2.clone()) {
                        Ok(result) => {
                            let value = result.as_bigint().to_f64().unwrap();
                            array[[rowIndex,columnIndex]] = value;
                            columnIndex+=1;
                        },
                        _ => {}
                    }
                    match PyFloatRef::try_from_object(vm, elem2.clone()) {
                        Ok(result) => {
                            let value = result.to_f64();
                            array[[rowIndex,columnIndex]] = value;
                            columnIndex+=1;
                        },
                        _ => {}
                    }
                }
                rowIndex+=1;
                columnIndex = 0;
            },
            _ => {
                return Err(vm.new_not_implemented_error("sorry, but the ndarry just support two dimension array".to_owned()))
            }
        }

    }
    println!("{:?}",array);
    let returnValue = Pyndarray{
        value:1,
        Ndarray:array
    };
    Ok(returnValue)
    
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;
    py_module!(vm, "numpy", {
        "array" => ctx.new_function(array1)

    })
}
