//#[test]
use bytes::bytes_mut::*;

pub fn test_original_capacity_to_repr() {
    assert_eq!(original_capacity_to_repr(0), 0);

    let max_width = 32;

    for width in 1..(max_width + 1) {
        let cap = 1 << width - 1;

        let expected = if width < MIN_ORIGINAL_CAPACITY_WIDTH {
            0
        } else if width < MAX_ORIGINAL_CAPACITY_WIDTH {
            width - MIN_ORIGINAL_CAPACITY_WIDTH
        } else {
            MAX_ORIGINAL_CAPACITY_WIDTH - MIN_ORIGINAL_CAPACITY_WIDTH
        };

        assert_eq!(original_capacity_to_repr(cap), expected);

        if width > 1 {
            assert_eq!(original_capacity_to_repr(cap + 1), expected);
        }

        //  MIN_ORIGINAL_CAPACITY_WIDTH must be bigger than 7 to pass tests below
        if width == MIN_ORIGINAL_CAPACITY_WIDTH + 1 {
            assert_eq!(original_capacity_to_repr(cap - 24), expected - 1);
            assert_eq!(original_capacity_to_repr(cap + 76), expected);
        } else if width == MIN_ORIGINAL_CAPACITY_WIDTH + 2 {
            assert_eq!(original_capacity_to_repr(cap - 1), expected - 1);
            assert_eq!(original_capacity_to_repr(cap - 48), expected - 1);
        }
    }
}

//#[test]
pub fn test_original_capacity_from_repr() {
    assert_eq!(0, original_capacity_from_repr(0));

    let min_cap = 1 << MIN_ORIGINAL_CAPACITY_WIDTH;

    assert_eq!(min_cap, original_capacity_from_repr(1));
    assert_eq!(min_cap * 2, original_capacity_from_repr(2));
    assert_eq!(min_cap * 4, original_capacity_from_repr(3));
    assert_eq!(min_cap * 8, original_capacity_from_repr(4));
    assert_eq!(min_cap * 16, original_capacity_from_repr(5));
    assert_eq!(min_cap * 32, original_capacity_from_repr(6));
    assert_eq!(min_cap * 64, original_capacity_from_repr(7));
}
