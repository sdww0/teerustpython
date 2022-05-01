use crate::obj::objiter;
use crate::function::{Args, KwArgs, OptionalArg, PyFuncArgs};
use num_bigint::{BigInt, Sign,ToBigInt};
use num_traits::{Zero, FromPrimitive};
use crate::obj::{objbool, objtype::PyClassRef};
use crate::VirtualMachine;
use crate::obj::objint::{PyInt, PyIntRef};
use crate::obj::objfloat::{PyFloat, PyFloatRef};
use crate::obj::objlist::{PyList, PyListRef,PyListIterator};
use crate::obj::objtuple::{PyTuple,PyTupleRef};
use crate::obj::objstr::{PyString,PyStringRef};
use crate::obj::objslice::{PySliceRef,PySlice};
use crate::stdlib::array::PyArrayRef;
use crate::pyobject::{
    IdProtocol, PyArithmaticValue::*, PyClassImpl, PyComparisonValue, PyContext, PyIterable,PyObject,
    PyObjectRef, PyRef, PyResult, PyValue, TryFromObject, TypeProtocol,
};
use crate::obj::objsequence::{get_item, get_pos, get_slice_range, SequenceIndex};
use ndarray::{s, Axis, Array, Array2, Array3, arr2, arr3};
use ndarray::RemoveAxis;
use ndarray_rand::rand::{distributions::Distribution, thread_rng};
use ndarray_rand::rand_distr::{Uniform,Normal};
use ndarray_rand::{RandomExt, SamplingStrategy};
use num_traits::ToPrimitive;
use core::ops::Deref;
use std::sync::{Arc,SgxRwLock as RwLock,SgxRwLockReadGuard as  RwLockReadGuard,SgxRwLockWriteGuard as RwLockWriteGuard};

use core::borrow::{BorrowMut, Borrow};
use std::str::FromStr;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;
use std::println; 
use core::{i16, i32, i64, i8, isize};


#[pyclass(name = "ndarray")]
#[derive(Debug)]
pub struct PyNdarray {
    ndarray: RwLock<Array2<f64>>
}

impl PyValue for PyNdarray {
    fn class(vm: &VirtualMachine) -> PyClassRef {
        vm.class("numpy", "ndarray")
    }
}
pub type PyNdarrayRef = PyRef<PyNdarray>;

#[pyimpl]
impl PyNdarray {

