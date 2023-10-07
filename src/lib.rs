use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use paste::paste;
use thunderdome::Arena;

pub trait Component: 'static + Sized {
    type Id: Copy;
}

macro_rules! component {
    ($ty:ident) => {
        $crate::paste! {
            #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
            pub struct  [<$ty Id>] (thunderdome::Index);

            impl From<thunderdome::Index> for [<$ty Id>] {
                fn from(index: thunderdome::Index) -> Self {
                    Self(index)
                }
            }
        }

        impl $crate::Component for $ty {
            type Id = $crate::paste! { [<$ty Id>] };
        }
    };
}

pub trait FromIndex: Sized {
    fn from_index(idx: thunderdome::Index) -> Self;
}

impl<T> FromIndex for T
where
    T: From<thunderdome::Index>,
{
    fn from_index(idx: thunderdome::Index) -> Self {
        Self::from(idx)
    }
}

pub struct Store {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn spawn<K, T>(&mut self, value: T) -> K
    where
        T: Component<Id = K>,
        K: FromIndex + Copy,
    {
        let type_id = TypeId::of::<T>();

        let idx = if let Some(arena) = self.data.get_mut(&type_id) {
            arena.downcast_mut::<Arena<T>>().unwrap().insert(value)
        } else {
            let mut arena = Arena::new();
            let idx = arena.insert(value);
            self.data.insert(type_id, Box::new(arena));
            idx
        };

        K::from_index(idx)
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

#[cfg(test)]
mod tests {
    use crate::Store;

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

        #[derive(Debug, PartialEq)]
        struct Thing {
            pub x: i32,
        }

        component!(Thing);

        store.spawn(Thing { x: 1 });
        store.spawn(Thing { x: 2 });

        let mut it = store.iter::<Thing>();
        assert_eq!(1, it.next().unwrap().x);
        assert_eq!(2, it.next().unwrap().x);
        assert_eq!(None, it.next());
    }
}
