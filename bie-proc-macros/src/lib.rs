use proc_macro::{self, TokenStream};
use quote::quote;

/// The smallest exponent that can appear in an f64, though not all mantissas are valid here.
const NUMBER_EXP_MIN: i32 = -324;
const NUMBER_EXP_MIN_P: i32 = 324;
const NUMBER_EXP_MIN_F: f64 = -324.0;
const EXP_MIN_VALUE: f64 = 1e-323;

/// The largest exponent that can appear in an f64, though not all mantissas are valid here.
const NUMBER_EXP_MAX: i32 = 308;

/// The length of the cache used for powers of 10.
const LENGTH: usize = (NUMBER_EXP_MAX - NUMBER_EXP_MIN + 1) as usize;

#[proc_macro]
pub fn insert_consts_and_cache_for_powers_of_ten(_: TokenStream) -> TokenStream {
    let mut values = [0.0; LENGTH];
    for (i, item) in &mut values.iter_mut().enumerate() {
        *item = 10.0_f64.powi((i as i32) + NUMBER_EXP_MIN);
    }

    let output = quote! {
        /// The smallest exponent that can appear in an f64, though not all mantissas are valid here.
        pub(crate) const NUMBER_EXP_MIN: i32 = #NUMBER_EXP_MIN;
        pub(crate) const NUMBER_EXP_MIN_P: i32 = #NUMBER_EXP_MIN_P;
        pub(crate) const NUMBER_EXP_MIN_F: f64 = #NUMBER_EXP_MIN_F;
        pub(crate) const EXP_MIN_VALUE: f64 = #EXP_MIN_VALUE;

        /// The largest exponent that can appear in an f64, though not all mantissas are valid here.
        pub(crate) const NUMBER_EXP_MAX: i32 = #NUMBER_EXP_MAX;

        // macro generated list of powers of 10
        pub static CACHED_POWERS: [f64; #LENGTH] = [#(#values),*];
    };

    output.into()
}