    pub fn borrow_elements(&self) -> RwLockReadGuard<'_, Array2<f64>> {
        self.ndarray.read().unwrap()
    }

    pub fn borrow_elements_mut(&self) -> RwLockWriteGuard<'_, Array2<f64>> {
        self.ndarray.write().unwrap()
    }

    #[pyproperty(name = "shape")]
    fn shape(zelf: PyRef<Self>,vm:&VirtualMachine) -> PyResult<PyObjectRef>{
        let borrow = zelf.borrow_elements();
        let ndarray_shape = borrow.shape();
        let mut vec_list : Vec<PyObjectRef> = Vec::new();
        for element in ndarray_shape{
            vec_list.push(vm.ctx.new_bigint(&ToBigInt::to_bigint(element).unwrap()));
        }
        Ok(vm.ctx.new_list(vec_list))
    }

    #[pymethod(magic)]
    fn str(zelf: PyRef<Self>) ->  PyResult<String>{
        Ok(zelf.borrow_elements().to_string())
    }

    #[pymethod(magic)]
    fn repr(zelf: PyNdarrayRef) -> PyResult<String> {
        Ok(zelf.borrow_elements().to_string())
    }


    #[pymethod(name = "__add__")]
    fn add(zelf: PyNdarrayRef,other:PyObjectRef,vm:&VirtualMachine) -> PyResult<PyNdarray> {
        match PyIntRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.as_bigint().to_f64().unwrap();
                let result_array = a + value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyFloatRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.to_f64();
                let result_array = a + value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyNdarrayRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let first = zelf.borrow_elements();
                let second = result.borrow_elements();
                let a = first.deref();
                let b = second.deref();
                let result = a+b;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result)
                })
            },
            Err(_) => {},
        }

        return Err(vm.new_type_error("check your input please".to_owned()));
    }


    #[pymethod(name = "__truediv__")]
    fn div(zelf: PyNdarrayRef,other:PyObjectRef,vm:&VirtualMachine) -> PyResult<PyNdarray> {
        match PyIntRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.as_bigint().to_f64().unwrap();
                let result_array = a/value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyFloatRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.to_f64();
                let result_array = a/value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyNdarrayRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let first = zelf.borrow_elements();
                let second = result.borrow_elements();
                let a = first.deref();
                let b = second.deref();
                let result = a/b;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result)
                })
            },
            Err(_) => {},
        }

        return Err(vm.new_type_error("check your input please".to_owned()));
    }


    #[pymethod(name = "__sub__")]
    fn sub(zelf: PyNdarrayRef,other:PyObjectRef,vm:&VirtualMachine) -> PyResult<PyNdarray> {
        match PyIntRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.as_bigint().to_f64().unwrap();
                let result_array = a - value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyFloatRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.to_f64();
                let result_array = a - value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyNdarrayRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let first = zelf.borrow_elements();
                let second = result.borrow_elements();
                let a = first.deref();
                let b = second.deref();
                let result = a-b;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result)
                })
            },
            Err(_) => {},
        }

        return Err(vm.new_type_error("check your input please".to_owned()));
    }

    #[pymethod(name = "__mul__")]
    fn mul(zelf: PyNdarrayRef,other:PyObjectRef,vm:&VirtualMachine) -> PyResult<PyNdarray> {
        match PyIntRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.as_bigint().to_f64().unwrap();
                let result_array = a * value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyFloatRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let ndarray = zelf.borrow_elements();
                let a = ndarray.deref();
                let value = result.to_f64();
                let result_array = a * value;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result_array)
                })
            },
            Err(_) => {},
        }
        match PyNdarrayRef::try_from_object(vm, other.to_owned()){
            Ok(result) => {
                let first = zelf.borrow_elements();
                let second = result.borrow_elements();
                let a = first.deref();
                let b = second.deref();
                let result = a*b;
                return Ok(PyNdarray{
                    ndarray:RwLock::new(result)
                })
            },
            Err(_) => {},
        }

        return Err(vm.new_type_error("check your input please".to_owned()));
    }

    // TODO: support other type of ndarray
    #[pymethod(name = "__getitem__")]
    fn getitem(zelf: PyRef<Self>, needle: PyObjectRef, vm: &VirtualMachine) -> PyResult<PyObjectRef> {
        match PyTupleRef::try_from_object(vm,needle.to_owned()){
            Ok(result) => {
                let mut count = 0;
                let mut row_index = 0;
                let mut column_index = 0;
                let mut input_tuple_type = 0;
                for element in result.as_slice(){
                    match PyIntRef::try_from_object(vm,element.to_owned()){
                        Ok(value) => {
                            if(input_tuple_type!=0&&input_tuple_type!=1){
                                return Err(vm.new_type_error("check your input please".to_owned()));
                            }
                            input_tuple_type = 1;
                            if(count==0){
                                row_index =  value.as_bigint().to_usize().unwrap();
                            }else if(count==1){
                                column_index = value.as_bigint().to_usize().unwrap();
                            }
                            count+=1;
                        },
                        Err(_) => {},
                    }
                    match PySliceRef::try_from_object(vm,element.to_owned()){
                        //currently, it just support the one in "test.py"
                        Ok(value) => {
                            if(input_tuple_type!=0&&input_tuple_type!=2){
                                return Err(vm.new_type_error("check your input please".to_owned()));
                            }
                            input_tuple_type = 2;
                            let ndarray = zelf.borrow_elements();
                            let start = value.start_index(vm).unwrap();
                            let stop = value.stop_index(vm).unwrap();
                            let result;
                            
                            match start{
                                Some(x)=>{

                                    match stop{
                                        Some(y)=>{
                                            let stop_pos = y.to_usize().unwrap();
                                            let start_pos = x.to_usize().unwrap();
                                            result = PyNdarray{
                                                ndarray:RwLock::new(ndarray.slice(s![start_pos..stop_pos,..]).to_owned())
                                            };
                                            return Ok(PyObject::new(result,PyNdarray::class(vm),None));
                                        },
                                        None => {
                                            let start_pos = x.to_usize().unwrap();
                                            result = PyNdarray{
                                                ndarray:RwLock::new(ndarray.slice(s![start_pos..,..]).to_owned())
                                            };
                                            return Ok(PyObject::new(result,PyNdarray::class(vm),None));

                                        }
                                    }
                                },
                                None => {
                                    match stop{
                                        Some(x)=>{
                                            let stop_pos = x.to_usize().unwrap();
                                            result = PyNdarray{
                                                ndarray:RwLock::new(ndarray.slice(s![..stop_pos,..]).to_owned())
                                            };
                                            return Ok(PyObject::new(result,PyNdarray::class(vm),None));

                                        },
                                        None => {
                                            result = PyNdarray{
                                                ndarray:RwLock::new(ndarray.to_owned())
                                            };
                                            return Ok(PyObject::new(result,PyNdarray::class(vm),None));
                                        }
                                    }
                                }
                            }
                        },
                        Err(_) => {},
                    }
                }
                if(input_tuple_type==1){
                    return Ok(vm.ctx.new_float(zelf.borrow_elements_mut()[[row_index,column_index]]));
                }
                
            },
            Err(_) => {},
        }
        return Err(vm.new_type_error("check your input please".to_owned()));
    }

    #[pymethod(name = "sum")]
    fn sum(
        self: &Self,
        axis: PyIntRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyNdarray> {
        let ndarray = self.borrow_elements();
        let result = ndarray.sum_axis(Axis(axis.as_bigint().to_usize().unwrap()));
        let return_result = result.insert_axis(Axis(1));
        Ok(PyNdarray{
            ndarray:RwLock::new(return_result)
        })
    }


    #[pymethod(name = "__neg__")]
    fn neg(
        self: &Self,
        vm: &VirtualMachine,
    ) -> PyResult<PyNdarray> {
        let ndarray = self.borrow_elements();
        let result = ndarray.mapv(|a| -a );
        Ok(PyNdarray{
            ndarray:RwLock::new(result)
        })
    }

    #[pymethod(name = "max")]
    fn max(
        self: &Self,
        axis: PyIntRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyNdarray> {
        let ndarray = self.borrow_elements();
        let matrix = ndarray.deref();
        let axis_value = axis.as_bigint().to_usize().unwrap();
        let mut result = Array::zeros(matrix.raw_dim().remove_axis(Axis(axis_value)));
        let shape = matrix.shape();
        let row_amount = shape[0];
        let column_amount = shape[1];
        if(axis_value==0){
            for column in 0..column_amount{
                let mut compare_value = matrix[[0,column]];
                for row in 0..row_amount{
                    compare_value = f64::max(matrix[[row,column]],compare_value);

                }
                result[column] = compare_value;
            }
        }else if(axis_value==1){
            for row in 0..row_amount{
                let mut compare_value = matrix[[row,0]];
                for column in 0..column_amount{
                    compare_value = f64::max(matrix[[row,column]],compare_value);
                }
                result[row] = compare_value;
            }
        }else{
            return Err(vm.new_not_implemented_error("not support other type of array".to_owned()));
        }
        let return_result = result.insert_axis(Axis(1));
        Ok(PyNdarray{
            ndarray:RwLock::new(return_result)
        })
    }

    #[pymethod(name = "set")]
    fn set(
        self: &Self,
        x: PyIntRef,
        y: PyIntRef,
        value: PyFloatRef,
        vm: &VirtualMachine,
    )  {
        let x_value = x.as_bigint().to_usize().unwrap();
        let y_value = y.as_bigint().to_usize().unwrap();
        let set_value = value.to_f64();
        self.borrow_elements_mut()[[x_value,y_value]] = set_value;
    }

    // TODO complete
    #[pymethod(name = "__setitem__")]
    fn setitem(
        zelf: PyRef<Self>,
        subscript: SequenceIndex,
        value: PyObjectRef,
        vm: &VirtualMachine,
    ) -> PyResult<PyObjectRef> {
        let mut set_value = 0.0;
        let mut check = false;
        match PyIntRef::try_from_object(vm,value.to_owned()){
            Ok(result) => {
                set_value = result.as_bigint().to_f64().unwrap();
                check = true;
            },
            Err(_) => {},
        }
        match PyFloatRef::try_from_object(vm,value.to_owned()){
            Ok(result) => {
                set_value = result.to_f64();
                check = true;
            },
            Err(_) => {},
        }
        if(!check){
            return Err(vm.new_value_error("check your value please".to_owned()));
        }
        match subscript {
            SequenceIndex::Int(index) => {return Err(vm.new_not_implemented_error("please input two index".to_owned()));},
            SequenceIndex::Slice(slice) => {
                println!("{:?}",slice.start_index(vm).unwrap());
                // if let Ok(sec) = PyIterable::try_from_object(vm, value) {
                //     return self.setslice(slice, sec, vm);
                // }

            }
        }
        return Err(vm.new_value_error("check your value please".to_owned()));
    }

}

fn ndarray_random_uniform(low: PyFloatRef,high:PyFloatRef,size:PyTupleRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let mut row_amount = 0;
    let mut column_amount = 0;
    let mut count = 0;
    let tuple = size.as_slice();
    for elemet in tuple{
        match PyIntRef::try_from_object(vm,elemet.to_owned()){
            Ok(result) => {
                if(count==0){
                    row_amount = result.as_bigint().to_usize().unwrap();
                }else{
                    column_amount = result.as_bigint().to_usize().unwrap();
                }
                count+=1;
                
            },
            Err(_) => return Err(vm.new_not_implemented_error("check your input please".to_owned())),
        }
    }
    let result = Array::random((row_amount, column_amount), Uniform::new(low.to_f64(), high.to_f64()));
    Ok(PyNdarray{
        ndarray:RwLock::new(result)
    })
}

fn ndarray_random_normal(loc: PyFloatRef,scale:PyFloatRef,size:PyTupleRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let mut row_amount = 0;
    let mut column_amount = 0;
    let mut count = 0;
    let tuple = size.as_slice();
    for elemet in tuple{
        match PyIntRef::try_from_object(vm,elemet.to_owned()){
            Ok(result) => {
                if(count==0){
                    row_amount = result.as_bigint().to_usize().unwrap();
                }else if(count==0){
                    column_amount = result.as_bigint().to_usize().unwrap();
                }
                count+=1;
                
            },
            Err(_) => return Err(vm.new_not_implemented_error("check your input please".to_owned())),
        }
    }
    let result = Array::random((row_amount, column_amount), Normal::new(loc.to_f64(), scale.to_f64()).unwrap());
    Ok(PyNdarray{
        ndarray:RwLock::new(result)
    })
}

fn ndarray_transpose(matrix: PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{

    let ndarray = &matrix.borrow_elements();
    let dimension = ndarray.shape();
    let mut new_array = Array::zeros((dimension[1],dimension[0]));
    for n in 0..dimension[0]{
        for i in 0..dimension[1]{
            new_array[[i,n]] = ndarray[[n,i]];
        } 
    }
    Ok(PyNdarray{
        ndarray: RwLock::new(new_array),
    })
}

fn ndarray_sum(matrix: PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyFloat>{
    let result = matrix.borrow_elements().sum();
    Ok(PyFloat::from(result))
}

fn ndarray_argmax(
    matrix: PyNdarrayRef,
    axis: PyIntRef,
    vm: &VirtualMachine,
) -> PyResult<PyNdarray> {
    let ndarray = matrix.borrow_elements();
    let matrix = ndarray.deref();
    let axis_value = axis.as_bigint().to_usize().unwrap();
    let mut result = Array::zeros(matrix.raw_dim().remove_axis(Axis(axis_value)));
    let shape = matrix.shape();
    let row_amount = shape[0];
    let column_amount = shape[1];
    if(axis_value==0){
        for column in 0..column_amount{
            let mut compare_value = matrix[[0,column]];
            let mut index: usize = 0;
            for row in 0..row_amount{
                if(compare_value<matrix[[row,column]]){
                    index = row;
                    compare_value = matrix[[row,column]];
                }
            }
            result[column] = index as f64;
        }
    }else if(axis_value==1){
        for row in 0..row_amount{
            let mut compare_value = matrix[[row,0]];
            let mut index: usize = 0;
            for column in 0..column_amount{
                if(compare_value<matrix[[row,column]]){
                    index = column;
                    compare_value = matrix[[row,column]];
                }
            }
            result[row] = index as f64;
        }
    }else{
        return Err(vm.new_not_implemented_error("not support other type of array".to_owned()));
    }
    let return_result = result.insert_axis(Axis(1));
    Ok(PyNdarray{
        ndarray:RwLock::new(return_result)
    })
}

fn ndarray_maximum(matrix:PyNdarrayRef,value:PyObjectRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    match PyIntRef::try_from_object(vm,value.to_owned()){
        Ok(result) => {
            let compare_value = result.as_bigint().to_f64().unwrap();
            let ndarray = matrix.borrow_elements();
            let ndarray_deref = ndarray.deref();
            let mut new_matrix = ndarray_deref.to_owned();
            let shape = new_matrix.shape();
            let row_amount = shape[0];
            let column_amount = shape[1];
            for row in 0..row_amount{
                for column in 0..column_amount{
                    if(new_matrix[[row,column]]<compare_value){
                        new_matrix[[row,column]] = compare_value;
                    }
                }
            }
            return Ok(PyNdarray{
                ndarray:RwLock::new(new_matrix)
            });
        },
        Err(_) => {}
    }
    match PyFloatRef::try_from_object(vm,value.to_owned()){
        Ok(result) => {
            let compare_value = result.to_f64();
            let ndarray = matrix.borrow_elements();
            let ndarray_deref = ndarray.deref();
            let mut new_matrix = ndarray_deref.to_owned();
            let shape = new_matrix.shape();
            let row_amount = shape[0];
            let column_amount = shape[1];
            for row in 0..row_amount{
                for column in 0..column_amount{
                    if(new_matrix[[row,column]]<compare_value){
                        new_matrix[[row,column]] = compare_value;
                    }
                }
            }
            return Ok(PyNdarray{
                ndarray:RwLock::new(new_matrix)
            });
        },
        Err(_) => {}
    }

    return Err(vm.new_value_error("check your input please".to_owned()));
}

fn ndarray_minimum_or_value(matrix:PyNdarrayRef,value:PyObjectRef,set_value:PyFloatRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    match PyIntRef::try_from_object(vm,value.to_owned()){
        Ok(result) => {
            let compare_value = result.as_bigint().to_f64().unwrap();
            let ndarray = matrix.borrow_elements();
            let ndarray_deref = ndarray.deref();
            let mut new_matrix = ndarray_deref.to_owned();
            let shape = new_matrix.shape();
            let row_amount = shape[0];
            let column_amount = shape[1];
            for row in 0..row_amount{
                for column in 0..column_amount{
                    if(new_matrix[[row,column]]>compare_value){
                        new_matrix[[row,column]] = set_value.to_f64();
                    }
                }
            }
            return Ok(PyNdarray{
                ndarray:RwLock::new(new_matrix)
            });
        },
        Err(_) => {}
    }
    match PyFloatRef::try_from_object(vm,value.to_owned()){
        Ok(result) => {
            let compare_value = result.to_f64();
            let ndarray = matrix.borrow_elements();
            let ndarray_deref = ndarray.deref();
            let mut new_matrix = ndarray_deref.to_owned();
            let shape = new_matrix.shape();
            let row_amount = shape[0];
            let column_amount = shape[1];
            for row in 0..row_amount{
                for column in 0..column_amount{
                    if(new_matrix[[row,column]]>compare_value){
                        new_matrix[[row,column]] = compare_value;
                    }
                }
            }
            return Ok(PyNdarray{
                ndarray:RwLock::new(new_matrix)
            });
        },
        Err(_) => {}
    }

    return Err(vm.new_value_error("check your input please".to_owned()));
}

fn ndarray_dot(first_matrix: PyNdarrayRef,second_matrix: PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let matrix2 = second_matrix.borrow_elements();
    let matrix2_deref = matrix2.deref();
    let matrix1 = first_matrix.borrow_elements();
    let matrix1_deref = matrix1.deref();
    let result = matrix1_deref.dot(matrix2_deref);
    Ok(PyNdarray{
        ndarray: RwLock::new(result),
    })
}

fn ndarray_sqrt(matrix: PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let result = matrix.borrow_elements_mut().mapv(f64::sqrt);
    Ok(PyNdarray{
        ndarray: RwLock::new(result),
    })
}

fn ndarray_log(matrix:PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let array = matrix.borrow_elements();
    let result = array.mapv(f64::ln);
    Ok(PyNdarray{
        ndarray:RwLock::new(result)
    })
}

fn ndarray_exp(matrix:PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let array = matrix.borrow_elements();
    let result = array.mapv(f64::exp);
    Ok(PyNdarray{
        ndarray:RwLock::new(result)
    })
}

fn ndarray_count_nonzero(matrix:PyNdarrayRef,vm:&VirtualMachine) -> PyResult<PyInt>{
    let mut count = 0;
    let ndarray = matrix.borrow_elements();
    let shape = ndarray.shape();
    for n in 0..shape[0]{
        for i in 0..shape[1]{
            if(!ndarray[[n,i]].is_zero()){
                count+=1;
            }
        }
    }
    Ok(PyInt::new(BigInt::from_i32(count).unwrap()))
}

fn ndarray_tile(matrix:PyNdarrayRef,tuple:PyTupleRef,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    // TODO maybe use the function in ndarray is better
    let ndarray = matrix.borrow_elements();
    let ndarray_shape = ndarray.shape();
    let mut new_row_amount :usize = 0; 
    let mut new_column_amount : usize = 0;
    let mut count = 0;
    let mut clone_row_amount : usize = 1;
    let mut clone_column_amount : usize = 1;
    for elemet in tuple.as_slice(){
        match PyIntRef::try_from_object(vm,elemet.to_owned()){
            Ok(result) => {
                let temp_value = result.as_bigint().to_usize().unwrap();
                if(count==0){
                    clone_row_amount = temp_value;
                    new_row_amount = temp_value*ndarray_shape[0];
                }else if(count==1){
                    clone_column_amount = temp_value;
                    new_column_amount = temp_value*ndarray_shape[1];
                }
                count+=1;
                
            },
            Err(_) => return Err(vm.new_value_error("check your input please".to_owned())),
        }
    }
    if(clone_row_amount<=0||clone_column_amount<=0){
        return Err(vm.new_value_error("check your input please".to_owned()));
    }
    let mut result = Array::zeros((new_row_amount,new_column_amount));
    let mut iter_row_start_index = 0;
    let mut iter_column_start_index = 0;
    for n in 0..clone_row_amount{
        iter_row_start_index = n*ndarray_shape[0];
        for i in 0..clone_column_amount{
            iter_column_start_index = i*ndarray_shape[1];
            // clone the value
            for x in 0..ndarray_shape[0]{
                for y in 0..ndarray_shape[1]{
                    result[[iter_row_start_index+x,iter_column_start_index+y]] = ndarray[[x,y]];
                }
            }

        }
    }
    Ok(PyNdarray{
        ndarray:RwLock::new(result)
    })
}


fn ndarray_zeros(input_tuple: PyTupleRef,vm: &VirtualMachine) -> PyResult<PyNdarray>{
    let mut row_amount =0;
    let mut column_amount =0;
    let tuple = input_tuple.as_slice();
    let mut count = 0;
    for elemet in tuple{
        match PyIntRef::try_from_object(vm,elemet.to_owned()){
            Ok(result) => {
                if(count==0){
                    row_amount = result.as_bigint().to_usize().unwrap();
                }else{
                    column_amount = result.as_bigint().to_usize().unwrap();
                }
                count+=1;
                
            },
            Err(_) => return Err(vm.new_not_implemented_error("check your input please".to_owned())),
        }
    }
    if(count!=2){
        return Err(vm.new_not_implemented_error("sorry, but the ndarry just support two dimension array".to_owned()))
    }
    let array = Array::zeros((row_amount,column_amount));
    let return_value = PyNdarray{
        ndarray:RwLock::new(array),
    };
    Ok(return_value)
}

fn ndarray_ones(inputTuple: PyTupleRef,vm: &VirtualMachine) -> PyResult<PyNdarray>{
    let mut row_amount =0;
    let mut column_amount =0;
    let tuple = inputTuple.as_slice();
    let mut count = 0;
    for elemet in tuple{
        match PyIntRef::try_from_object(vm,elemet.to_owned()){
            Ok(result) => {
                if(count==0){
                    row_amount = result.as_bigint().to_usize().unwrap();
                }else{
                    column_amount = result.as_bigint().to_usize().unwrap();
                }
                count+=1;
                
            },
            Err(_) => return Err(vm.new_not_implemented_error("check your input please".to_owned())),
        }
    }
    if(count!=2){
        return Err(vm.new_not_implemented_error("sorry, but the ndarry just support two dimension array".to_owned()))
    }
    let array = Array::ones((row_amount,column_amount));
    let return_value = PyNdarray{
        ndarray:RwLock::new(array),
    };
    Ok(return_value)
}

fn ndarray_new_one_dimension_array(input_list:RwLockReadGuard<'_, Vec<PyObjectRef>>,vm:&VirtualMachine) -> PyResult<PyNdarray>{
    let length : usize = input_list.len();
    let mut array = Array::ones((length,1));
    let mut row_index = 0;
    for elem1 in input_list.iter() {
        match PyIntRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                let value = result.as_bigint().to_f64().unwrap();
                array[[row_index,0]] = value;
                row_index+=1;
            },
            _ => {}
        }
        match PyFloatRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                let value = result.to_f64();
                array[[row_index,0]] = value;
                row_index+=1;
            },
            _ => {}
        }
    }
    let return_value = PyNdarray{
        ndarray:RwLock::new(array),
    };
    Ok(return_value)
}

