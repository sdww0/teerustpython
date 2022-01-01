use std::prelude::v1::*;
use std::collections::{
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    HashMap,
    HashSet,
    LinkedList,
    VecDeque,
};
use std::fmt::Debug;
use std::hash::Hash;
use std::num::Wrapping;
use std::path::PathBuf;
use quickcheck::Arbitrary;

//#[test]
pub fn arby_unit() {
    assert_eq!(arby::<()>(), ());
}

//#[test]
pub fn arby_int() {
    rep(&mut || { let n: isize = arby(); assert!(n >= -5 && n <= 5); } );
}

//#[test]
pub fn arby_uint() {
    rep(&mut || { let n: usize = arby(); assert!(n <= 5); } );
}

fn arby<A: quickcheck::Arbitrary>() -> A {
    quickcheck::Arbitrary::arbitrary(&mut gen())
}

fn gen() -> quickcheck::StdGen<rand::rngs::ThreadRng> {
    quickcheck::StdGen::new(rand::thread_rng(), 5)
}

fn rep<F>(f: &mut F) where F : FnMut() -> () {
    for _ in 0..100 {
        f()
    }
}

// Shrink testing.
//#[test]
pub fn unit() {
    eq((), vec![]);
}

//#[test]
pub fn bools() {
    eq(false, vec![]);
    eq(true, vec![false]);
}

//#[test]
pub fn options() {
    eq(None::<()>, vec![]);
    eq(Some(false), vec![None]);
    eq(Some(true), vec![None, Some(false)]);
}

//#[test]
pub fn results() {
    // Result<A, B> doesn't implement the Hash trait, so these tests
    // depends on the order of shrunk results. Ug.
    // TODO: Fix this.
    ordered_eq(Ok::<bool, ()>(true), vec![Ok(false)]);
    ordered_eq(Err::<(), bool>(true), vec![Err(false)]);
}

//#[test]
pub fn tuples() {
    eq((false, false), vec![]);
    eq((true, false), vec![(false, false)]);
    eq((true, true), vec![(false, true), (true, false)]);
}

//#[test]
pub fn triples() {
    eq((false, false, false), vec![]);
    eq((true, false, false), vec![(false, false, false)]);
    eq((true, true, false),
       vec![(false, true, false), (true, false, false)]);
}

//#[test]
pub fn quads() {
    eq((false, false, false, false), vec![]);
    eq((true, false, false, false), vec![(false, false, false, false)]);
    eq((true, true, false, false),
        vec![(false, true, false, false), (true, false, false, false)]);
}

//#[test]
pub fn ints() {
    // TODO: Test overflow?
    eq(5isize, vec![0, 3, 4]);
    eq(-5isize, vec![5, 0, -3, -4]);
    eq(0isize, vec![]);
}

//#[test]
pub fn ints8() {
    eq(5i8, vec![0, 3, 4]);
    eq(-5i8, vec![5, 0, -3, -4]);
    eq(0i8, vec![]);
}

//#[test]
pub fn ints16() {
    eq(5i16, vec![0, 3, 4]);
    eq(-5i16, vec![5, 0, -3, -4]);
    eq(0i16, vec![]);
}

//#[test]
pub fn ints32() {
    eq(5i32, vec![0, 3, 4]);
    eq(-5i32, vec![5, 0, -3, -4]);
    eq(0i32, vec![]);
}

//#[test]
pub fn ints64() {
    eq(5i64, vec![0, 3, 4]);
    eq(-5i64, vec![5, 0, -3, -4]);
    eq(0i64, vec![]);
}

//#[test]
pub fn ints128() {
    eq(5i128, vec![0, 3, 4]);
    eq(-5i128, vec![5, 0, -3, -4]);
    eq(0i128, vec![]);
}

//#[test]
pub fn uints() {
    eq(5usize, vec![0, 3, 4]);
    eq(0usize, vec![]);
}

//#[test]
pub fn uints8() {
    eq(5u8, vec![0, 3, 4]);
    eq(0u8, vec![]);
}

//#[test]
pub fn uints16() {
    eq(5u16, vec![0, 3, 4]);
    eq(0u16, vec![]);
}

//#[test]
pub fn uints32() {
    eq(5u32, vec![0, 3, 4]);
    eq(0u32, vec![]);
}

//#[test]
pub fn uints64() {
    eq(5u64, vec![0, 3, 4]);
    eq(0u64, vec![]);
}

//#[test]
pub fn uints128() {
    eq(5u128, vec![0, 3, 4]);
    eq(0u128, vec![]);
}

macro_rules! define_float_eq {
    ($ty:ty) => {
        fn eq(s:$ty, v: Vec<$ty> ) {
            let shrunk: Vec<$ty> = s.shrink().collect();
            for n in v {
                let found = shrunk.iter().any(|&i| i == n);
                if !found {
                    panic!(format!(
                        "Element {:?} was not found \
                         in shrink results {:?}",
                        n, shrunk));
                }
            }
        }
    }
}

//#[test]
pub fn floats32() {
    define_float_eq!(f32);

    eq(0.0, vec![]);
    eq(-0.0, vec![]);
    eq(1.0, vec![0.0]);
    eq(2.0, vec![0.0, 1.0]);
    eq(-2.0, vec![0.0, 2.0, -1.0]);
    eq(1.5, vec![0.0]);
}

