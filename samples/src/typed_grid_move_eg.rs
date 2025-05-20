mod derived {
    use typed_grid::*;

    typed_grid!(3, 1);

    impl Moved for i32 {
        fn moved(&mut self, p: Position) {
            *self += 1;
            println!("MOVED: {p:?} {self}")
        }
    }

    #[test]
    fn test() {
        fn run(start: Ctx<Pos0x0, i32>) -> Ctx<Pos0x0, i32> {
            start.right().right().left().left()
        }

        let pos = Ctx(Pos0x0, 0);
        let pos = pos.right().right().left().left();
        let pos = run(pos);

        println!("POSITION {:?}", pos);
    }
}

mod expanded {
    use typed_grid::*;

    // Generated with procedural macro

    #[derive(Debug)]
    pub struct Pos0x0;
    #[derive(Debug)]
    pub struct Pos1x0;
    #[derive(Debug)]
    pub struct Pos2x0;

    #[derive(Debug)]
    pub enum Position {
        Pos0x0(Pos0x0),
        Pos1x0(Pos1x0),
        Pos2x0(Pos2x0),
    }

    #[derive(Debug)]
    pub struct Ctx<P, T>(P, T);

    impl<T: Moved> MoveRight for Ctx<Pos0x0, T> {
        type Then = Ctx<Pos1x0, T>;

        fn right(self) -> Self::Then {
            let mut s = self.1;
            s.moved(Position::Pos1x0(Pos1x0));
            Ctx(Pos1x0, s)
        }
    }

    impl<T: Moved> MoveRight for Ctx<Pos1x0, T> {
        type Then = Ctx<Pos2x0, T>;

        fn right(self) -> Self::Then {
            let mut s = self.1;
            s.moved(Position::Pos2x0(Pos2x0));
            Ctx(Pos2x0, s)
        }
    }

    impl<T: Moved> MoveLeft for Ctx<Pos1x0, T> {
        type Then = Ctx<Pos0x0, T>;

        fn left(self) -> Self::Then {
            let mut s = self.1;
            s.moved(Position::Pos0x0(Pos0x0));
            Ctx(Pos0x0, s)
        }
    }

    impl<T: Moved> MoveLeft for Ctx<Pos2x0, T> {
        type Then = Ctx<Pos1x0, T>;

        fn left(self) -> Self::Then {
            let mut s = self.1;
            s.moved(Position::Pos1x0(Pos1x0));
            Ctx(Pos1x0, s)
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
        fn run(start: Ctx<Pos0x0, i32>) -> Ctx<Pos1x0, i32> {
            start.right().right().left()
        }

        let pos = Ctx(Pos0x0, 0);
        let pos = pos.right().right().left().left();
        let pos = run(pos);

        println!("POSITION {:?}", pos);
    }
}
