use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use thunderdome::{Arena, Index};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Id<T>(Index, std::marker::PhantomData<T>);

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct Store {
    pub data: HashMap<TypeId, Box<dyn Any>>,
}

impl Store {
    /// Creates a new plushy store.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Spawns a new value in the store and returns an id to it.
    ///
    /// Types don't need to be registered beforehand or implement any traits.
    /// The ids are strongly typed, for example id for `i32` is `Id<i32>`.
    ///
    /// ```
    /// use plushy::*;
    /// let mut store = Store::new();
    ///
    /// let id = store.spawn(3);
    ///
    /// assert_eq!(Some(&3), store.get(id));
    /// ```
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

    /// Returns a reference to the value corresponding to the given id.
    /// ```
    /// use plushy::*;
    ///
    /// let mut store = Store::new();
    ///
    /// let id1 = store.spawn(3);
    /// let id2 = store.spawn(2);
    ///
    /// assert_eq!(Some(&3), store.get(id1));
    /// assert_eq!(Some(&2), store.get(id2));
    /// ```
    pub fn get<T: 'static>(&self, id: Id<T>) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get(&type_id) {
            arena.downcast_ref::<Arena<T>>().unwrap().get(id.0)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the value corresponding to the given id.
    ///
    /// ```
    /// use plushy::*;
    ///
    /// let mut store = Store::new();
    ///
    /// let id1 = store.spawn(3);
    /// let id2 = store.spawn(2);
    ///
    /// assert_eq!(Some(&mut 3), store.get_mut(id1));
    /// assert_eq!(Some(&mut 2), store.get_mut(id2));
    ///
    /// *store.get_mut(id1).unwrap() = 4;
    /// *store.get_mut(id2).unwrap() = 5;
    ///
    /// assert_eq!(Some(&4), store.get(id1));
    /// assert_eq!(Some(&5), store.get(id2));
    /// ```
    pub fn get_mut<T: 'static>(&mut self, id: Id<T>) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get_mut(&type_id) {
            arena.downcast_mut::<Arena<T>>().unwrap().get_mut(id.0)
        } else {
            None
        }
    }

    /// Returns an iterator over all values of the given type.
    /// ```
    /// use plushy::*;
    /// let mut store = Store::new();
    /// store.spawn(3);
    /// store.spawn(2);
    ///
    /// let mut it = store.iter::<i32>();
    ///
    /// assert_eq!(&3, it.next().unwrap().1);
    /// assert_eq!(&2, it.next().unwrap().1);
    /// ```
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

    /// Returns a mutable iterator over all values of the given type.
    /// ```
    /// use plushy::*;
    /// let mut store = Store::new();   
    /// store.spawn(3);
    /// store.spawn(2);
    ///
    /// let mut it = store.iter::<i32>();
    /// assert_eq!(&mut 3, it.next().unwrap().1);
    /// assert_eq!(&mut 2, it.next().unwrap().1);
    ///
    /// drop(it);
    ///
    /// let mut it = store.iter_mut::<i32>();
    /// *it.next().unwrap().1 = 4;
    /// *it.next().unwrap().1 = 5;
    ///
    /// drop(it);
    ///
    /// let mut it = store.iter::<i32>();
    /// assert_eq!(&mut 4, it.next().unwrap().1);
    /// assert_eq!(&mut 5, it.next().unwrap().1);
    /// ```
    pub fn iter_mut<T: 'static>(&mut self) -> Box<dyn Iterator<Item = (Id<T>, &mut T)> + '_> {
        let type_id = TypeId::of::<T>();

        if let Some(arena) = self.data.get_mut(&type_id) {
            Box::new(
                arena
                    .downcast_mut::<Arena<T>>()
                    .unwrap()
                    .iter_mut()
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

        struct Enemy {
            pub x: i32,
        }

        struct Player {
            pub health: f32,
        }

        store.spawn(Enemy { x: 1 });
        store.spawn(Enemy { x: 2 });

        // Store the player's ID for later
        let player = store.spawn(Player { health: 100.0 });

        assert_eq!(
            &[1, 2],
            store
                .iter::<Enemy>()
                .map(|t| t.1.x)
                .collect::<Vec<_>>()
                .as_slice()
        );

        // Fetch the player based on the ID. Note we don't need to write
        // `store.get::<Player>(player)`, the type is inferred from the
        // strongly typed ID.
        assert_eq!(100.0, store.get(player.clone()).unwrap().health);

        // Change player health
        store.get_mut(player).unwrap().health = 200.0;

        // Fetch it again and verify the change.
        assert_eq!(200.0, store.get(player).unwrap().health);
    }
}
