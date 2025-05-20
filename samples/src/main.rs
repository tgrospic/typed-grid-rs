mod typed_grid_ext_eg;
mod typed_grid_move_eg;

use std::fmt::Debug;
use typed_grid::*;

typed_grid_ext!(2, 5);

fn main() {
    run();
}

impl Moved for i32 {
    fn moved(&mut self, p: Position) {
        *self += 1;
        println!("MOVED: {p:?} {self}")
    }
}

fn moves<P: IPos0x0<T>, T: Moved + Debug>(start: P) -> impl IPos0x1<T> {
    start
        .right()
        .up()
        .up()
        .up()
        .up()
        .down()
        .down()
        .left()
        .down()
}

fn run() {
    let pos = Ctx::new(Pos0x0, 0);

    let end = moves(pos);

    println!("POSITION {:?}", end);
}
