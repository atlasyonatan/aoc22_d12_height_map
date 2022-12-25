use std::ops::{Index, IndexMut};

pub type Direction = bool;
pub const FORWARD: Direction = true;
pub const BACKWARD: Direction = false;

pub struct BiDirection<T> {
    pub forward: Option<T>,
    pub backward: Option<T>,
}

impl<T> BiDirection<T> {
    pub fn new() -> Self {
        Self {
            forward: None,
            backward: None,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Direction, &T)> {
        [FORWARD, BACKWARD]
            .into_iter()
            .filter_map(|direction| Some((direction, self[direction].as_ref()?)))
    }

    pub fn get(&self, direction: Direction) -> Option<&T> {
        self[direction].as_ref()
    }

    pub fn get_mut(&mut self, direction: Direction) -> Option<&mut T> {
        self[direction].as_mut()
    }
}

impl<T> Index<Direction> for BiDirection<T> {
    type Output = Option<T>;

    fn index(&self, index: Direction) -> &Self::Output {
        match index {
            FORWARD => &self.forward,
            BACKWARD => &self.backward,
        }
    }
}

impl<T> IndexMut<Direction> for BiDirection<T> {
    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        match index {
            FORWARD => &mut self.forward,
            BACKWARD => &mut self.backward,
        }
    }
}
