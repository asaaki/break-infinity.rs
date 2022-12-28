use proc_macro::{self, TokenStream};
use quote::quote;

/// The smallest exponent that can appear in an f64, though not all mantissas are valid here.
const NUMBER_EXP_MIN: i32 = -324;

/// The largest exponent that can appear in an f64, though not all mantissas are valid here.
const NUMBER_EXP_MAX: i32 = 308;

/// The length of the cache used for powers of 10.
const LENGTH: usize = (NUMBER_EXP_MAX - NUMBER_EXP_MIN + 1) as usize;

#[proc_macro]
pub fn insert_cache_for_powers_of_ten(_: TokenStream) -> TokenStream {
    let mut values = [0.0; LENGTH];
    for (i, item) in &mut values.iter_mut().enumerate() {
        *item = 10.0_f64.powi((i as i32) + NUMBER_EXP_MIN);
    }

    let output = quote! {
        pub static CACHED_POWERS: [f64; #LENGTH] = [#(#values),*];
    };

    output.into()
}
