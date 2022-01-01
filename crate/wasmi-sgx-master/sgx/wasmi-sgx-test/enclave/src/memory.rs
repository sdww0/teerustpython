use std::prelude::v1::*;
use wasmi::memory::{MemoryInstance, MemoryRef, LINEAR_MEMORY_PAGE_SIZE};
use wasmi::memory_units::Pages;
use std::sync::Arc;
use Error;

//#[test]
pub fn alloc() {
    let mut fixtures = vec![
        (0, None, true),
        (0, Some(0), true),
        (1, None, true),
        (1, Some(1), true),
        (0, Some(1), true),
        (1, Some(0), false),
    ];

    // 65536 is too much. we limit it to 1000
    //#[cfg(target_pointer_width = "64")]
    fixtures.extend(&[
        (1000, Some(1000), true),
        (1000, Some(0), false),
        (1000, None, true),
    ]);

    for (index, &(initial, maybe_max, expected_ok)) in fixtures.iter().enumerate() {
        let initial: Pages = Pages(initial);
        let maximum: Option<Pages> = maybe_max.map(|m| Pages(m));
        let result = MemoryInstance::alloc(initial, maximum);
        if result.is_ok() != expected_ok {
            panic!(
                "unexpected error at {}, initial={:?}, max={:?}, expected={}, result={:?}",
                index, initial, maybe_max, expected_ok, result,
            );
        }
    }
}

//#[test]
pub fn ensure_page_size() {
    use wasmi::memory_units::ByteSize;
    assert_eq!(LINEAR_MEMORY_PAGE_SIZE, Pages::byte_size());
}

fn create_memory(initial_content: &[u8]) -> MemoryInstance {
    let mem = MemoryInstance::new(Pages(1), Some(Pages(1))).unwrap();
    mem.set(0, initial_content)
        .expect("Successful initialize the memory");
    mem
}

//#[test]
pub fn copy_overlaps_1() {
    let mem = create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.copy(0, 4, 6).expect("Successfully copy the elements");
    let result = mem.get(0, 10).expect("Successfully retrieve the result");
    assert_eq!(result, &[0, 1, 2, 3, 0, 1, 2, 3, 4, 5]);
}

//#[test]
pub fn copy_overlaps_2() {
    let mem = create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.copy(4, 0, 6).expect("Successfully copy the elements");
    let result = mem.get(0, 10).expect("Successfully retrieve the result");
    assert_eq!(result, &[4, 5, 6, 7, 8, 9, 6, 7, 8, 9]);
}

//#[test]
pub fn copy_nonoverlapping() {
    let mem = create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.copy_nonoverlapping(0, 10, 10)
        .expect("Successfully copy the elements");
    let result = mem.get(10, 10).expect("Successfully retrieve the result");
    assert_eq!(result, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

//#[test]
pub fn copy_nonoverlapping_overlaps_1() {
    let mem = create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let result = mem.copy_nonoverlapping(0, 4, 6);
    match result {
        Err(Error::Memory(_)) => {}
        _ => panic!("Expected Error::Memory(_) result, but got {:?}", result),
    }
}

//#[test]
pub fn copy_nonoverlapping_overlaps_2() {
    let mem = create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let result = mem.copy_nonoverlapping(4, 0, 6);
    match result {
        Err(Error::Memory(_)) => {}
        _ => panic!("Expected Error::Memory(_), but got {:?}", result),
    }
}

//#[test]
pub fn transfer_works() {
    let src = MemoryRef(Arc::new(create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
    let dst = MemoryRef(Arc::new(create_memory(&[
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ])));

    MemoryInstance::transfer(&src, 4, &dst, 0, 3).unwrap();

    assert_eq!(src.get(0, 10).unwrap(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(
        dst.get(0, 10).unwrap(),
        &[4, 5, 6, 13, 14, 15, 16, 17, 18, 19]
    );
}

//#[test]
pub fn transfer_still_works_with_same_memory() {
    let src = MemoryRef(Arc::new(create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));

    MemoryInstance::transfer(&src, 4, &src, 0, 3).unwrap();

    assert_eq!(src.get(0, 10).unwrap(), &[4, 5, 6, 3, 4, 5, 6, 7, 8, 9]);
}

//#[test]
pub fn transfer_oob_with_same_memory_errors() {
    let src = MemoryRef(Arc::new(create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
    assert!(MemoryInstance::transfer(&src, 65535, &src, 0, 3).is_err());

    // Check that memories content left untouched
    assert_eq!(src.get(0, 10).unwrap(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

//#[test]
pub fn transfer_oob_errors() {
    let src = MemoryRef(Arc::new(create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
    let dst = MemoryRef(Arc::new(create_memory(&[
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ])));

    assert!(MemoryInstance::transfer(&src, 65535, &dst, 0, 3).is_err());

    // Check that memories content left untouched
    assert_eq!(src.get(0, 10).unwrap(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(
        dst.get(0, 10).unwrap(),
        &[10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    );
}

//#[test]
pub fn clear() {
    let mem = create_memory(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    mem.clear(0, 0x4A, 10)
        .expect("To successfully clear the memory");
    let result = mem.get(0, 10).expect("To successfully retrieve the result");
    assert_eq!(result, &[0x4A; 10]);
}

//#[test]
pub fn get_into() {
    let mem = MemoryInstance::new(Pages(1), None).unwrap();
    mem.set(6, &[13, 17, 129])
        .expect("memory set should not fail");

    let mut data = [0u8; 2];
    mem.get_into(7, &mut data[..])
        .expect("get_into should not fail");

    assert_eq!(data, [17, 129]);
}

//#[test]
pub fn zero_copy() {
    let mem = MemoryInstance::alloc(Pages(1), None).unwrap();
    mem.set(100, &[0]).expect("memory set should not fail");
    mem.with_direct_access_mut(|buf| {
        assert_eq!(
            buf.len(),
            65536,
            "the buffer length is expected to be 1 page long"
        );
        buf[..10].copy_from_slice(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    });
    mem.with_direct_access(|buf| {
        assert_eq!(
            buf.len(),
            65536,
            "the buffer length is expected to be 1 page long"
        );
        assert_eq!(&buf[..10], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    });
}

//#[should_panic]
//#[test]
pub fn zero_copy_panics_on_nested_access() {
    let mem = MemoryInstance::alloc(Pages(1), None).unwrap();
    let mem_inner = mem.clone();
    mem.with_direct_access(move |_| {
        let _ = mem_inner.set(0, &[11, 12, 13]);
    });
}
