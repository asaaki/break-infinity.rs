use consts::*;
use std::{
    cmp::Ordering::{self, *},
    f64::consts::{E, LN_10, LOG2_10, PI},
};

mod calculations;
mod cmp;
mod formatters;
mod from;
mod macros;
mod ops;
mod traits;
mod utils;

#[cfg(test)]
mod test;

// publicly exported modules and functions

pub mod consts;
pub use crate::{calculations::*, cmp::*, formatters::*, ops::*, traits::*, utils::*};

/// A struct representing a decimal number, which can reach a maximum of 1e1.79e308 instead of `f64`'s maximum of 1.79e308.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Decimal {
    mantissa: f64,
    exponent: f64,
}

impl Default for Decimal {
    fn default() -> Self {
        consts::ZERO
    }
}

impl Decimal {
    /// Creates a new instance of Decimal with the given value.
    pub fn new(value: f64) -> Decimal {
        // SAFETY: Handle Infinity and NaN in a somewhat meaningful way.
        if f64::is_nan(value) {
            return NAN;
        } else if value == 0.0 {
            return ZERO;
        } else if value == 1.0 {
            return ONE;
        } else if value == 2.0 {
            return TWO;
        } else if value == -1.0 {
            return NEG_ONE;
        } else if f64::is_infinite(value) && f64::is_sign_positive(value) {
            return MAX;
        } else if f64::is_infinite(value) && f64::is_sign_negative(value) {
            return MIN;
        }

        let exponent = value.abs().log10().floor();
        let mantissa = if (exponent - NUMBER_EXP_MIN as f64).abs() < f64::EPSILON {
            value * 10.0
                / ("1e".to_owned() + (NUMBER_EXP_MIN + 1).to_string().as_str())
                    .parse::<f64>()
                    .unwrap()
        } else {
            let power_10 = power_of_10(exponent as i32);
            // This essentially rounds the mantissa for very high numbers.
            ((value / power_10) * 1_000_000_000_000_000.0).round() / 1_000_000_000_000_000.0
        };
        let decimal = Decimal { mantissa, exponent };
        decimal.normalize()
    }

    /// Normalizes the mantissa when it is too denormalized.
    #[inline]
    fn normalize(&self) -> Decimal {
        if self.mantissa >= 1.0 && self.mantissa < 10.0 {
            return *self;
        } else if self.mantissa == 0.0 {
            return ZERO;
        }

        let temp_exponent = self.mantissa.abs().log10().floor();
        let mantissa = if (temp_exponent as i32) == NUMBER_EXP_MIN {
            self.mantissa * 10.0 / 1e-323
        } else {
            self.mantissa / power_of_10(temp_exponent as i32)
        };
        let exponent = self.exponent + temp_exponent;
        Decimal { mantissa, exponent }
    }

    pub const fn zero() -> Decimal {
        consts::ZERO
    }

    pub const fn one() -> Decimal {
        consts::ONE
    }

    pub const fn two() -> Decimal {
        consts::TWO
    }

    pub const fn neg_one() -> Decimal {
        consts::NEG_ONE
    }

    pub const fn minus_one() -> Decimal {
        consts::NEG_ONE
    }

    pub const fn max_value() -> Decimal {
        consts::MAX
    }

    pub const fn min_value() -> Decimal {
        consts::MIN
    }

    pub const fn nan() -> Decimal {
        consts::NAN
    }

    pub const fn infinity() -> Decimal {
        consts::INFINITY
    }

    pub const fn neg_infinity() -> Decimal {
        consts::NEG_INFINITY
    }

    pub const fn pi() -> Decimal {
        consts::PI
    }

    pub const fn tau() -> Decimal {
        consts::TAU
    }

    pub const fn e() -> Decimal {
        consts::E
    }

