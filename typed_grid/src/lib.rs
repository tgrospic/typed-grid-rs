//! A macro to generate grid navigation (left/right/up/down) types that allows only moves
//! inside of grid, offering compile time check of movements.
//!
//! To track movements and support custom state, generated `Moved` must be implemented for
//! the state carried throughout the movements.
//!
//! ```
//! use typed_grid::*;
//!
//! typed_grid!(2, 2);
//!
//! impl Moved for i32 {
//!     fn moved(&mut self, p: Position) {
//!         *self += 1;
//!         println!("MOVED: {p:?} {self}")
//!     }
//! }
//!
//! fn run(start: Ctx<Pos0x0, i32>) -> Ctx<Pos0x0, i32> {
//!     start.right().up().down().left()
//! }
//!
//! // State is a number for which `Moved` is implemented
//! let pos = Ctx(Pos0x0, 42);
//! let pos = pos.right().up().down().left();
//! let pos = run(pos);
//! ```

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
