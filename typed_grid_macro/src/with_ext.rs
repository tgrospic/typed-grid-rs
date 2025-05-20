use crate::with_traits::position_struct;
use itertools::iproduct;
use proc_macro2::*;
use quote::quote;

pub fn position_trait(x: usize, y: usize) -> String {
    format!("I{}", position_struct(x, y))
}

pub fn typed_grid_ext(cols: usize, rows: usize) -> TokenStream {
    let mut structs = vec![];
    let mut enum_variants = vec![];
    let mut traits = vec![];
    let mut movements = vec![];

    for (x, y) in iproduct!(0..cols, 0..rows) {
        let name = Ident::new(&position_struct(x, y), Span::call_site());
        let iname = Ident::new(&position_trait(x, y), Span::call_site());

        structs.push(quote! {
            #[derive(std::fmt::Debug)]
            pub struct #name;
        });

        enum_variants.push(quote! {
            #name(#name),
        });

        traits.push(quote! {
            pub trait #iname<T>: IContext<T> + std::fmt::Debug {}
            impl<T: std::fmt::Debug> #iname<T> for Ctx<#name, T> {}
        });

        let current = quote! { #iname };

        // RIGHT
        if x + 1 < cols {
            movements.push(gen_move_ext("right", &current, x + 1, y));
        }

        // LEFT
        if x > 0 {
            movements.push(gen_move_ext("left", &current, x - 1, y));
        }

        // DOWN
        if y > 0 {
            movements.push(gen_move_ext("down", &current, x, y - 1));
        }

        // UP
        if y + 1 < rows {
            movements.push(gen_move_ext("up", &current, x, y + 1));
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

        // Traits with context
        #(#traits)*

        // Movement extensions with traits / context impls
        #(#movements)*

        trait Moved {
            fn moved(&mut self, p: Position);
        }
    }
}

pub fn gen_move_ext(
    method: &str,
    current_trait: &TokenStream,
    target_x: usize,
    target_y: usize,
) -> TokenStream {
    let move_method = Ident::new(method, Span::call_site());
    let target_struct = Ident::new(&position_struct(target_x, target_y), Span::call_site());
    let target_trait = Ident::new(&position_trait(target_x, target_y), Span::call_site());
    let ext_ident = Ident::new(
        &format!("Ext_{}_{}", current_trait, target_trait),
        Span::call_site(),
    );

    quote! {
        #[allow(non_camel_case_types)]
        pub trait #ext_ident<This, T>
        where
            This: #current_trait<T>,
            T: Moved + std::fmt::Debug,
        {
            #[allow(patterns_in_fns_without_body, clippy::inline_fn_without_body, unused_attributes)]
            fn #move_method(self) -> impl #target_trait<T>;
        }
        impl<This, T> #ext_ident<This, T> for This
        where
            This: #current_trait<T>,
            T: Moved + std::fmt::Debug,
        {
            fn #move_method(self) -> impl #target_trait<T> {
                let mut ctx = self.ctx();
                ctx.moved(Position::#target_struct(#target_struct));

                Ctx::new(#target_struct, ctx)
            }
        }
    }
}