    #[inline]
    fn _to_f64(&self) -> f64 {
        //  Problem: new(116.0).to_number() returns 115.99999999999999.
        //  TODO: How to fix in general case? It's clear that if to_number() is
        //    VERY close to an integer, we want exactly the integer.
        //    But it's not clear how to specifically write that.
        //    So I'll just settle with 'exponent >= 0 and difference between rounded
        //    and not rounded < 1e-9' as a quick fix.
        //  var result = self.mantissa * 10.0_f64.powf(self.exponent);
        if !f64::is_finite(self.exponent) {
            return f64::NAN;
        }

        if self.exponent > NUMBER_EXP_MAX as f64 {
            return if self.mantissa > 0.0 {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            };
        }

        if self.exponent < NUMBER_EXP_MIN as f64 {
            return 0.0;
        }

        if (self.exponent - NUMBER_EXP_MIN as f64).abs() < f64::EPSILON {
            return if self.mantissa > 0.0 { 5e-324 } else { -5e-324 };
        }

        let result: f64 = self.mantissa * power_of_10(self.exponent as i32);

        if !f64::is_finite(result) || self.exponent < 0.0 {
            return result;
        }

        let result_rounded = result.round();

        if (result_rounded - result).abs() < ROUND_TOLERANCE {
            return result_rounded;
        }

        result
    }

    /// Converts the Decimal into a string.
    pub fn to_f64(&self) -> f64 {
        self._to_f64()
    }

    /// Converts the Decimal into a string.
    pub fn to_number(&self) -> f64 {
        self._to_f64()
    }

    /// Converts the Decimal into a string with the scientific notation.
    pub fn to_exponential(&self, mut places: u32) -> String {
        if f64::is_nan(self.mantissa) || f64::is_nan(self.exponent) {
            return String::from("NaN");
        } else if self.exponent >= EXP_LIMIT {
            return if self.mantissa > 0.0 {
                String::from("Infinity")
            } else {
                String::from("-Infinity")
            };
        }

        let tmp = pad_end(String::from("."), places + 1, String::from("0"));
        // 1) exponent is < 308 and > -324: use basic to_fixed
        // 2) everything else: we have to do it ourselves!
        if self.exponent <= -EXP_LIMIT || self.mantissa == 0.0 {
            let str = if places > 0 { tmp.as_str() } else { "" };
            return "0".to_owned() + str + "e+0";
        } else if !f32::is_finite(places as f32) {
            places = MAX_SIGNIFICANT_DIGITS;
        }

        let len = places + 1;
        let num_digits = self.mantissa.abs().log10().max(1.0) as u32;
        let rounded = (self.mantissa * 10.0_f64.powi(len as i32 - num_digits as i32)).round()
            * 10.0_f64.powi(num_digits as i32 - len as i32);
        return to_fixed(rounded, 0_u32.max(len - num_digits))
            + "e"
            + if self.exponent >= 0.0 { "+" } else { "" }
            + self.exponent.to_string().as_str();
    }

    /// Converts the Decimal into a string with the fixed notation.
    pub fn to_fixed(&self, places: u32) -> String {
        if f64::is_nan(self.mantissa) || f64::is_nan(self.exponent) {
            return String::from("NaN");
        } else if self.exponent >= EXP_LIMIT {
            return if self.mantissa > 0.0 {
                String::from("Infinity")
            } else {
                String::from("-Infinity")
            };
        }

        let tmp = pad_end(String::from("."), places + 1, String::from("0"));
        if self.exponent <= -EXP_LIMIT || self.mantissa == 0.0 {
            // Two Cases:
            // 1) exponent is 17 or greater: just print out mantissa with the appropriate number of zeroes after it
            // 2) exponent is 16 or less: use basic to_fixed
            let str = if places > 0 { tmp.as_str() } else { "" };
            return "0".to_owned() + str;
        } else if self.exponent >= MAX_SIGNIFICANT_DIGITS as f64 {
            let str = pad_end(
                self.mantissa.to_string().replace('.', ""),
                (self.exponent + 1.0) as u32,
                String::from("0"),
            ) + if places > 0 { tmp.as_str() } else { "" };
            return str;
        }

        to_fixed(self.to_number(), places)
    }

    /// Converts the Decimal into a string with the scientific notation if the exponent is greater than the precision.
    pub fn to_precision(&self, places: u32) -> String {
        if self.exponent <= -7.0 {
            return self.to_exponential(places - 1);
        }

        if (places as f64) > self.exponent {
            return self.to_fixed((places as f64 - self.exponent - 1.0) as u32);
        }

        self.to_exponential(places - 1)
    }

