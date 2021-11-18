use super::os::errno_err;
use crate::obj::objbytes::PyBytesRef;
use crate::obj::objstr::PyStringRef;
use crate::pyobject::{PyObjectRef, PyResult};
use crate::VirtualMachine;

use itertools::Itertools;

extern "C" {
    fn _getch() -> i32;
    fn _getwch() -> u32;
    fn _getche() -> i32;
    fn _getwche() -> u32;
    fn _putch(c: u32) -> i32;
    fn _putwch(c: u16) -> u32;
}

fn msvcrt_getch() -> Vec<u8> {
    let c = unsafe { _getch() };
    vec![c as u8]
}
fn msvcrt_getwch() -> String {
    let c = unsafe { _getwch() };
    std::char::from_u32(c).unwrap().to_string()
}
fn msvcrt_getche() -> Vec<u8> {
    let c = unsafe { _getche() };
    vec![c as u8]
}
fn msvcrt_getwche() -> String {
    let c = unsafe { _getwche() };
    std::char::from_u32(c).unwrap().to_string()
}
fn msvcrt_putch(b: PyBytesRef, vm: &VirtualMachine) -> PyResult<()> {
    let &c = b.get_value().iter().exactly_one().map_err(|_| {
        vm.new_type_error("putch() argument must be a byte string of length 1".to_owned())
    })?;
    unsafe { suppress_iph!(_putch(c.into())) };
    Ok(())
}
fn msvcrt_putwch(s: PyStringRef, vm: &VirtualMachine) -> PyResult<()> {
    let c = s.as_str().chars().exactly_one().map_err(|_| {
        vm.new_type_error("putch() argument must be a string of length 1".to_owned())
    })?;
    unsafe { suppress_iph!(_putwch(c as u16)) };
    Ok(())
}

extern "C" {
    fn _setmode(fd: i32, flags: i32) -> i32;
}

fn msvcrt_setmode(fd: i32, flags: i32, vm: &VirtualMachine) -> PyResult<i32> {
    let flags = unsafe { suppress_iph!(_setmode(fd, flags)) };
    if flags == -1 {
        Err(errno_err(vm))
    } else {
        Ok(flags)
    }
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;
    py_module!(vm, "_msvcrt", {
        "getch" => ctx.new_function(msvcrt_getch),
        "getwch" => ctx.new_function(msvcrt_getwch),
        "getche" => ctx.new_function(msvcrt_getche),
        "getwche" => ctx.new_function(msvcrt_getwche),
        "putch" => ctx.new_function(msvcrt_putch),
        "putwch" => ctx.new_function(msvcrt_putwch),
        "setmode" => ctx.new_function(msvcrt_setmode),
    })
}
