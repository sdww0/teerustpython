use wasmi::host::{HostError, RuntimeArgs};
use wasmi::RuntimeValue;

//#[test]
pub fn i32_runtime_args() {
    let args: RuntimeArgs = (&[RuntimeValue::I32(0)][..]).into();
    let val: i32 = args.nth_checked(0).unwrap();
    assert_eq!(val, 0);
}

//#[test]
pub fn i64_invalid_arg_cast() {
    let args: RuntimeArgs = (&[RuntimeValue::I64(90534534545322)][..]).into();
    assert!(args.nth_checked::<i32>(0).is_err());
}

// Tests that `HostError` trait is object safe.
fn _host_error_is_object_safe(_: &HostError) {}
