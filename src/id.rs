use rand::{thread_rng, Rng};
use std::{
    collections::HashMap,
    marker::PhantomData,
    cell::*,
    rc::*,
    ops::{Deref, DerefMut, BitOr}, borrow::{Borrow, BorrowMut}, hash::Hash
};

pub struct Id<T> {
    pub id: usize,
    map: Rc<RefCell<HashMap<usize, T>>>,
    _marker: PhantomData<T>
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id {
            id: self.id,
            map: Rc::clone(&self.map),
            _marker: PhantomData
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.id = source.id;
        self.map = Rc::clone(&source.map);
    }
}

impl<T> Id<T> {
    pub fn new(map: &Rc<RefCell<HashMap<usize, T>>>) -> Self {
        Id {
            id: thread_rng().gen_range(0..usize::MAX),
            map: Rc::clone(map),
            _marker: PhantomData
        }
    }

    pub fn create(map: &Rc<RefCell<HashMap<usize, T>>>, value: T) -> Self {
        let mut this = Self::new(map);

        this.map.get_mut().insert(this.id, value);

        this
    }

    pub fn get<'a>(&'a mut self) -> Option<&'a T> {
        // let value: Option<&T> = self.map.borrow_mut().get(&self.id);
        let map = self.map.get_mut();

        map.get(&self.id)
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        self.map.get_mut().get_mut(&self.id)
    }

    pub fn set(&self, value: T) {
        self.map.get_mut().entry(self.id).and_modify(|x|{ *x = value });
    }
}

