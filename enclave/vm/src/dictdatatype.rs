use crate::obj::objstr::{PyString, PyStringRef};
use crate::pyhash;
use crate::pyobject::{IdProtocol, IntoPyObject, PyObjectRef, PyResult};
use crate::vm::VirtualMachine;
use num_bigint::ToBigInt;
/// Ordered dictionary implementation.
/// Inspired by: https://morepypy.blogspot.com/2015/01/faster-more-memory-efficient-and-more.html
/// And: https://www.youtube.com/watch?v=p33CVV29OG8
/// And: http://code.activestate.com/recipes/578375/
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::mem::size_of;
use std::sync::{SgxRwLock as RwLock,SgxRwLockReadGuard as  RwLockReadGuard,SgxRwLockWriteGuard as RwLockWriteGuard};
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::borrow::ToOwned;

/// hash value of an object returned by __hash__
type HashValue = pyhash::PyHash;
/// index calculated by resolving collision
type HashIndex = pyhash::PyHash;
/// entry index mapped in indices
type EntryIndex = usize;

pub struct Dict<T = PyObjectRef> {
    inner: RwLock<InnerDict<T>>,
}

struct InnerDict<T> {
    size: usize,
    indices: HashMap<HashIndex, EntryIndex>,
    entries: Vec<Option<DictEntry<T>>>,
}

impl<T: Clone> Clone for InnerDict<T> {
    fn clone(&self) -> Self {
        InnerDict {
            size: self.size,
            indices: self.indices.clone(),
            entries: self.entries.clone(),
        }
    }
}

impl<T: Clone> Clone for Dict<T> {
    fn clone(&self) -> Self {
        Dict {
            inner: RwLock::new(self.inner.read().unwrap().clone()),
        }
    }
}

impl<T> Default for Dict<T> {
    fn default() -> Self {
        Dict {
            inner: RwLock::new(InnerDict {
                size: 0,
                indices: HashMap::new(),
                entries: Vec::new(),
            }),
        }
    }
}

struct DictEntry<T> {
    hash: HashValue,
    key: PyObjectRef,
    value: T,
}

