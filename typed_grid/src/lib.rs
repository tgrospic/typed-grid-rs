#![doc = include_str!("../README.md")]

pub use typed_grid_macro::{typed_grid, typed_grid_ext};

use derive_new::new as New;
use std::fmt::Debug;

// Traits (move) based typed grid

pub trait MoveRight {
    type Then;

    fn right(self) -> Self::Then;
}

pub trait MoveLeft {
    type Then;

    fn left(self) -> Self::Then;
}

pub trait MoveDown {
    type Then;

    fn down(self) -> Self::Then;
}

pub trait MoveUp {
    type Then;

    fn up(self) -> Self::Then;
}

// Extension based typed grid

pub trait IContext<T> {
    fn ctx(self) -> T;
}

#[derive(Debug, New)]
pub struct Ctx<P, T>(P, T);

impl<P, T: Debug> IContext<T> for Ctx<P, T> {
    fn ctx(self) -> T {
        self.1
    }
}
