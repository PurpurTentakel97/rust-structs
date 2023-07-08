#![allow(unused)]
use std::{fmt::Debug, ops::Add};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub(crate) fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub(crate) fn x(&self) -> &T {
        &self.x
    }
    pub(crate) fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }
    pub(crate) fn y(&self) -> &T {
        &self.y
    }
    pub(crate) fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