impl<T: Clone> Clone for DictEntry<T> {
    fn clone(&self) -> Self {
        DictEntry {
            hash: self.hash,
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct DictSize {
    size: usize,
    entries_size: usize,
}

impl<T: Clone> Dict<T> {
    fn borrow_value(&self) -> RwLockReadGuard<'_, InnerDict<T>> {
        self.inner.read().unwrap()
    }

    fn borrow_value_mut(&self) -> RwLockWriteGuard<'_, InnerDict<T>> {
        self.inner.write().unwrap()
    }

    fn resize(&self) {
        let mut inner = self.borrow_value_mut();
        let mut new_indices = HashMap::with_capacity(inner.size);
        let mut new_entries = Vec::with_capacity(inner.size);
        for maybe_entry in inner.entries.drain(0..) {
            if let Some(entry) = maybe_entry {
                let mut hash_index = entry.hash;
                // Faster version of lookup. No equality checks here.
                // We assume dict doesn't contatins any duplicate keys
                while new_indices.contains_key(&hash_index) {
                    hash_index = Self::next_index(entry.hash, hash_index);
                }
                new_indices.insert(hash_index, new_entries.len());
                new_entries.push(Some(entry));
            }
        }
        inner.indices = new_indices;
        inner.entries = new_entries;
    }

    fn unchecked_push(
        &self,
        hash_index: HashIndex,
        hash_value: HashValue,
        key: PyObjectRef,
        value: T,
    ) {
        let entry = DictEntry {
            hash: hash_value,
            key,
            value,
        };
        let mut inner = self.borrow_value_mut();
        let entry_index = inner.entries.len();
        inner.entries.push(Some(entry));
        inner.indices.insert(hash_index, entry_index);
        inner.size += 1;
    }

    /// Store a key
    pub fn insert<K: DictKey + IntoPyObject + Copy>(
        &self,
        vm: &VirtualMachine,
        key: K,
        value: T,
    ) -> PyResult<()> {
        // This does not need to be accurate so we can take the lock mutiple times.
        if self.borrow_value().indices.len() > 2 * self.borrow_value().size {
            self.resize();
        }
        loop {
            match self.lookup(vm, key)? {
                LookupResult::Existing(index) => {
                    let mut inner = self.borrow_value_mut();
                    // Update existing key
                    if let Some(ref mut entry) = inner.entries[index] {
                        // They entry might have changed since we did lookup. Should we update the key?
                        entry.value = value;
                        break Ok(());
                    } else {
                        // The dict was changed since we did lookup. Let's try again.
                        continue;
                    }
                }
                LookupResult::NewIndex {
                    hash_index,
                    hash_value,
                } => {
                    // New key:
                    self.unchecked_push(hash_index, hash_value, key.into_pyobject(vm)?, value);
                    break Ok(());
                }
            }
        }
    }

    pub fn contains<K: DictKey + Copy>(&self, vm: &VirtualMachine, key: K) -> PyResult<bool> {
        if let LookupResult::Existing(_) = self.lookup(vm, key)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Retrieve a key
    #[cfg_attr(feature = "flame-it", flame("Dict"))]
    pub fn get<K: DictKey + Copy>(&self, vm: &VirtualMachine, key: K) -> PyResult<Option<T>> {
        loop {
            if let LookupResult::Existing(index) = self.lookup(vm, key)? {
                if let Some(entry) = &self.borrow_value().entries[index] {
                    break Ok(Some(entry.value.clone()));
                } else {
                    // The dict was changed since we did lookup. Let's try again.
                    continue;
                }
            } else {
                break Ok(None);
            }
        }
    }

    pub fn clear(&self) {
        let mut inner = self.borrow_value_mut();
        inner.entries.clear();
        inner.indices.clear();
        inner.size = 0
    }

    /// Delete a key
    pub fn delete(&self, vm: &VirtualMachine, key: &PyObjectRef) -> PyResult<()> {
        if self.delete_if_exists(vm, key)? {
            Ok(())
        } else {
            Err(vm.new_key_error(key.clone()))
        }
    }

    pub fn delete_if_exists(&self, vm: &VirtualMachine, key: &PyObjectRef) -> PyResult<bool> {
        loop {
            if let LookupResult::Existing(entry_index) = self.lookup(vm, key)? {
                let mut inner = self.borrow_value_mut();
                if inner.entries[entry_index].is_some() {
                    inner.entries[entry_index] = None;
                    inner.size -= 1;
                    break Ok(true);
                } else {
                    // The dict was changed since we did lookup. Let's try again.
                    continue;
                }
            } else {
                break Ok(false);
            }
        }
    }

    pub fn delete_or_insert(
        &self,
        vm: &VirtualMachine,
        key: &PyObjectRef,
        value: T,
    ) -> PyResult<()> {
        loop {
            match self.lookup(vm, key)? {
                LookupResult::Existing(entry_index) => {
                    let mut inner = self.borrow_value_mut();
                    if inner.entries[entry_index].is_some() {
                        inner.entries[entry_index] = None;
                        inner.size -= 1;
                        break Ok(());
                    } else {
                        // The dict was changed since we did lookup. Let's try again.
                        continue;
                    }
                }
                LookupResult::NewIndex {
                    hash_value,
                    hash_index,
                } => {
                    self.unchecked_push(hash_index, hash_value, key.clone(), value);
                    break Ok(());
                }
            };
        }
    }

    pub fn len(&self) -> usize {
        self.borrow_value().size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn size(&self) -> DictSize {
        let inner = self.borrow_value();
        DictSize {
            size: inner.size,
            entries_size: inner.entries.len(),
        }
    }

    pub fn next_entry(&self, position: &mut EntryIndex) -> Option<(PyObjectRef, T)> {
        self.borrow_value().entries[*position..]
            .iter()
            .find_map(|entry| {
                *position += 1;
                entry
                    .as_ref()
                    .map(|DictEntry { key, value, .. }| (key.clone(), value.clone()))
            })
    }

    pub fn len_from_entry_index(&self, position: EntryIndex) -> usize {
        self.borrow_value().entries[position..]
            .iter()
            .flatten()
            .count()
    }

    pub fn has_changed_size(&self, position: &DictSize) -> bool {
        let inner = self.borrow_value();
        position.size != inner.size || inner.entries.len() != position.entries_size
    }

    pub fn keys(&self) -> Vec<PyObjectRef> {
        self.borrow_value()
            .entries
            .iter()
            .filter_map(|v| v.as_ref().map(|v| v.key.clone()))
            .collect()
    }

    /// Lookup the index for the given key.
    #[cfg_attr(feature = "flame-it", flame("Dict"))]
    fn lookup<K: DictKey + Copy>(&self, vm: &VirtualMachine, key: K) -> PyResult<LookupResult> {
        let hash_value = key.do_hash(vm)?;
        let perturb = hash_value;
        let mut hash_index: HashIndex = hash_value;
        'outer: loop {
            let (entry, index) = loop {
                let inner = self.borrow_value();
                if inner.indices.contains_key(&hash_index) {
                    // Now we have an index, lets check the key.
                    let index = inner.indices[&hash_index];
                    if let Some(entry) = &inner.entries[index] {
                        // Okay, we have an entry at this place
                        if key.do_is(&entry.key) {
                            // Literally the same object
                            break 'outer Ok(LookupResult::Existing(index));
                        } else if entry.hash == hash_value {
                            break (entry.clone(), index);
                        } else {
                            // entry mismatch.
                        }
                    } else {
                        // Removed entry, continue search...
                    }
                } else {
                    // Hash not in table, we are at free slot now.
                    break 'outer Ok(LookupResult::NewIndex {
                        hash_value,
                        hash_index,
                    });
                }
                // Update i to next probe location:
                hash_index = Self::next_index(perturb, hash_index)
                // warn!("Perturb value: {}", i);
            };
            // This comparison needs to be done outside the lock.
            if key.do_eq(vm, &entry.key)? {
                break Ok(LookupResult::Existing(index));
            } else {
                // entry mismatch.
            }

            // Update i to next probe location:
            hash_index = Self::next_index(perturb, hash_index)
            // warn!("Perturb value: {}", i);
        }
    }

