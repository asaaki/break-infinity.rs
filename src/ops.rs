use crate::{
    consts::MAX_SIGNIFICANT_DIGITS_F, from_mantissa_exponent, from_mantissa_exponent_no_normalize,
    power_of_10, Decimal,
};

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

    if (self.exponent - rhs.exponent).abs() > MAX_SIGNIFICANT_DIGITS_F {
        if self.exponent >= rhs.exponent {
            return self.to_owned()
        };
        return rhs.to_owned()
    }

    if self.exponent > rhs.exponent {
        from_mantissa_exponent(
            (1e3 * self.mantissa)
                + (1e3 * rhs.mantissa * power_of_10((rhs.exponent - self.exponent) as i32)),
            self.exponent - 3.0,
        )
    } else if self.exponent < rhs.exponent {
        from_mantissa_exponent(
            (1e3 * rhs.mantissa)
                + (1e3 * self.mantissa * power_of_10((self.exponent - rhs.exponent) as i32)),
            rhs.exponent - 3.0,
        )
    } else {
        from_mantissa_exponent(1e3 * rhs.mantissa + 1e3 * self.mantissa, rhs.exponent - 3.0)
    }
}

#[opimps::impl_ops_assign(AddAssign)]
fn add_assign(self: Decimal, rhs: Decimal) {
    *self = self.add(rhs);
}

#[opimps::impl_ops(Sub)]
fn sub(self: Decimal, rhs: Decimal) -> Decimal {
    self.add(rhs.neg())
}

#[opimps::impl_ops_assign(SubAssign)]
fn sub_assign(self: Decimal, rhs: Decimal) {
    *self = self.add(rhs.neg());
}

#[opimps::impl_ops(Mul)]
fn mul(self: Decimal, rhs: Decimal) -> Decimal {
    from_mantissa_exponent(self.mantissa * rhs.mantissa, self.exponent + rhs.exponent)
}

#[opimps::impl_ops_assign(MulAssign)]
fn mul_assign(self: Decimal, rhs: Decimal) {
    *self = from_mantissa_exponent(self.mantissa * rhs.mantissa, self.exponent + rhs.exponent);
}

#[opimps::impl_ops(Div)]
fn div(self: Decimal, rhs: Decimal) -> Decimal {
    from_mantissa_exponent(self.mantissa / rhs.mantissa, self.exponent - rhs.exponent)
}

#[opimps::impl_ops_assign(DivAssign)]
fn div_assign(self: Decimal, rhs: Decimal) {
    *self = from_mantissa_exponent(self.mantissa / rhs.mantissa, self.exponent - rhs.exponent);
}

#[opimps::impl_uni_ops(Neg)]
fn neg(self: Decimal) -> Decimal {
    from_mantissa_exponent_no_normalize(-self.mantissa, self.exponent)
}
