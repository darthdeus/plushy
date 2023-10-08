use crate::*;

use once_cell::sync::Lazy;

#[allow(unused_imports)]
use atomic_refcell::{AtomicRef, AtomicRefCell};

//pub static STORE: Lazy<AtomicRefCell<Store>> = Lazy::new(|| AtomicRefCell::new(Store::new()));

// Here lies sadness
// pub fn iter<T>() -> AtomicRef<'static, &'static Box<dyn Iterator<Item = (Id<T>, &'static T)>>> {
//     AtomicRef::map(STORE.borrow(), |store| &store.iter::<T>())
// }

// ****************************************

// Alternative sadness, no less sad

use parking_lot::*;
//
//
type Store = Vec<i32>;
type Guard = MutexGuard<'static, Store>;

pub static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));

pub fn iter<T>() -> StoreHolder<T> {
    //MutexGuard::map(STORE.lock(), |store| &mut store[0])
    StoreHolder { guard: STORE.lock(), phantom_data: PhantomData }
}

pub struct StoreHolder<T: 'static> {
    guard: Guard,
    phantom_data: PhantomData<T>,
}

impl<'a, T> IntoIterator for &'a StoreHolder<T> {
    type Item = &'a i32;
    type IntoIter = StoreIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        StoreIterator {
            holder: self,
            current: 0,
            phantom_data: PhantomData,
        }
    }
}

pub struct StoreIterator<'a, T: 'static> {
    holder: &'a StoreHolder<T>,
    current: usize,
    phantom_data: PhantomData<&'a T>,
}

impl<'a, T> Iterator for StoreIterator<'a, T> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.holder.guard.get(self.current)
    }
}
