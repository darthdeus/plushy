use crate::*;

use once_cell::sync::Lazy;
use parking_lot::*;

type Guard<T> = MappedMutexGuard<'static, Arena<T>>;

pub static STORE: Lazy<Mutex<Store>> = Lazy::new(|| Mutex::new(Store::new()));

pub fn iter<T: 'static + Sync + Send>() -> StoreHolder<T> {
    let type_id = TypeId::of::<T>();
    let guard = MutexGuard::map(STORE.lock(), |store| {
        store
            .data
            .entry(type_id)
            .or_insert_with(|| {
                let arena: Arena<T> = Arena::new();
                Box::new(arena)
            })
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
    type IntoIter = StoreIteratorMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        StoreIteratorMut {
            inner: self.guard.iter_mut(),
        }
    }
}

pub struct StoreIteratorMut<'a, T: 'static> {
    inner: thunderdome::iter::IterMut<'a, T>,
}

impl<'a, T> Iterator for StoreIteratorMut<'a, T> {
    type Item = (Id<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        //self.holder.guard.get(self.current)
        self.inner.next().map(|x| (Id(x.0, PhantomData), x.1))
    }
}

impl<'a, T> IntoIterator for &'a StoreHolder<T> {
    type Item = (Id<T>, &'a T);
    type IntoIter = StoreIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        StoreIterator {
            inner: self.guard.iter(),
        }
    }
}

pub struct StoreIterator<'a, T: 'static> {
    inner: thunderdome::iter::Iter<'a, T>,
}

impl<'a, T> Iterator for StoreIterator<'a, T> {
    type Item = (Id<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| (Id(x.0, PhantomData), x.1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_iter() {
        let val: i32 = 1;
        STORE.lock().spawn(val);
        for val in &mut iter::<i32>() {
            *val.1 *= 2;
        }
        let values = (&iter::<i32>())
            .into_iter()
            .map(|it| *it.1)
            .collect::<Vec<i32>>();
        assert_eq!(&[2], values.as_slice());
    }
}