    /// Returns the mantissa with the specified precision.
    pub fn mantissa_with_decimal_places(&self, places: u32) -> f64 {
        // https://stackoverflow.com/a/37425022
        if f64::is_nan(self.mantissa) || f64::is_nan(self.exponent) {
            return f64::NAN;
        } else if self.mantissa == 0.0 {
            return 0.0;
        }

        let len = places + 1;
        let num_digits = self.mantissa.abs().log10().ceil() as u32;
        let rounded = (self.mantissa * 10.0_f64.powi(len as i32 - num_digits as i32)).round()
            * 10.0_f64.powi(num_digits as i32 - len as i32);
        to_fixed_num(rounded, 0.max(len - num_digits))
    }

    /// Returns the absolute value of the Decimal.
    pub fn abs(&self) -> Decimal {
        from_mantissa_exponent_no_normalize(self.mantissa.abs(), self.exponent)
    }

    #[inline]
    pub fn _sign(&self) -> i32 {
        if self.mantissa.is_sign_positive() {
            1
        } else if self.mantissa.is_sign_negative() {
            -1
        } else {
            0
        }
    }

    /// Returns the sign of the Decimal.
    pub fn sign(&self) -> i32 {
        self._sign()
    }

    /// Returns the sign of the Decimal.
    pub fn sgn(&self) -> i32 {
        self._sign()
    }

    /// Rounds the Decimal, if the exponent isn't greater than the maximum significant digits.
    pub fn round(&self) -> Decimal {
        if self.exponent < -1.0 {
            return ZERO;
        } else if self.exponent < MAX_SIGNIFICANT_DIGITS as f64 {
            return Decimal::new(self.to_number().round());
        }

        *self
    }

    /// Truncates the Decimal, if the exponent isn't greater than the maximum significant digits.
    pub fn trunc(&self) -> Decimal {
        if self.exponent < 0.0 {
            return ZERO;
        } else if self.exponent < MAX_SIGNIFICANT_DIGITS as f64 {
            return Decimal::new(self.to_number().trunc());
        }

        *self
    }

    /// Floors the Decimal, if the exponent isn't greater than the maximum significant digits.
    pub fn floor(&self) -> Decimal {
        if self.exponent < -1.0 {
            return if self.sign() >= 0 { ZERO } else { NEG_ONE };
        } else if self.exponent < MAX_SIGNIFICANT_DIGITS as f64 {
            return Decimal::new(self.to_number().floor());
        }

        *self
    }

    /// Rounds the Decimal to its ceiling, if the exponent isn't greater than the maximum significant digits.
    pub fn ceil(&self) -> Decimal {
        if self.exponent < -1.0 {
            return if self.sign() > 0 { ONE } else { ZERO };
        } else if self.exponent < MAX_SIGNIFICANT_DIGITS as f64 {
            return Decimal::new(self.to_number().ceil());
        }

        *self
    }

    #[inline]
    fn _reciprocal(&self) -> Decimal {
        from_mantissa_exponent(1.0 / self.mantissa, -self.exponent)
    }

    /// Returns the reciprocal of the Decimal.
    pub fn recip(&self) -> Decimal {
        self._reciprocal()
    }

    /// Returns the reciprocal of the Decimal.
    pub fn reciprocal(&self) -> Decimal {
        self._reciprocal()
    }

    /// Returns the reciprocal of the Decimal.
    pub fn reciprocate(&self) -> Decimal {
        self._reciprocal()
    }

    pub fn compare(&self, decimal: &Decimal) -> Option<Ordering> {
        self.partial_cmp(decimal)
    }

    pub fn equals(&self, decimal: &Decimal) -> bool {
        self.eq(decimal)
    }

    pub fn neq(&self, decimal: &Decimal) -> bool {
        !self.eq(decimal)
    }
    pub fn not_equals(&self, decimal: &Decimal) -> bool {
        !self.neq(decimal)
    }

    pub fn lt(&self, decimal: &Decimal) -> bool {
        self < decimal
    }

    pub fn lte(&self, decimal: &Decimal) -> bool {
        self <= decimal
    }

    pub fn gt(&self, decimal: &Decimal) -> bool {
        self > decimal
    }
    pub fn gte(&self, decimal: &Decimal) -> bool {
        self >= decimal
    }

    pub fn less_than_or_equal_to(&self, other: &Decimal) -> bool {
        self.lte(other)
    }

    pub fn less_than(&self, other: &Decimal) -> bool {
        self.lt(other)
    }

    pub fn greater_than_or_equal_to(&self, other: &Decimal) -> bool {
        self.gte(other)
    }

