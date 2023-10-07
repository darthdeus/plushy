use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use thunderdome::Arena;

pub struct Store {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn spawn<T: 'static>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get_mut(&type_id) {
            arena.downcast_mut::<Arena<T>>().unwrap().insert(value);
        } else {
            let mut arena = Arena::new();
            arena.insert(value);
            self.data.insert(type_id, Box::new(arena));
        }
    }

    pub fn iter<T: 'static>(&self) -> impl Iterator<Item = &T> {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get(&type_id) {
            arena
                .downcast_ref::<Arena<T>>()
                .unwrap()
                .iter()
                .map(|x| x.1)
        } else {
            // std::iter::empty()
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Store;

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
        assert_eq!(1, it.next().unwrap().x);
        assert_eq!(2, it.next().unwrap().x);
        assert_eq!(None, it.next());
    }
}
