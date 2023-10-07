use crate::*;

use once_cell::sync::Lazy;

#[allow(unused_imports)]
use atomic_refcell::{AtomicRef, AtomicRefCell};

pub static STORE: Lazy<AtomicRefCell<Store>> = Lazy::new(|| AtomicRefCell::new(Store::new()));

// Here lies sadness
// pub fn iter<T>() -> AtomicRef<'static, &'static Box<dyn Iterator<Item = (Id<T>, &'static T)>>> {
//     AtomicRef::map(STORE.borrow(), |store| &store.iter::<T>())
// }









// Alternative sadness, no less sad

// use parking_lot::Mutex;
//
// type Store = Vec<i32>;
//
// pub static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));
//
// pub fn iter<T>() -> MutexGuard<'static, Box<dyn Iterator<Item = &T>>> {
//     MutexGuard::map(STORE.lock(), |store| &store.iter::<T>())
// }
//
// use parking_lot::*;
//
// type Store = Vec<i32>;
//
// pub static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));
//
// pub fn iter<'a>() -> MappedMutexGuard<'static, impl Iterator<Item = &'a i32>> {
//     MutexGuard::map(STORE.lock(), |store| &mut store.iter())
// }
