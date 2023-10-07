use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

pub use paste;
use thunderdome::{Arena, Index};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id<T>(Index, std::marker::PhantomData<T>);

pub struct Store {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn spawn<T: 'static>(&mut self, value: T) -> Id<T> {
        let type_id = TypeId::of::<T>();

        let idx = if let Some(arena) = self.data.get_mut(&type_id) {
            arena.downcast_mut::<Arena<T>>().unwrap().insert(value)
        } else {
            let mut arena = Arena::new();
            let idx = arena.insert(value);
            self.data.insert(type_id, Box::new(arena));
            idx
        };

        Id(idx, PhantomData::default())
    }

    pub fn iter<T: 'static>(&self) -> Box<dyn Iterator<Item = (Id<T>, &T)> + '_> {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get(&type_id) {
            Box::new(
                arena
                    .downcast_ref::<Arena<T>>()
                    .unwrap()
                    .iter()
                    .map(|x| (Id(x.0, PhantomData::default()), x.1)),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Store;

    #[test]
    fn empty_store_is_okay() {
        let store = Store::new();

        assert_eq!(None, store.iter::<i32>().next());
        assert_eq!(None, store.iter::<f32>().next());
    }

    #[test]
    fn simple_test() {
        let mut store = Store::new();

        #[derive(Debug, PartialEq)]
        struct Thing {
            pub x: i32,
        }

        store.spawn(Thing { x: 1 });
        store.spawn(Thing { x: 2 });

        let mut it = store.iter::<Thing>();
        assert_eq!(1, it.next().unwrap().1.x);
        assert_eq!(2, it.next().unwrap().1.x);
        assert_eq!(None, it.next());
    }
}
