use crate::{
    from_mantissa_exponent, from_mantissa_exponent_no_normalize, power_of_10, Decimal,
    MAX_SIGNIFICANT_DIGITS,
};

pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Add<Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, decimal: Decimal) -> Decimal {
        // Figure out which is bigger, shrink the mantissa of the smaller
        // by the difference in exponents, add mantissas, normalize and return
        // TODO: Optimizations and simplification may be possible, see https://github.com/Patashu/break_infinity.js/issues/8
        if self.mantissa == 0.0 {
            return decimal;
        }

        if decimal.mantissa == 0.0 {
            return self;
        }

        let bigger_decimal;
        let smaller_decimal;

        if self.exponent >= decimal.exponent {
            bigger_decimal = self;
            smaller_decimal = decimal;
        } else {
            bigger_decimal = decimal;
            smaller_decimal = self;
        }

        if bigger_decimal.exponent - smaller_decimal.exponent > MAX_SIGNIFICANT_DIGITS as f64 {
            return bigger_decimal;
        }

        from_mantissa_exponent(
            (1e14 * bigger_decimal.mantissa)
                + 1e14
                    * &smaller_decimal.mantissa
                    * power_of_10((smaller_decimal.exponent - bigger_decimal.exponent) as i32),
            bigger_decimal.exponent - 14.0,
        )
    }
}

impl Add<&Decimal> for Decimal {
    type Output = Decimal;

    fn add(self, decimal: &Decimal) -> Decimal {
        self + *decimal
    }
}

impl Add<Decimal> for &Decimal {
    type Output = Decimal;

    fn add(self, decimal: Decimal) -> Decimal {
        *self + decimal
    }
}

impl Add<&Decimal> for &Decimal {
    type Output = Decimal;

    fn add(self, decimal: &Decimal) -> Decimal {
        *self + *decimal
    }
}

impl AddAssign<&Decimal> for Decimal {
    fn add_assign(&mut self, rhs: &Decimal) {
        *self = *self + rhs;
    }
}

impl AddAssign<Decimal> for Decimal {
    fn add_assign(&mut self, rhs: Decimal) {
        *self = *self + rhs;
    }
}

impl Sub<&Decimal> for &Decimal {
    type Output = Decimal;

    fn sub(self, decimal: &Decimal) -> Decimal {
        *self - *decimal
    }
}

impl Sub<&Decimal> for Decimal {
    type Output = Decimal;

    fn sub(self, decimal: &Decimal) -> Decimal {
        self - *decimal
    }
}

impl Sub<Decimal> for &Decimal {
    type Output = Decimal;

    fn sub(self, decimal: Decimal) -> Decimal {
        *self - decimal
    }
}

impl Sub<Decimal> for Decimal {
    type Output = Decimal;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, decimal: Decimal) -> Decimal {
        self + decimal.neg()
    }
}

impl SubAssign<&Decimal> for Decimal {
    fn sub_assign(&mut self, rhs: &Decimal) {
        *self = *self - rhs;
    }
}

impl SubAssign<Decimal> for Decimal {
    fn sub_assign(&mut self, rhs: Decimal) {
        *self = *self - rhs;
    }
}

impl Mul<Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, decimal: Decimal) -> Decimal {
        from_mantissa_exponent(
            self.mantissa * decimal.mantissa,
            self.exponent + decimal.exponent,
        )
    }
}

impl Mul<&Decimal> for Decimal {
    type Output = Decimal;

    fn mul(self, decimal: &Decimal) -> Decimal {
        self * *decimal
    }
}

impl Mul<Decimal> for &Decimal {
    type Output = Decimal;

    fn mul(self, decimal: Decimal) -> Decimal {
        *self * decimal
    }
}

impl Mul<&Decimal> for &Decimal {
    type Output = Decimal;

    fn mul(self, decimal: &Decimal) -> Decimal {
        *self * *decimal
    }
}

impl MulAssign<&Decimal> for Decimal {
    fn mul_assign(&mut self, rhs: &Decimal) {
        *self = *self * rhs;
    }
}

impl MulAssign<Decimal> for Decimal {
    fn mul_assign(&mut self, rhs: Decimal) {
        *self = *self * rhs;
    }
}

impl Div<Decimal> for Decimal {
    type Output = Decimal;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, decimal: Decimal) -> Decimal {
        self * decimal.recip()
    }
}

impl Div<&Decimal> for Decimal {
    type Output = Decimal;

    fn div(self, decimal: &Decimal) -> Decimal {
        self / *decimal
    }
}

impl Div<Decimal> for &Decimal {
    type Output = Decimal;

    fn div(self, decimal: Decimal) -> Decimal {
        *self / decimal
    }
}

impl Div<&Decimal> for &Decimal {
    type Output = Decimal;

    fn div(self, decimal: &Decimal) -> Decimal {
        *self / *decimal
    }
}

impl DivAssign<&Decimal> for Decimal {
    fn div_assign(&mut self, rhs: &Decimal) {
        *self = *self / rhs;
    }
}

impl DivAssign<Decimal> for Decimal {
    fn div_assign(&mut self, rhs: Decimal) {
        *self = *self / rhs;
    }
}

impl Neg for &Decimal {
    type Output = Decimal;

    fn neg(self) -> Decimal {
        from_mantissa_exponent_no_normalize(-self.mantissa, self.exponent)
    }
}

impl Neg for Decimal {
    type Output = Decimal;

    fn neg(self) -> Decimal {
        let decimal = &self.clone();
        from_mantissa_exponent_no_normalize(-decimal.mantissa, decimal.exponent)
    }
}
