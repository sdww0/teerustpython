use core::iter::FromIterator;
use std::prelude::v1::*;
use parity_wasm::io;
use parity_wasm::elements::*;

//#[test]
pub fn default_is_empty_no_matter_how_we_look_at_it() {
	let map = IndexMap::<String>::default();
	assert_eq!(map.len(), 0);
	assert!(map.is_empty());
	assert_eq!(map.iter().collect::<Vec<_>>().len(), 0);
	assert_eq!(map.into_iter().collect::<Vec<_>>().len(), 0);
}

//#[test]
pub fn with_capacity_creates_empty_map() {
	let map = IndexMap::<String>::with_capacity(10);
	assert!(map.is_empty());
}

//#[test]
pub fn clear_removes_all_values() {
	let mut map = IndexMap::<String>::default();
	map.insert(0, "sample value".to_string());
	assert_eq!(map.len(), 1);
	map.clear();
	assert_eq!(map.len(), 0);
}

//#[test]
pub fn get_returns_elements_that_are_there_but_nothing_else() {
	let mut map = IndexMap::<String>::default();
	map.insert(1, "sample value".to_string());
	assert_eq!(map.len(), 1);
	assert_eq!(map.get(0), None);
	assert_eq!(map.get(1), Some(&"sample value".to_string()));
	assert_eq!(map.get(2), None);
}

//#[test]
pub fn contains_key_returns_true_when_a_key_is_present() {
	let mut map = IndexMap::<String>::default();
	map.insert(1, "sample value".to_string());
	assert!(!map.contains_key(0));
	assert!(map.contains_key(1));
	assert!(!map.contains_key(2));
}

//#[test]
pub fn insert_behaves_like_other_maps() {
	let mut map = IndexMap::<String>::default();

	// Insert a key which requires extending our storage.
	assert_eq!(map.insert(1, "val 1".to_string()), None);
	assert_eq!(map.len(), 1);
	assert!(map.contains_key(1));

	// Insert a key which requires filling in a hole.
	assert_eq!(map.insert(0, "val 0".to_string()), None);
	assert_eq!(map.len(), 2);
	assert!(map.contains_key(0));

	// Insert a key which replaces an existing key.
	assert_eq!(
		map.insert(1, "val 1.1".to_string()),
		Some("val 1".to_string())
	);
	assert_eq!(map.len(), 2);
	assert!(map.contains_key(1));
	assert_eq!(map.get(1), Some(&"val 1.1".to_string()));
}

//#[test]
pub fn remove_behaves_like_other_maps() {
	let mut map = IndexMap::<String>::default();
	assert_eq!(map.insert(1, "val 1".to_string()), None);

	// Remove an out-of-bounds element.
	assert_eq!(map.remove(2), None);
	assert_eq!(map.len(), 1);

	// Remove an in-bounds but missing element.
	assert_eq!(map.remove(0), None);
	assert_eq!(map.len(), 1);

	// Remove an existing element.
	assert_eq!(map.remove(1), Some("val 1".to_string()));
	assert_eq!(map.len(), 0);
}

//#[test]
pub fn partial_eq_works_as_expected_in_simple_cases() {
	let mut map1 = IndexMap::<String>::default();
	let mut map2 = IndexMap::<String>::default();
	assert_eq!(map1, map2);

	map1.insert(1, "a".to_string());
	map2.insert(1, "a".to_string());
	assert_eq!(map1, map2);

	map1.insert(0, "b".to_string());
	assert_ne!(map1, map2);
	map1.remove(0);
	assert_eq!(map1, map2);

	map1.insert(1, "not a".to_string());
	assert_ne!(map1, map2);
}

//#[test]
pub fn partial_eq_is_smart_about_none_values_at_the_end() {
	let mut map1 = IndexMap::<String>::default();
	let mut map2 = IndexMap::<String>::default();

	map1.insert(1, "a".to_string());
	map2.insert(1, "a".to_string());

	// Both maps have the same (idx, value) pairs, but map2 has extra space.
	map2.insert(10, "b".to_string());
	map2.remove(10);
	assert_eq!(map1, map2);

	// Both maps have the same (idx, value) pairs, but map1 has extra space.
	map1.insert(100, "b".to_string());
	map1.remove(100);
	assert_eq!(map1, map2);

	// Let's be paranoid.
	map2.insert(1, "b".to_string());
	assert_ne!(map1, map2);
}

