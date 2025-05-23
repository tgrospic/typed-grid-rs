# Typed grid

**Strongly typed grid navigation in Rust with compile-time movement constraints.**

typed_grid is a Rust procedural macro crate that generates a grid of position types (`PosXxY`) and enforces legal movements (`left`, `right`, `up`, `down`) at compile time using Rust’s type system. Inspired by dependently typed programming, it encodes position and movement rules in the type system itself—ensuring that only valid transitions are representable and catchable by the compiler.

This is especially useful in domains such as:

* Grid-based games
* UI navigation
* Embedded systems
* Robotics path planning
* Finite state machines over 2D layouts

## ✨ Features

* ✅ **Compile-time checked movement logic**
* ✅ **Zero runtime cost for boundary enforcement**
* ✅ **Custom per-position logic via trait implementations**
* ✅ **Composable movement sequences**
* ✅ **Custom state tracking across movements**

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typed_grid = "*" # or replace with latest version
```

Add the macro import to your crate root:

```rust
use typed_grid::*;
```

## ⚙️ Usage

```rust
use typed_grid::*;

typed_grid!(2, 2); // Generates a 2x2 grid

impl Moved for i32 {
    fn moved(&mut self, p: Position) {
        *self += 1;
        println!("MOVED: {p:?} {self}")
    }
}

fn run(start: Ctx<Pos0x0, i32>) -> Ctx<Pos0x0, i32> {
    start.right().up().down().left()
}

let pos = Ctx(Pos0x0, 42);
let pos = pos.right().up().down().left();
let pos = run(pos);
```

## 🧠 Concepts

### `typed_grid!(W, H)`

This macro generates:

* Position types like `Pos0x0`, `Pos1x2`, etc.
* A `Ctx<P, T>` wrapper for combining a position and your custom state
* Movement trait implementations (`MoveRight`, `MoveLeft`, `MoveUp`, `MoveDown`) encoded at the type level

### `typed_grid_ext!(W, H)`

The `typed_grid_ext!` macro generates movements with traits (`IPosXxY<T>`) allowing for more composable and generic movement logic. 

```rust
# use std::fmt::Debug;
# use typed_grid::*;
#
typed_grid_ext!(2, 5);

fn moves<P: IPos0x0<T>, T: Moved + Debug>(start: P) -> impl IPos0x1<T> {
    start.right().up().up().up().up().down().down().left().down()
}
```

### `Ctx<P, T>`

`Ctx` is a context object wrapping a position `P` and user-defined state `T`.

You move across the grid using:

```rust
# use typed_grid::*;
#
# typed_grid!(2, 2);
#
# let ctx = Ctx(Pos0x0, ());
#
# impl Moved for () {
#     fn moved(&mut self, p: Position) {}
# }
#
ctx.right().up().down().left();
```

### `Moved` Trait

You MUST implement the `Moved` trait for your custom state type (`T`) to receive notifications whenever a movement occurs.

```rs
trait Moved {
    fn moved(&mut self, to: Position);
}
```

This allows mutation or side-effects during position transitions.

## 📐 Compile-time Safety

Invalid moves are caught at **compile time**:

```rust,compile_fail
# use typed_grid::*;
typed_grid!(2, 2);

let pos = Ctx(Pos1x1, ());
let new_pos = pos.right(); // ❌ Compile-time error: Pos2x1 doesn't exist
```

## 📌 Why?

Rather than relying on runtime conditionals or grid bounds checks, `typed_grid` ensures safety using **Rust’s type system**. You cannot move to an invalid position, and legal transitions are encoded via trait bounds.

## 🧪 Testing

To run tests:

```bash
cargo test --workspace
```

## 🔍 Explore the samples

Examples of macro expansion and usage are available under [`samples/src`](https://github.com/tgrospic/typed-grid-rs/blob/master/samples/src).


To run examples:

```bash
cargo run -p samples
```

## 🔮 Wishlist

* [ ] Optional position metadata
* [ ] Visual debugger or macro-generated grid visualization
* [ ] Support for 3D or N-dimensional grids

## 💬 Contributing

Contributions are welcome! Please open issues for bugs, ideas, or improvements.

## 📄 License

* [MIT license](https://github.com/tgrospic/typed-grid-rs/blob/master/LICENSE)

## 👋 Acknowledgements

Inspired by dependently typed languages, denotational design, and a love for statically enforced correctness.
