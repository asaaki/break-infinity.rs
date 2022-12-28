use crate::{impl_from, Decimal, NAN};

impl From<&str> for Decimal {
    /// Creates a new instance of Decimal from the given string.
    #[allow(dead_code)]
    fn from(string: &str) -> Decimal {
        return if string.find('e').is_some() {
            let parts: Vec<&str> = string.split('e').collect();
            let decimal = Decimal {
                mantissa: String::from(parts[0]).parse().unwrap(),
                exponent: String::from(parts[1]).parse().unwrap(),
            };

            decimal.normalize()
        } else if string == "NaN" {
            NAN
        } else {
            Decimal::new(String::from(string).parse().unwrap())
        };
    }
}

// This allows converting virtually any number to a Decimal.
impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(i128);
impl_from!(isize);
impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);
impl_from!(usize);
impl_from!(f32);
impl_from!(f64);