//#[test]
pub fn from_iterator_builds_a_map() {
	let data = &[
		// We support out-of-order values here!
		(3, "val 3"),
		(2, "val 2"),
		(5, "val 5"),
	];
	let iter = data.iter().map(|&(idx, val)| (idx, val.to_string()));
	let map = IndexMap::from_iter(iter);
	assert_eq!(map.len(), 3);
	assert_eq!(map.get(2), Some(&"val 2".to_string()));
	assert_eq!(map.get(3), Some(&"val 3".to_string()));
	assert_eq!(map.get(5), Some(&"val 5".to_string()));
}

//#[test]
pub fn iterators_are_well_behaved() {
	// Create a map with reasonably complex internal structure, making
	// sure that we have both internal missing elements, and a bunch of
	// missing elements at the end.
	let data = &[(3, "val 3"), (2, "val 2"), (5, "val 5")];
	let src_iter = data.iter().map(|&(idx, val)| (idx, val.to_string()));
	let mut map = IndexMap::from_iter(src_iter);
	map.remove(5);

	// Make sure `size_hint` and `next` behave as we expect at each step.
	{
		let mut iter1 = map.iter();
		assert_eq!(iter1.size_hint(), (2, Some(2)));
		assert_eq!(iter1.next(), Some((2, &"val 2".to_string())));
		assert_eq!(iter1.size_hint(), (1, Some(1)));
		assert_eq!(iter1.next(), Some((3, &"val 3".to_string())));
		assert_eq!(iter1.size_hint(), (0, Some(0)));
		assert_eq!(iter1.next(), None);
		assert_eq!(iter1.size_hint(), (0, Some(0)));
		assert_eq!(iter1.next(), None);
		assert_eq!(iter1.size_hint(), (0, Some(0)));
	}

	// Now do the same for a consuming iterator.
	let mut iter2 = map.into_iter();
	assert_eq!(iter2.size_hint(), (2, Some(2)));
	assert_eq!(iter2.next(), Some((2, "val 2".to_string())));
	assert_eq!(iter2.size_hint(), (1, Some(1)));
	assert_eq!(iter2.next(), Some((3, "val 3".to_string())));
	assert_eq!(iter2.size_hint(), (0, Some(0)));
	assert_eq!(iter2.next(), None);
	assert_eq!(iter2.size_hint(), (0, Some(0)));
	assert_eq!(iter2.next(), None);
	assert_eq!(iter2.size_hint(), (0, Some(0)));
}

//#[test]
pub fn serialize_and_deserialize() {
	let mut map = IndexMap::<String>::default();
	map.insert(1, "val 1".to_string());

	let mut output = vec![];
	map.clone()
		.serialize(&mut output)
		.expect("serialize failed");

	let mut input = io::Cursor::new(&output);
	let deserialized = IndexMap::deserialize(2, &mut input).expect("deserialize failed");

	assert_eq!(deserialized, map);
}

//#[test]
pub fn deserialize_requires_elements_to_be_in_order() {
	// Build a in-order example by hand.
	let mut valid = vec![];
	VarUint32::from(2u32).serialize(&mut valid).unwrap();
	VarUint32::from(0u32).serialize(&mut valid).unwrap();
	"val 0".to_string().serialize(&mut valid).unwrap();
	VarUint32::from(1u32).serialize(&mut valid).unwrap();
	"val 1".to_string().serialize(&mut valid).unwrap();
	let map = IndexMap::<String>::deserialize(2, &mut io::Cursor::new(valid))
		.expect("unexpected error deserializing");
	assert_eq!(map.len(), 2);

	// Build an out-of-order example by hand.
	let mut invalid = vec![];
	VarUint32::from(2u32).serialize(&mut invalid).unwrap();
	VarUint32::from(1u32).serialize(&mut invalid).unwrap();
	"val 1".to_string().serialize(&mut invalid).unwrap();
	VarUint32::from(0u32).serialize(&mut invalid).unwrap();
	"val 0".to_string().serialize(&mut invalid).unwrap();
	let res = IndexMap::<String>::deserialize(2, &mut io::Cursor::new(invalid));
	assert!(res.is_err());
}

//#[test]
pub fn deserialize_enforces_max_idx() {
	// Build an example with an out-of-bounds index by hand.
	let mut invalid = vec![];
	VarUint32::from(1u32).serialize(&mut invalid).unwrap();
	VarUint32::from(5u32).serialize(&mut invalid).unwrap();
	"val 5".to_string().serialize(&mut invalid).unwrap();
	let res = IndexMap::<String>::deserialize(1, &mut io::Cursor::new(invalid));
	assert!(res.is_err());
}
