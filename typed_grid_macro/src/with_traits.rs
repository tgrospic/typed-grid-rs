use itertools::iproduct;
use proc_macro2::*;
use quote::quote;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn trait_name(&self) -> String {
        format!("Move{:?}", self)
    }

    pub fn method_name(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

pub fn position_struct(x: usize, y: usize) -> String {
    format!("Pos{}x{}", x, y)
}

pub fn typed_grid(cols: usize, rows: usize) -> TokenStream {
    let mut structs = vec![];
    let mut enum_variants = vec![];
    let mut movements = vec![];

    for (x, y) in iproduct!(0..cols, 0..rows) {
        let name = Ident::new(&position_struct(x, y), Span::call_site());
        structs.push(quote! {
            #[derive(Debug)]
            pub struct #name;
        });
        enum_variants.push(quote! {
            #name(#name),
        });

        let current = quote! { #name };

        movements.extend(
            [
                (x + 1 < cols).then(|| (Direction::Right, x + 1, y)),
                (x > 0).then(|| (Direction::Left, x - 1, y)),
                (y > 0).then(|| (Direction::Down, x, y - 1)),
                (y + 1 < rows).then(|| (Direction::Up, x, y + 1)),
            ]
            .into_iter()
            .flatten()
            .map(|(direction, x, y)| gen_move_impl(direction, &current, x, y)),
        );
    }

    quote! {
        // Structs
        #(#structs)*

        // Runtime position
        #[derive(std::fmt::Debug)]
        pub enum Position {
            #(#enum_variants)*
        }

        // Context
        #[derive(std::fmt::Debug)]
        pub struct Ctx<P, T>(P, T);

        // Movement impls
        #(#movements)*

        trait Moved {
            fn moved(&mut self, p: Position);
        }
    }
}

pub fn gen_move_impl(
    dir: Direction,
    current_position: &TokenStream,
    target_x: usize,
    target_y: usize,
) -> TokenStream {
    let direction = Ident::new(&dir.trait_name(), Span::call_site());
    let move_method = Ident::new(&dir.method_name(), Span::call_site());
    let target_struct = Ident::new(&position_struct(target_x, target_y), Span::call_site());

    quote! {
        impl<T: Moved> #direction for Ctx<#current_position, T> {
            type Then = Ctx<#target_struct, T>;

            fn #move_method(self) -> Self::Then {
                let mut s = self.1;
                s.moved(Position::#target_struct(#target_struct));

                Ctx(#target_struct, s)
            }
        }
    }
}
