use crate::Decimal;
use std::cmp::Ordering::{self, *};

impl PartialOrd for Decimal {
    fn partial_cmp(&self, decimal: &Self) -> Option<Ordering> {
        /*
        From smallest to largest:
        -Infinity
        -3e100
        -1e100
        -3e99
        -1e99
        -3e0
        -1e0
        -3e-99
        -1e-99
        -3e-100
        -1e-100
        0
        1e-100
        3e-100
        1e-99
        3e-99
        1e0
        3e0
        1e99
        3e99
        1e100
        3e100
        Infinity
        */

        if f64::is_nan(self.mantissa)
            || f64::is_nan(self.exponent)
            || f64::is_nan(decimal.mantissa)
            || f64::is_nan(decimal.exponent)
        {
            None
        } else if (f64::is_infinite(self.mantissa) && self.mantissa.is_sign_negative())
            || (f64::is_infinite(decimal.mantissa) && decimal.mantissa.is_sign_positive())
        {
            Some(Less)
        } else if (f64::is_infinite(self.mantissa) && self.mantissa.is_sign_negative())
            || (f64::is_infinite(decimal.mantissa) && decimal.mantissa.is_sign_positive())
        {
            Some(Greater)
        } else if self.mantissa == 0.0 {
            if decimal.mantissa == 0.0 {
                Some(Equal)
            } else if decimal.mantissa < 0.0 {
                Some(Greater)
            } else {
                Some(Less)
            }
        } else if decimal.mantissa == 0.0 {
            if self.mantissa < 0.0 {
                Some(Less)
            } else {
                Some(Greater)
            }
        } else if self.mantissa > 0.0 {
            if self.exponent > decimal.exponent || decimal.mantissa < 0.0 {
                Some(Greater)
            } else if self.exponent < decimal.exponent {
                Some(Less)
            } else if self.mantissa > decimal.mantissa {
                Some(Greater)
            } else if self.mantissa < decimal.mantissa {
                Some(Less)
            } else {
                Some(Equal)
            }
        } else if self.exponent > decimal.exponent || decimal.mantissa > 0.0 {
            Some(Less)
        } else if self.mantissa > decimal.mantissa || self.exponent < decimal.exponent {
            Some(Greater)
        } else if self.mantissa < decimal.mantissa {
            Some(Less)
        } else {
            Some(Equal)
        }
    }

    fn lt(&self, other: &Decimal) -> bool {
        self.partial_cmp(other)
            .map(Ordering::is_lt)
            .unwrap_or(false)
    }
    fn le(&self, other: &Decimal) -> bool {
        self.partial_cmp(other)
            .map(Ordering::is_le)
            .unwrap_or(false)
    }

    fn gt(&self, other: &Decimal) -> bool {
        self.partial_cmp(other)
            .map(Ordering::is_gt)
            .unwrap_or(false)
    }
    fn ge(&self, other: &Decimal) -> bool {
        self.partial_cmp(other)
            .map(Ordering::is_ge)
            .unwrap_or(false)
    }
}

impl PartialEq<Decimal> for Decimal {
    fn eq(&self, decimal: &Decimal) -> bool {
        self.mantissa == decimal.mantissa && self.exponent == decimal.exponent
    }
}

impl Eq for Decimal {}
