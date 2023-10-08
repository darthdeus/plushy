use crate::*;

use once_cell::sync::Lazy;
use parking_lot::*;

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

impl<'a, T> IntoIterator for &'a mut StoreHolder<T> {
    type Item = (Id<T>, &'a mut T);
    type IntoIter = StoreIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        StoreIterator {
            holder: self.guard.iter_mut(),
        }
    }
}

pub struct StoreIterator<'a, T: 'static> {
    holder: thunderdome::iter::IterMut<'a, T>,
}

impl<'a, T> Iterator for StoreIterator<'a, T> {
    type Item = (Id<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        //self.holder.guard.get(self.current)
        self.holder.next().map(|x| (Id(x.0, PhantomData), x.1))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_iter() {
        let val: i32 = 1;
        STORE.lock().spawn(val);
        for val in iter::<i32>().into_iter() {
            assert_eq!(1, *val.1);
        }
    }
}
