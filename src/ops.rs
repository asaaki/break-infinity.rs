use crate::{
    from_mantissa_exponent, from_mantissa_exponent_no_normalize, power_of_10, Decimal,
    consts::MAX_SIGNIFICANT_DIGITS_F,
};

use std::borrow::Borrow;
pub use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[opimps::impl_ops(Add)]
fn add(self: Decimal, rhs: Decimal) -> Decimal {
    // Figure out which is bigger, shrink the mantissa of the smaller
    // by the difference in exponents, add mantissas, normalize and return

    if self.mantissa == 0.0 {
        return rhs.to_owned();
    }

    if rhs.mantissa == 0.0 {
        return self.to_owned();
    }

    let (bigger_decimal, smaller_decimal) = if self.exponent >= rhs.exponent {
        (self.borrow(), rhs.borrow())
    } else {
        (rhs.borrow(), self.borrow())
    };

    if bigger_decimal.exponent - smaller_decimal.exponent > MAX_SIGNIFICANT_DIGITS_F {
        return bigger_decimal.to_owned();
    }

    from_mantissa_exponent(
        (1e3 * bigger_decimal.mantissa)
            + (1e3
                * smaller_decimal.mantissa
                * power_of_10((smaller_decimal.exponent - bigger_decimal.exponent) as i32)),
        bigger_decimal.exponent - 3.0,
    )
}

#[opimps::impl_ops_assign(AddAssign)]
fn add_assign(self: Decimal, rhs: Decimal) {
    *self += rhs;
}

#[opimps::impl_ops(Sub)]
fn sub(self: Decimal, rhs: Decimal) -> Decimal {
    self + rhs.neg()
}

#[opimps::impl_ops_assign(SubAssign)]
fn sub_assign(self: Decimal, rhs: Decimal) {
    *self -= rhs;
}

#[opimps::impl_ops(Mul)]
fn mul(self: Decimal, rhs: Decimal) -> Decimal {
    from_mantissa_exponent(self.mantissa * rhs.mantissa, self.exponent + rhs.exponent)
}

#[opimps::impl_ops_assign(MulAssign)]
fn mul_assign(self: Decimal, rhs: Decimal) {
    *self *= rhs;
}

#[opimps::impl_ops(Div)]
fn div(self: Decimal, rhs: Decimal) -> Decimal {
    from_mantissa_exponent(self.mantissa / rhs.mantissa, self.exponent - rhs.exponent)
}

#[opimps::impl_ops_assign(DivAssign)]
fn div_assign(self: Decimal, rhs: Decimal) {
    *self /= rhs;
}

#[opimps::impl_uni_ops(Neg)]
fn neg(self: Decimal) -> Decimal {
    from_mantissa_exponent_no_normalize(-self.mantissa, self.exponent)
}