//#[test]
pub fn floats64() {
    define_float_eq!(f64);

    eq(0.0, vec![]);
    eq(-0.0, vec![]);
    eq(1.0, vec![0.0]);
    eq(2.0, vec![0.0, 1.0]);
    eq(-2.0, vec![0.0, 2.0, -1.0]);
    eq(1.5, vec![0.0]);
}

//#[test]
pub fn wrapping_ints32() {
    eq(Wrapping(5i32), vec![Wrapping(0), Wrapping(3), Wrapping(4)]);
    eq(Wrapping(-5i32), vec![Wrapping(5), Wrapping(0), Wrapping(-3), Wrapping(-4)]);
    eq(Wrapping(0i32), vec![]);
}

//#[test]
pub fn vecs() {
    eq({let it: Vec<isize> = vec![]; it}, vec![]);
    eq({let it: Vec<Vec<isize>> = vec![vec![]]; it}, vec![vec![]]);
    eq(vec![1isize], vec![vec![], vec![0]]);
    eq(vec![11isize], vec![vec![], vec![0], vec![6], vec![9], vec![10]]);
    eq(
        vec![3isize, 5],
        vec![vec![], vec![5], vec![3], vec![0,5], vec![2,5],
             vec![3,0], vec![3,3], vec![3,4]]
    );
}

macro_rules! map_tests {
    ($name:ident, $ctor:expr) => {
        //#[test]
        pub fn $name() {
            ordered_eq($ctor, vec![]);

            {
                let mut map = $ctor;
                map.insert(1usize, 1isize);

                let shrinks = vec![
                    $ctor,
                    {let mut m = $ctor; m.insert(0, 1); m},
                    {let mut m = $ctor; m.insert(1, 0); m},
                ];

                ordered_eq(map, shrinks);
            }
        }
    }
}

map_tests!(btreemap, BTreeMap::<usize, isize>::new());
map_tests!(hashmap, HashMap::<usize, isize>::new());

macro_rules! list_tests {
    ($name:ident, $ctor:expr, $push:ident) => {
       // #[test]
        pub fn $name() {
            ordered_eq($ctor, vec![]);

            {
                let mut list = $ctor;
                list.$push(2usize);

                let shrinks = vec![
                    $ctor,
                    {let mut m = $ctor; m.$push(0); m},
                    {let mut m = $ctor; m.$push(1); m},
                ];

                ordered_eq(list, shrinks);
            }
        }
    }
}

list_tests!(btreesets, BTreeSet::<usize>::new(), insert);
list_tests!(hashsets, HashSet::<usize>::new(), insert);
list_tests!(linkedlists, LinkedList::<usize>::new(), push_back);
list_tests!(vecdeques, VecDeque::<usize>::new(), push_back);

//#[test]
pub fn binaryheaps() {
    ordered_eq(
        BinaryHeap::<usize>::new().into_iter().collect::<Vec<_>>(),
        vec![]);

    {
        let mut heap = BinaryHeap::<usize>::new();
        heap.push(2usize);

        let shrinks = vec![
            vec![],
            vec![0],
            vec![1],
        ];

        ordered_eq(heap.into_iter().collect::<Vec<_>>(), shrinks);
    }
}

//#[test]
pub fn chars() {
    eq('\x00', vec![]);
}

// All this jazz is for testing set equality on the results of a shrinker.
fn eq<A: Arbitrary + Eq + Debug + Hash>(s: A, v: Vec<A>) {
    let (left, right) = (shrunk(s), set(v));
    assert_eq!(left, right);
}
fn shrunk<A: Arbitrary + Eq + Hash>(s: A) -> HashSet<A> {
    set(s.shrink().collect())
}
fn set<A: Eq + Hash>(xs: Vec<A>) -> HashSet<A> {
    xs.into_iter().collect()
}

fn ordered_eq<A: Arbitrary + Eq + Debug>(s: A, v: Vec<A>) {
    let (left, right) = (s.shrink().collect::<Vec<A>>(), v);
    assert_eq!(left, right);
}

//#[test]
pub fn bounds() {
    use std::ops::Bound::*;
    for i in -5..=5 {
        ordered_eq(Included(i), i.shrink().map(Included).collect());
        ordered_eq(Excluded(i), i.shrink().map(Excluded).collect());
    }
    eq(Unbounded::<i32>, vec![]);
}

//#[test]
pub fn ranges() {
    ordered_eq(0..0, vec![]);
    ordered_eq(1..1, vec![0..1, 1..0]);
    ordered_eq(3..5, vec![0..5, 2..5, 3..0, 3..3, 3..4]);
    ordered_eq(5..3, vec![0..3, 3..3, 4..3, 5..0, 5..2]);
    ordered_eq(3.., vec![0.., 2..]);
    ordered_eq(..3, vec![..0, ..2]);
    ordered_eq(.., vec![]);
}

//#[test]
pub fn pathbuf() {
    ordered_eq(PathBuf::from("/home/foo//.././bar"), vec![
        PathBuf::from("/home/foo//.."),
        PathBuf::from("/home/foo/../bar"),
    ]);
}
