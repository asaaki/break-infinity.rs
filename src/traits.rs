use crate::{consts::*, Decimal};
use std::fmt::{Display, Formatter, Result};

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if f64::is_nan(self.mantissa) || f64::is_nan(self.exponent) {
            return write!(f, "NaN");
        } else if self.exponent >= EXP_LIMIT {
            return if self.mantissa > 0.0 {
                write!(f, "Infinity")
            } else {
                write!(f, "-Infinity")
            };
        } else if self.exponent <= -EXP_LIMIT || self.mantissa == 0.0 {
            return write!(f, "0");
        } else if self.exponent < 21.0 && self.exponent > -7.0 {
            return if let Some(places) = f.precision() {
                write!(f, "{:.*}", places, self.to_number())
            } else {
                write!(f, "{}", self.to_number())
            };
        }

        let form = if let Some(places) = f.precision() {
            self.to_exponential(places as u32)
        } else {
            self.to_exponential(16)
        };

        write!(f, "{}", form)
    }
}