    pub fn greater_than(&self, other: &Decimal) -> bool {
        self.gt(other)
    }

    pub fn max(&self, other: &Decimal) -> Decimal {
        if self > other {
            *self
        } else {
            *other
        }
    }

    pub fn min(&self, other: &Decimal) -> Decimal {
        if self < other {
            *self
        } else {
            *other
        }
    }

    pub fn clamp(&self, min: &Decimal, max: &Decimal) -> Decimal {
        self.min(max).max(min)
    }

    pub fn cmp_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> Option<Ordering> {
        if self.eq_tolerance(decimal, tolerance) {
            Some(Equal)
        } else {
            self.partial_cmp(decimal)
        }
    }

    pub fn compare_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> Option<Ordering> {
        self.cmp_tolerance(decimal, tolerance)
    }

    #[inline]
    fn _eq_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        (self - decimal)
            .abs()
            .lte(&(tolerance * self.abs().max(&decimal.abs())))
    }

    /// Tolerance is a relative tolerance, multiplied by the greater of the magnitudes of the two arguments.
    /// For example, if you put in 1e-9, then any number closer to the
    /// larger number than (larger number) * 1e-9 will be considered equal.
    pub fn eq_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        self._eq_tolerance(decimal, tolerance)
    }

    pub fn equals_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        self._eq_tolerance(decimal, tolerance)
    }

    pub fn neq_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        !self._eq_tolerance(decimal, tolerance)
    }

    pub fn ne_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        !self._eq_tolerance(decimal, tolerance)
    }

    pub fn not_equals_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        !self._eq_tolerance(decimal, tolerance)
    }

    pub fn lt_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        !self._eq_tolerance(decimal, tolerance) && self.lt(decimal)
    }

    pub fn lte_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        self._eq_tolerance(decimal, tolerance) || self.lt(decimal)
    }

    pub fn gt_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        !self._eq_tolerance(decimal, tolerance) && self.gt(decimal)
    }

    pub fn gte_tolerance(&self, decimal: &Decimal, tolerance: &Decimal) -> bool {
        self._eq_tolerance(decimal, tolerance) || self.gt(decimal)
    }

    #[inline]
    pub fn log10(&self) -> f64 {
        self.exponent + self.mantissa.log10()
    }

    pub fn abs_log10(&self) -> f64 {
        self.exponent + self.mantissa.abs().log10()
    }

    pub fn p_log10(&self) -> f64 {
        if self.mantissa <= 0.0 || self.exponent < 0.0 {
            0.0
        } else {
            self.log10()
        }
    }

    #[inline]
    fn _log(&self, base: f64) -> f64 {
        // UN-SAFETY: Most incremental game cases are log(number := 1 or greater, base := 2 or greater).
        // We assume this to be true and thus only need to return a number, not a Decimal,
        LN_10 / base.ln() * self.log10()
    }

    pub fn log(&self, base: f64) -> f64 {
        self._log(base)
    }

    pub fn logarithm(&self, base: f64) -> f64 {
        self._log(base)
    }

    pub fn log2(&self) -> f64 {
        LOG2_10 * self.log10()
    }

    pub fn ln(&self) -> f64 {
        LN_10 * self.log10()
    }

    pub fn ln_1p(&self) -> f64 {
        LN_10 * (ONE + self).log10()
    }

    #[inline]
    fn _pow(&self, number: f64) -> Decimal {
        //  UN-SAFETY: Accuracy not guaranteed beyond ~9-11 decimal places.
        //  TODO: Decimal.pow(new Decimal(0.5), 0); or Decimal.pow(new Decimal(1), -1);
        //    makes an exponent of -0! Is a negative zero ever a problem?

        //  TODO: Fast track seems about neutral for performance.
        //    It might become faster if an integer pow is implemented,
        //    or it might not be worth doing (see https://github.com/Patashu/break_infinity.js/issues/4 )
        //  Fast track: If (this.e*value) is an integer and mantissa ^ value
        //  fits in a Number, we can do a very fast method.

        let temp = self.exponent * number;
        let mut new_mantissa;

        if temp < MAX_SAFE_INTEGER {
            // Same speed and usually more accurate.
            new_mantissa = self.mantissa.powf(number);

            if f64::is_finite(new_mantissa) && new_mantissa != 0.0 {
                return from_mantissa_exponent(new_mantissa, temp);
            }
        }

        let new_exponent = temp.trunc();
        let residue = temp - new_exponent;
        new_mantissa = 10.0_f64.powf(number * self.mantissa.log10() + residue);

        if f64::is_finite(new_mantissa) && new_mantissa != 0.0 {
            //  return Decimal.exp(value*this.ln());
            //  UN-SAFETY: This should return NaN when mantissa is negative and value is non-integer.
            return from_mantissa_exponent(new_mantissa, new_exponent);
        }

        let result = TEN.pow(&Decimal::new(number * self.abs_log10()));

        if self.sign() == -1 && (number % 2.0 - 1.0).abs() < f64::EPSILON {
            return result.neg();
        }

        result
    }

    /// Raises the Decimal to the power of the given Decimal.
    pub fn pow(&self, decimal: &Decimal) -> Decimal {
        self._pow(decimal.to_number())
    }

    pub fn powi(&self, exponent: i32) -> Decimal {
        self._pow(exponent as f64)
    }

    pub fn powf(&self, exponent: f64) -> Decimal {
        self._pow(exponent)
    }

    pub fn pow_base(&self, decimal: &Decimal) -> Decimal {
        decimal._pow(self.to_number())
    }

    pub fn powi_base(&self, number: i32) -> Decimal {
        Decimal::from(number)._pow(self.to_number())
    }

    pub fn powf_base(&self, number: f64) -> Decimal {
        Decimal::from(number)._pow(self.to_number())
    }

    pub fn factorial(&self) -> Decimal {
        //  Using Stirling's Approximation.
        //  https://en.wikipedia.org/wiki/Stirling%27s_approximation#Versions_suitable_for_calculators
        let n = self.to_number() + 1.0;
        Decimal::new(n / E * (n * f64::sinh(1.0 / n) + 1.0 / (810.0 * n.powi(6))))
            .pow(&Decimal::new(n))
            * Decimal::new(f64::sqrt(2.0 * PI / n))
    }

    pub fn exp(&self) -> Decimal {
        // Fast track: if -706 < this < 709, we can use regular exp.
        let number = self.to_number();
        if -706.0 < number && number < 709.0 {
            return Decimal::new(f64::exp(number));
        }
        Decimal::new(E).pow(self)
    }

    pub fn square(&self) -> Decimal {
        from_mantissa_exponent(self.mantissa.powi(2), self.exponent * 2.0)
    }

    pub fn sqr(&self) -> Decimal {
        from_mantissa_exponent(self.mantissa.powi(2), self.exponent * 2.0)
    }

    #[inline]
    fn _square_root(&self) -> Decimal {
        if self.mantissa < 0.0 {
            return Decimal::new(f64::NAN);
        } else if self.exponent % 2.0 != 0.0 {
            // Mod of a negative number is negative, so != means '1 or -1'
            return from_mantissa_exponent(
                f64::sqrt(self.mantissa) * 3.16227766016838,
                (self.exponent / 2.0).floor(),
            );
        }
        from_mantissa_exponent(f64::sqrt(self.mantissa), (self.exponent / 2.0).floor())
    }

    pub fn square_root(&self) -> Decimal {
        self._square_root()
    }

    pub fn sqrt(&self) -> Decimal {
        self._square_root()
    }

    pub fn cube(&self) -> Decimal {
        from_mantissa_exponent(self.mantissa.powi(3), self.exponent * 3.0)
    }

    #[inline]
    fn _cubic_root(&self) -> Decimal {
        let mut sign = 1;
        let mut mantissa = self.mantissa;

        if mantissa < 0.0 {
            sign = -1;
            mantissa = -mantissa;
        }

        let new_mantissa = sign as f64 * mantissa.powf((1 / 3) as f64);
        let remainder = (self.exponent % 3.0) as i32;

        if remainder == 1 || remainder == -1 {
            return from_mantissa_exponent(
                new_mantissa * 2.154_434_690_031_884,
                (self.exponent / 3.0).floor(),
            );
        }

        if remainder != 0 {
            // remainder != 0 at this point means 'remainder == 2 || remainder == -2'
            return from_mantissa_exponent(
                new_mantissa * 4.641_588_833_612_779,
                (self.exponent / 3.0).floor(),
            );
        }

        from_mantissa_exponent(new_mantissa, (self.exponent / 3.0).floor())
    }

    pub fn cubic_root(&self) -> Decimal {
        self._cubic_root()
    }

    pub fn cbrt(&self) -> Decimal {
        self._cubic_root()
    }

    pub fn mul_add(&self, a: &Decimal, b: &Decimal) -> Decimal {
        self.mul(a).add(b)
    }

    pub fn is_sign_positive(&self) -> bool {
        self.mantissa.is_sign_positive()
    }

    pub fn is_sign_negative(&self) -> bool {
        self.mantissa.is_sign_negative()
    }

    pub fn is_positive(&self) -> bool {
        self.mantissa.is_sign_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.mantissa.is_sign_negative()
    }

    // bits and bytes
    //
    // Might be useful for non-serde (de)serialization.
    //
    // NOTE: no matter the endianness of the machine,
    // the mantissa always comes first, then the exponent.

    pub fn from_bits(bits: &[u64; 2]) -> Decimal {
        Decimal {
            mantissa: f64::from_bits(bits[0]),
            exponent: f64::from_bits(bits[1]),
        }
    }

    pub fn to_bits(&self) -> [u64; 2] {
        [self.mantissa.to_bits(), self.exponent.to_bits()]
    }

    pub fn from_be_bytes(bytes: &[u8; 16]) -> Decimal {
        Decimal {
            mantissa: f64::from_be_bytes(bytes[0..8].try_into().unwrap()),
            exponent: f64::from_be_bytes(bytes[8..16].try_into().unwrap()),
        }
    }

    pub fn to_be_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&self.mantissa.to_be_bytes());
        bytes[8..16].copy_from_slice(&self.exponent.to_be_bytes());
        bytes
    }

    pub fn from_le_bytes(bytes: &[u8; 16]) -> Decimal {
        Decimal {
            mantissa: f64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            exponent: f64::from_le_bytes(bytes[8..16].try_into().unwrap()),
        }
    }

    pub fn to_le_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        bytes[0..8].copy_from_slice(&self.mantissa.to_le_bytes());
        bytes[8..16].copy_from_slice(&self.exponent.to_le_bytes());
        bytes
    }

    // Some hyperbolic trigonometry functions that happen to be easy

    pub fn sinh(&self) -> Decimal {
        (self.exp() - self.neg().exp()) / TWO
    }

    pub fn cosh(&self) -> Decimal {
        (self.exp() + self.neg().exp()) / TWO
    }

    pub fn tanh(&self) -> Decimal {
        self.sinh() / self.cosh()
    }

    pub fn asinh(&self) -> f64 {
        (self + (self.sqr() + ONE).sqrt()).ln()
    }

    pub fn acosh(&self) -> f64 {
        (self + (self.sqr() - ONE).sqrt()).ln()
    }

    pub fn atanh(&self) -> f64 {
        if self.abs().gte(&ONE) {
            return f64::NAN;
        }

        ((ONE + self) / (ONE - self)).ln() / 2.0
    }

    #[inline]
    fn _decimal_places(&self) -> i32 {
        if !f64::is_finite(self.mantissa) || self.exponent >= MAX_SIGNIFICANT_DIGITS as f64 {
            return 0;
        }

        let mantissa = self.mantissa;
        let mut places = -self.exponent as i32;
        let mut e = 1.0;

        while (mantissa * e).round().abs() / e - mantissa > ROUND_TOLERANCE {
            e *= 10.0;
            places += 1;
        }

        if places > 0 {
            places
        } else {
            0
        }
    }

    pub fn decimal_places(&self) -> i32 {
        self._decimal_places()
    }

    pub fn dp(&self) -> i32 {
        self._decimal_places()
    }

    // The joke functions section; enable with the 'jokes' feature

    /// Joke function from Realm Grinder
    #[cfg(feature = "jokes")]
    pub fn ascension_penalty(&self, ascensions: f64) -> Decimal {
        if ascensions == 0.0 {
            return *self;
        }

        self.pow(&Decimal::new(10.0_f64.powf(-ascensions)))
    }

    /// Joke function from Cookie Clicker. It's 'egg'
    #[cfg(feature = "jokes")]
    pub fn egg(&self) -> Decimal {
        self + Decimal::new(9.0)
    }
}
