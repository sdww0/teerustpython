use assert_matches::assert_matches;
use parity_wasm::elements::{Local, ValueType};
use wasmi_validation::util::Locals;

//#[test]
pub fn locals_it_works() {
    let params = vec![ValueType::I32, ValueType::I64];
    let local_groups = vec![Local::new(2, ValueType::F32), Local::new(2, ValueType::F64)];
    let locals = Locals::new(&params, &local_groups).unwrap();

    assert_matches!(locals.type_of_local(0), Ok(ValueType::I32));
    assert_matches!(locals.type_of_local(1), Ok(ValueType::I64));
    assert_matches!(locals.type_of_local(2), Ok(ValueType::F32));
    assert_matches!(locals.type_of_local(3), Ok(ValueType::F32));
    assert_matches!(locals.type_of_local(4), Ok(ValueType::F64));
    assert_matches!(locals.type_of_local(5), Ok(ValueType::F64));
    assert_matches!(locals.type_of_local(6), Err(_));
}

//#[test]
pub fn locals_no_declared_locals() {
    let params = vec![ValueType::I32];
    let locals = Locals::new(&params, &[]).unwrap();

    assert_matches!(locals.type_of_local(0), Ok(ValueType::I32));
    assert_matches!(locals.type_of_local(1), Err(_));
}

//#[test]
pub fn locals_no_params() {
    let local_groups = vec![Local::new(2, ValueType::I32), Local::new(3, ValueType::I64)];
    let locals = Locals::new(&[], &local_groups).unwrap();

    assert_matches!(locals.type_of_local(0), Ok(ValueType::I32));
    assert_matches!(locals.type_of_local(1), Ok(ValueType::I32));
    assert_matches!(locals.type_of_local(2), Ok(ValueType::I64));
    assert_matches!(locals.type_of_local(3), Ok(ValueType::I64));
    assert_matches!(locals.type_of_local(4), Ok(ValueType::I64));
    assert_matches!(locals.type_of_local(5), Err(_));
}

//#[test]
pub fn locals_u32_overflow() {
    let local_groups = vec![
        Local::new(u32::max_value(), ValueType::I32),
        Local::new(1, ValueType::I64),
    ];
    assert_matches!(Locals::new(&[], &local_groups), Err(_));
}
