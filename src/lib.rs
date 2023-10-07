use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use thunderdome::Arena;
use paste::paste;

pub trait Component: 'static + Sized {
    type Id: Copy;
}

// macro_rules! component {
//     ($ty:ident) => {
//         #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
//         paste! {
//             pub struct [< $ty Id >](thunderdome::Index);
//         }
//
//
//         impl Component for $ty {
//             type Id = paste! [< $ty Id >];
//         }
//     };
// }

macro_rules! component {
    ($ty:ident) => {
        paste! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            pub struct  [<$ty Id>] (thunderdome::Index);
        }

        impl Component for $ty {
            type Id = paste! { [<$ty Id>] };
        }
    };
}

// macro_rules! component {
//     ($ty:ident, $id:ident) => {
//         #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
//         pub struct $id(thunderdome::Index);
//
//
//         impl Component for $ty {
//             type Id = $id;
//         }
//     };
// }

pub struct Store {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn spawn<T: Component>(&mut self, value: T) {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get_mut(&type_id) {
            arena.downcast_mut::<Arena<T>>().unwrap().insert(value);
        } else {
            let mut arena = Arena::new();
            arena.insert(value);
            self.data.insert(type_id, Box::new(arena));
        }
    }

    pub fn iter<T: Component>(&self) -> Box<dyn Iterator<Item = &T> + '_> {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get(&type_id) {
            Box::new(
                arena
                    .downcast_ref::<Arena<T>>()
                    .unwrap()
                    .iter()
                    .map(|x| x.1),
            )
        } else {
            Box::new(std::iter::empty())
        }
    }
}

#[derive(Debug, PartialEq)]
struct Thing {
    pub x: i32,
}

component!(Thing);
// impl Component for Thing {
//     type Id = ThingId;
// }
//
// #[derive(Copy, Clone)]
// struct ThingId(thunderdome::Index);

#[cfg(test)]
mod tests {
    use crate::{Component, Store, Thing};

    // #[test]
    // fn empty_store_is_okay() {
    //     let store = Store::new();
    //
    //     assert_eq!(None, store.iter::<i32>().next());
    //     assert_eq!(None, store.iter::<f32>().next());
    // }

    #[test]
    fn simple_test() {
        let mut store = Store::new();

        store.spawn(Thing { x: 1 });
        store.spawn(Thing { x: 2 });

        let mut it = store.iter::<Thing>();
        assert_eq!(1, it.next().unwrap().x);
        assert_eq!(2, it.next().unwrap().x);
        assert_eq!(None, it.next());
    }
}
