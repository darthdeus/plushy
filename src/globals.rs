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
//type Store = Vec<i32>;
type Guard<T> = MappedMutexGuard<'static, Arena<T>>;

pub static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));

pub fn iter<T>() -> StoreHolder<T> {
    let type_id = TypeId::of::<T>();
    let guard = MutexGuard::map(STORE.lock(), |store| {
        store
            .data
            .get_mut(&type_id)
            .unwrap() // TODO create if it doesn't exist, entry api?
            .downcast_mut::<Arena<T>>()
            .unwrap()
    });
    StoreHolder { guard }
}

pub struct StoreHolder<T: 'static> {
    guard: Guard<T>,
}

impl<'a, T> IntoIterator for &'a StoreHolder<T> {
    type Item = (Id<T>, &'a T);
    type IntoIter = StoreIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        StoreIterator {
            holder: self.guard.iter(),
        }
    }
}

pub struct StoreIterator<'a, T: 'static> {
    holder: thunderdome::iter::Iter<'a, T>,
}

impl<'a, T> Iterator for StoreIterator<'a, T> {
    type Item = (Id<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        //self.holder.guard.get(self.current)
        self.holder.next().map(|x| (Id(x.0, PhantomData), x.1))
    }
}
