use proc_macro2::*;
use quote::quote;

pub fn position_struct(x: usize, y: usize) -> String {
    format!("Pos{}x{}", x, y)
}

pub fn typed_grid(cols: usize, rows: usize) -> TokenStream {
    let mut structs = vec![];
    let mut enum_variants = vec![];
    let mut movements = vec![];

    for x in 0..cols {
        for y in 0..rows {
            let name = Ident::new(&position_struct(x, y), Span::call_site());
            structs.push(quote! {
                #[derive(Debug)]
                pub struct #name;
            });
            enum_variants.push(quote! {
                #name(#name),
            });

            let current = quote! { #name };

            // RIGHT
            if x + 1 < cols {
                movements.push(gen_move_impl("MoveRight", "right", &current, x + 1, y));
            }

            // LEFT
            if x > 0 {
                movements.push(gen_move_impl("MoveLeft", "left", &current, x - 1, y));
            }

            // DOWN
            if y > 0 {
                movements.push(gen_move_impl("MoveDown", "down", &current, x, y - 1));
            }

            // UP
            if y + 1 < rows {
                movements.push(gen_move_impl("MoveUp", "up", &current, x, y + 1));
            }
        }
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
    direction_trait: &str,
    direction_method: &str,
    current_position: &TokenStream,
    target_x: usize,
    target_y: usize,
) -> TokenStream {
    let direction = Ident::new(direction_trait, Span::call_site());
    let move_method = Ident::new(direction_method, Span::call_site());
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
