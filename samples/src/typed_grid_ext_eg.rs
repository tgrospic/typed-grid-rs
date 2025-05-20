mod derived {
    use typed_grid::*;

    typed_grid_ext!(3, 1);

    impl Moved for i32 {
        fn moved(&mut self, p: Position) {
            *self += 1;
            println!("MOVED: {p:?} {self}")
        }
    }

    #[test]
    fn test() {
        fn run<P: IPos0x0<i32>>(start: P) -> impl IPos0x0<i32> {
            start.right().right().left().left()
        }

        let pos = Ctx::new(Pos0x0, 0);
        let pos = pos.right().right().left().left();
        let pos = run(pos);

        println!("POSITION {:?}", pos);
    }
}

mod expanded {
    use extend::ext;
    use std::fmt::Debug;
    use typed_grid::*;

    // Generated with procedural macro

    #[derive(Debug)]
    pub struct Pos0x0;
    #[derive(Debug)]
    pub struct Pos0x1;
    #[derive(Debug)]
    pub struct Pos1x0;

    #[derive(Debug)]
    pub enum Position {
        Pos0x0(Pos0x0),
        Pos0x1(Pos0x1),
        Pos1x0(Pos1x0),
    }

    pub trait IPos0x0<T>: IContext<T> + Debug {}
    impl<T: Debug> IPos0x0<T> for Ctx<Pos0x0, T> {}

    pub trait IPos0x1<T>: IContext<T> + Debug {}
    impl<T: Debug> IPos0x1<T> for Ctx<Pos0x1, T> {}

    pub trait IPos1x0<T>: IContext<T> + Debug {}
    impl<T: Debug> IPos1x0<T> for Ctx<Pos1x0, T> {}

    #[ext(name = Pos00_01)]
    pub impl<This, T> This
    where
        This: IPos0x0<T>,
        T: Moved + Debug,
    {
        fn right(self) -> impl IPos0x1<T> {
            let mut ctx = self.ctx();
            ctx.moved(Position::Pos0x1(Pos0x1));

            Ctx::new(Pos0x1, ctx)
        }
    }

    #[ext(name = Pos01_00)]
    pub impl<This, T> This
    where
        This: IPos0x1<T>,
        T: Moved + Debug,
    {
        fn left(self) -> impl IPos0x0<T> {
            let mut ctx = self.ctx();
            ctx.moved(Position::Pos0x0(Pos0x0));

            Ctx::new(Pos0x0, ctx)
        }
    }

    #[ext(name = Pos01_10)]
    pub impl<This, T> This
    where
        This: IPos0x1<T>,
        T: Moved + Debug,
    {
        fn right(self) -> impl IPos1x0<T> {
            let mut ctx = self.ctx();
            ctx.moved(Position::Pos1x0(Pos1x0));

            Ctx::new(Pos1x0, ctx)
        }
    }

    #[ext(name = Pos10_01)]
    pub impl<This, T> This
    where
        This: IPos1x0<T>,
        T: Moved + Debug,
    {
        fn left(self) -> impl IPos0x1<T> {
            let mut ctx = self.ctx();
            ctx.moved(Position::Pos0x1(Pos0x1));

            Ctx::new(Pos0x1, ctx)
        }
    }

    trait Moved {
        fn moved(&mut self, p: Position);
    }

    // User code

    impl Moved for i32 {
        fn moved(&mut self, p: Position) {
            *self += 1;
            println!("MOVED: {p:?} {self}")
        }
    }

    #[test]
    fn test() {
        fn run<P: IPos0x0<i32>>(start: P) -> impl IPos0x0<i32> {
            start.right().right().left().left()
        }

        let pos = Ctx::new(Pos0x0, 0);
        let pos = pos.right().right().left().left();
        let pos = run(pos);

        println!("POSITION {:?}", pos);
    }
}
