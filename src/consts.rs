use crate::Decimal;

pub const MAX_SAFE_INTEGER: f64 = 9007199254740991.0;

pub const MAX_SIGNIFICANT_DIGITS: u32 = 17;
pub(crate) const MAX_SIGNIFICANT_DIGITS_F: f64 = 17.0;

pub const EXP_LIMIT: f64 = 1.79e308;
pub const NEG_EXP_LIMIT: f64 = -EXP_LIMIT;
// -1.79e308 + 1e292 = -1.7899999999999998e308; the smallest possible difference before rounding
pub(crate) const ALMOST_ZERO_EXP_LIMIT: f64 = NEG_EXP_LIMIT + 1e292;

/// Tolerance which is used for f64 conversion to compensate for floating-point error.
pub const ROUND_TOLERANCE: f64 = f64::EPSILON;

bie_proc_macros::insert_consts_and_cache_for_powers_of_ten!();

pub const NAN: Decimal = Decimal {
    mantissa: f64::NAN,
    exponent: f64::NAN,
};

// "true" infinity; internal use only
pub const INFINITY: Decimal = Decimal {
    mantissa: 1.0,
    exponent: f64::INFINITY,
};

// "true" -infinity; internal use only
pub const NEG_INFINITY: Decimal = Decimal {
    mantissa: -1.0,
    exponent: f64::INFINITY,
};

pub const ZERO: Decimal = Decimal {
    mantissa: 0.0,
    exponent: 0.0,
};

pub const ONE: Decimal = Decimal {
    mantissa: 1.0,
    exponent: 0.0,
};

pub const NEG_ONE: Decimal = Decimal {
    mantissa: -1.0,
    exponent: 0.0,
};

pub const TWO: Decimal = Decimal {
    mantissa: 2.0,
    exponent: 0.0,
};

pub const TEN: Decimal = Decimal {
    mantissa: 1.0,
    exponent: 1.0,
};

/// The largest positive number that can be represented by a Decimal
/// theoretically there is some space left both in mantissa and exponent,
/// but for practical and compatibility reasons we use the same values as in `break_infinity`
pub const MAX: Decimal = Decimal {
    mantissa: 1.0,
    exponent: EXP_LIMIT,
};

/// The smallest negative number that can be represented by a Decimal
/// theoretically there is some space left both in mantissa and exponent,
/// but for practical and compatibility reasons we use the same values as in `break_infinity`
pub const MIN: Decimal = Decimal {
    mantissa: -1.0,
    exponent: EXP_LIMIT,
};

/// The smallest positive number that can be represented by a Decimal;
/// the absolute value of the exponent has to be slightly smaller than EXP_LIMIT
pub const ALMOST_ZERO: Decimal = Decimal {
    mantissa: 1.0,
    exponent: ALMOST_ZERO_EXP_LIMIT,
};

/// The largest negative number that can be represented by a Decimal
/// the absolute value of the exponent has to be slightly smaller than EXP_LIMIT
pub const ALMOST_NEGATIVE_ZERO: Decimal = Decimal {
    mantissa: -1.0,
    exponent: ALMOST_ZERO_EXP_LIMIT,
};

pub const PI: Decimal = Decimal {
    mantissa: std::f64::consts::PI,
    exponent: 0.0,
};

pub const TAU: Decimal = Decimal {
    mantissa: std::f64::consts::TAU,
    exponent: 0.0,
};

pub const E: Decimal = Decimal {
    mantissa: std::f64::consts::E,
    exponent: 0.0,
};
