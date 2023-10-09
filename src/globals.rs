use crate::*;

use once_cell::sync::Lazy;

#[allow(unused_imports)]
use atomic_refcell::{AtomicRef, AtomicRefCell};

pub static STORE: Lazy<AtomicRefCell<Store>> = Lazy::new(|| AtomicRefCell::new(Store::new()));

// Here lies sadness
// pub fn iter<T>() -> AtomicRef<'static, &'static Box<dyn Iterator<Item = (Id<T>, &'static T)>>> {
//     AtomicRef::map(STORE.borrow(), |store| &store.iter::<T>())
// }

// ****************************************

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

mod wip_delete_this {
    use once_cell::sync::Lazy;
    use std::{
        ops::{Deref, DerefMut},
        sync::atomic::AtomicBool,
    };

    struct Storage {
        data: Vec<i32>,
        borrow: AtomicBool,
    }

    static STORAGE: Lazy<Storage> = Lazy::new(|| Storage {
        data: vec![],
        borrow: AtomicBool::new(false),
    });

    pub struct StorageMutGuard<'a> {
        storage: (*mut i32, usize),
        borrow: &'a AtomicBool,
    }

    impl Deref for StorageMutGuard<'_> {
        type Target = [i32];

        fn deref(&self) -> &Self::Target {
            // SAFETY: The existence of the borrow guard ensures that the
            // storage is not accessed concurrently. The pointer and len are
            // correct because they are set when the guard is created.
            unsafe { std::slice::from_raw_parts(self.storage.0, self.storage.1) }
        }
    }

    impl DerefMut for StorageMutGuard<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            // SAFETY: The existence of the borrow guard ensures that the
            // storage is not accessed concurrently. The pointer and len are
            // correct because they are set when the guard is created.
            unsafe { std::slice::from_raw_parts_mut(self.storage.0, self.storage.1) }
        }
    }

    fn get_storage_mut<'a>() -> StorageMutGuard<'a> {
        if STORAGE
            .borrow
            .compare_and_swap(false, true, std::sync::atomic::Ordering::SeqCst)
        {
            panic!("Storage already borrowed");
        }
        StorageMutGuard {
            storage: (STORAGE.data.as_ptr() as *mut _, STORAGE.data.len()),
            borrow: &STORAGE.borrow,
        }
    }

    impl Drop for StorageMutGuard<'_> {
        fn drop(&mut self) {
            self.borrow
                .store(false, std::sync::atomic::Ordering::SeqCst);
        }
    }

    #[test]
    fn test_storage_mut() {
        // Iterating works like this
        for i in &mut *get_storage_mut() {
            *i += 1;
        }

        // Or like this
        get_storage_mut().iter().for_each(|i| println!("{}", i));

        // A double borrow panics
        let guard = get_storage_mut();
        assert!(std::panic::catch_unwind(|| {
            let _guard2 = get_storage_mut();
        })
        .is_err());
    }
}
