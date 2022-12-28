use crate::Decimal;

pub const MAX_SAFE_INTEGER: f64 = 9007199254740991.0;

pub const MAX_SIGNIFICANT_DIGITS: u32 = 17;

// pub const EXP_LIMIT: f64 = 1.79e308;
pub const EXP_LIMIT: f64 = f64::MAX;

pub const NEG_EXP_LIMIT: f64 = -1.78e308;

/// Tolerance which is used for f64 conversion to compensate for floating-point error.
pub const ROUND_TOLERANCE: f64 = f64::EPSILON;

/// The smallest exponent that can appear in an f64, though not all mantissas are valid here.
pub const NUMBER_EXP_MIN: i32 = -324;

/// The largest exponent that can appear in an f64, though not all mantissas are valid here.
pub const NUMBER_EXP_MAX: i32 = 308;

/// The length of the cache used for powers of 10.
pub const LENGTH: usize = (NUMBER_EXP_MAX - NUMBER_EXP_MIN + 1) as usize;

// NOTE: consts NUMBER_EXP_MIN, NUMBER_EXP_MAX, and LENGTH need to be copied into the proc-macro crate
// "exports" a pregenerated `pub static CACHED_POWERS`
bie_proc_macros::insert_cache_for_powers_of_ten!();

pub const NAN: Decimal = Decimal {
    mantissa: f64::NAN,
    exponent: f64::NAN,
};

pub const INFINITY: Decimal = Decimal {
    mantissa: 1.0,
    exponent: f64::INFINITY,
};
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

pub const MAX: Decimal = Decimal {
    mantissa: 1.0,
    exponent: EXP_LIMIT,
};

pub const MIN: Decimal = Decimal {
    mantissa: -1.0,
    exponent: EXP_LIMIT,
};

/// The smallest positive number that can be represented by a Decimal.
pub const ALMOST_ZERO: Decimal = Decimal {
    mantissa: 1.0,
    exponent: NEG_EXP_LIMIT,
};

/// The largest negative number that can be represented by a Decimal.
pub const ALMOST_NEGATIVE_ZERO: Decimal = Decimal {
    mantissa: -1.0,
    exponent: NEG_EXP_LIMIT,
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