    fn next_index(perturb: HashValue, hash_index: HashIndex) -> HashIndex {
        hash_index
            .wrapping_mul(5)
            .wrapping_add(perturb)
            .wrapping_add(1)
    }

    /// Retrieve and delete a key
    pub fn pop<K: DictKey + Copy>(&self, vm: &VirtualMachine, key: K) -> PyResult<Option<T>> {
        loop {
            if let LookupResult::Existing(index) = self.lookup(vm, key)? {
                let mut inner = self.borrow_value_mut();
                if let Some(entry) = &inner.entries[index] {
                    let value = entry.value.clone();
                    inner.entries[index] = None;
                    inner.size -= 1;
                    break Ok(Some(value));
                } else {
                    // The dict was changed since we did lookup. Let's try again.
                    continue;
                }
            } else {
                break Ok(None);
            }
        }
    }

    pub fn pop_front(&self) -> Option<(PyObjectRef, T)> {
        let mut position = 0;
        let mut inner = self.borrow_value_mut();
        let first_item = inner.entries.iter().find_map(|entry| {
            position += 1;
            entry
                .as_ref()
                .map(|DictEntry { key, value, .. }| (key.clone(), value.clone()))
        });
        if let Some(item) = first_item {
            inner.entries[position - 1] = None;
            inner.size -= 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn sizeof(&self) -> usize {
        size_of::<Self>() + self.borrow_value().size * size_of::<DictEntry<T>>()
    }
}

enum LookupResult {
    NewIndex {
        hash_value: HashValue,
        hash_index: HashIndex,
    }, // return not found, index into indices
    Existing(EntryIndex), // Existing record, index into entries
}

/// Types implementing this trait can be used to index
/// the dictionary. Typical usecases are:
/// - PyObjectRef -> arbitrary python type used as key
/// - str -> string reference used as key, this is often used internally
pub trait DictKey {
    fn do_hash(self, vm: &VirtualMachine) -> PyResult<HashValue>;
    fn do_is(self, other: &PyObjectRef) -> bool;
    fn do_eq(self, vm: &VirtualMachine, other_key: &PyObjectRef) -> PyResult<bool>;
}

/// Implement trait for PyObjectRef such that we can use python objects
/// to index dictionaries.
impl DictKey for &PyObjectRef {
    fn do_hash(self, vm: &VirtualMachine) -> PyResult<HashValue> {
        let raw_hash = vm._hash(self)?;
        let mut hasher = DefaultHasher::new();
        raw_hash.hash(&mut hasher);
        Ok(hasher.finish() as HashValue)
    }

    fn do_is(self, other: &PyObjectRef) -> bool {
        self.is(other)
    }

    fn do_eq(self, vm: &VirtualMachine, other_key: &PyObjectRef) -> PyResult<bool> {
        vm.identical_or_equal(self, other_key)
    }
}

impl DictKey for &PyStringRef {
    fn do_hash(self, _vm: &VirtualMachine) -> PyResult<HashValue> {
        Ok(self.hash())
    }

    fn do_is(self, other: &PyObjectRef) -> bool {
        self.is(other)
    }

    fn do_eq(self, vm: &VirtualMachine, other_key: &PyObjectRef) -> PyResult<bool> {
        if self.is(other_key) {
            Ok(true)
        } else if let Some(py_str_value) = other_key.payload::<PyString>() {
            Ok(py_str_value.as_str() == self.as_str())
        } else {
            vm.bool_eq(self.clone().into_object(), other_key.clone())
        }
    }
}

/// Implement trait for the str type, so that we can use strings
/// to index dictionaries.
impl DictKey for &str {
    fn do_hash(self, _vm: &VirtualMachine) -> PyResult<HashValue> {
        // follow a similar route as the hashing of PyStringRef
        let raw_hash = pyhash::hash_value(&self.to_owned()).to_bigint().unwrap();
        let raw_hash = pyhash::hash_bigint(&raw_hash);
        let mut hasher = DefaultHasher::new();
        raw_hash.hash(&mut hasher);
        Ok(hasher.finish() as HashValue)
    }

    fn do_is(self, _other: &PyObjectRef) -> bool {
        // No matter who the other pyobject is, we are never the same thing, since
        // we are a str, not a pyobject.
        false
    }

    fn do_eq(self, vm: &VirtualMachine, other_key: &PyObjectRef) -> PyResult<bool> {
        if let Some(py_str_value) = other_key.payload::<PyString>() {
            Ok(py_str_value.as_str() == self)
        } else {
            // Fall back to PyString implementation.
            let s = vm.new_str(self.to_owned());
            s.do_eq(vm, other_key)
        }
    }
}

impl DictKey for &String {
    fn do_hash(self, vm: &VirtualMachine) -> PyResult<HashValue> {
        self.as_str().do_hash(vm)
    }

    fn do_is(self, other: &PyObjectRef) -> bool {
        self.as_str().do_is(other)
    }

    fn do_eq(self, vm: &VirtualMachine, other_key: &PyObjectRef) -> PyResult<bool> {
        self.as_str().do_eq(vm, other_key)
    }
}

#[cfg(test)]
mod tests {
    use super::{Dict, DictKey, VirtualMachine};

    #[test]
    fn test_insert() {
        let vm: VirtualMachine = Default::default();
        let dict = Dict::default();
        assert_eq!(0, dict.len());

        let key1 = vm.new_bool(true);
        let value1 = vm.new_str("abc".to_owned());
        dict.insert(&vm, &key1, value1.clone()).unwrap();
        assert_eq!(1, dict.len());

        let key2 = vm.new_str("x".to_owned());
        let value2 = vm.new_str("def".to_owned());
        dict.insert(&vm, &key2, value2.clone()).unwrap();
        assert_eq!(2, dict.len());

        dict.insert(&vm, &key1, value2.clone()).unwrap();
        assert_eq!(2, dict.len());

        dict.delete(&vm, &key1).unwrap();
        assert_eq!(1, dict.len());

        dict.insert(&vm, &key1, value2.clone()).unwrap();
        assert_eq!(2, dict.len());

        assert_eq!(true, dict.contains(&vm, &key1).unwrap());
        assert_eq!(true, dict.contains(&vm, "x").unwrap());

        let val = dict.get(&vm, "x").unwrap().unwrap();
        vm.bool_eq(val, value2)
            .expect("retrieved value must be equal to inserted value.");
    }

    macro_rules! hash_tests {
        ($($name:ident: $example_hash:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    check_hash_equivalence($example_hash);
                }
            )*
        }
    }

    hash_tests! {
        test_abc: "abc",
        test_x: "x",
    }

    fn check_hash_equivalence(text: &str) {
        let vm: VirtualMachine = Default::default();
        let value1 = text;
        let value2 = vm.new_str(value1.to_owned());

        let hash1 = value1.do_hash(&vm).expect("Hash should not fail.");
        let hash2 = value2.do_hash(&vm).expect("Hash should not fail.");
        assert_eq!(hash1, hash2);
    }
}