fn ndarray_new_array(input_list: PyListRef,vm: &VirtualMachine) -> PyResult<PyNdarray>{

    /**
     * not implemented 
     * 0: initialized   
     * 1: int128
     * 2: float64
     */
    let mut each_length : usize = 0;
    let length : usize = input_list.borrow_elements().len();
    for elem1 in input_list.borrow_elements().iter() {
        match PyFloatRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                return ndarray_new_one_dimension_array(input_list.borrow_elements(),vm);
            },
            _ => {}
        }
        match PyIntRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                return ndarray_new_one_dimension_array(input_list.borrow_elements(),vm);
            },
            _ => {}
        }
        match PyListRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                if(each_length==0){
                    each_length = result.borrow_elements().len();
                }
                if(each_length!=result.borrow_elements().len()){
                    return Err(vm.new_value_error("The length of some input list are not the same".to_owned()))
                }
            },
            _ => {
                return Err(vm.new_not_implemented_error("input wrong".to_owned()))
            }
        }
    }
    let mut array = Array::ones((length,each_length));
    let mut row_index = 0;
    let mut column_index = 0;
    for elem1 in input_list.borrow_elements().iter() {
        match PyListRef::try_from_object(vm, elem1.clone()) {
            Ok(result) => {
                for elem2 in result.borrow_elements().iter(){
                    match PyIntRef::try_from_object(vm, elem2.clone()) {
                        Ok(result) => {
                            let value = result.as_bigint().to_f64().unwrap();
                            array[[row_index,column_index]] = value;
                            column_index+=1;
                        },
                        _ => {}
                    }
                    match PyFloatRef::try_from_object(vm, elem2.clone()) {
                        Ok(result) => {
                            let value = result.to_f64();
                            array[[row_index,column_index]] = value;
                            column_index+=1;
                        },
                        _ => {}
                    }
                }
                row_index+=1;
                column_index = 0;
            },
            _ => {
                return Err(vm.new_not_implemented_error("sorry, but the ndarry just support two dimension array".to_owned()))
            }
        }
    }
    let return_value = PyNdarray{
        ndarray:RwLock::new(array),
    };
    Ok(return_value)
    
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;
    py_module!(vm, "numpy", {
        "ndarray" => PyNdarray::make_class(&vm.ctx),
        "array" => ctx.new_function(ndarray_new_array),
        "ones" => ctx.new_function(ndarray_ones),
        "zeros" => ctx.new_function(ndarray_zeros),
        "dot" => ctx.new_function(ndarray_dot),
        "sqrt" => ctx.new_function(ndarray_sqrt),
        "sum" => ctx.new_function(ndarray_sum),
        "transpose" => ctx.new_function(ndarray_transpose),
        "random_uniform" => ctx.new_function(ndarray_random_uniform),
        "random_normal" => ctx.new_function(ndarray_random_normal),
        "maximum" => ctx.new_function(ndarray_maximum),
        "minimum_or_value" => ctx.new_function(ndarray_minimum_or_value),
        "exp" => ctx.new_function(ndarray_exp),
        "log" => ctx.new_function(ndarray_log),
        "tile" => ctx.new_function(ndarray_tile),
        "count_nonzero" => ctx.new_function(ndarray_count_nonzero),
        "argmax" => ctx.new_function(ndarray_argmax),
        
    })
}
