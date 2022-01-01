use wasmi::{ExternVal, ModuleInstance};
use wasmi::FuncInstance;
use wasmi::ImportsBuilder;
use tests::parse_wat;
use wasmi::{Signature, ValueType};

//#[should_panic]
//#[test]
pub fn assert_no_start_panics_on_module_with_start() {
    let module_with_start = parse_wat(
        r#"
		(module
			(func $f)
			(start $f))
		"#,
    );
    let module = ModuleInstance::new(&module_with_start, &ImportsBuilder::default()).unwrap();
    assert!(!module.has_start());
    module.assert_no_start();
}

//#[test]
pub fn imports_provided_by_externvals() {
    let module_with_single_import = parse_wat(
        r#"
		(module
			(import "foo" "bar" (func))
			)
		"#,
    );

    assert!(ModuleInstance::with_externvals(
        &module_with_single_import,
        [ExternVal::Func(FuncInstance::alloc_host(
            Signature::new(&[][..], None),
            0
        ),)]
        .iter(),
    )
    .is_ok());

    // externval vector is longer than import count.
    assert!(ModuleInstance::with_externvals(
        &module_with_single_import,
        [
            ExternVal::Func(FuncInstance::alloc_host(Signature::new(&[][..], None), 0)),
            ExternVal::Func(FuncInstance::alloc_host(Signature::new(&[][..], None), 1)),
        ]
        .iter(),
    )
    .is_err());

    // externval vector is shorter than import count.
    assert!(ModuleInstance::with_externvals(&module_with_single_import, [].iter(),).is_err());

    // externval vector has an unexpected type.
    assert!(ModuleInstance::with_externvals(
        &module_with_single_import,
        [ExternVal::Func(FuncInstance::alloc_host(
            Signature::new(&[][..], Some(ValueType::I32)),
            0
        ),)]
        .iter(),
    )
    .is_err());
}
