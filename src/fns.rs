use crate::{consts::*, ops::*, Decimal};

/// Pads the given string with the fill string to the given max length.
pub fn pad_end(string: String, max_length: u32, fill_string: String) -> String {
    if f32::is_nan(max_length as f32) || f32::is_infinite(max_length as f32) {
        return string;
    }

    let length = string.chars().count() as u32;
    if length >= max_length {
        return string;
    }

    let mut filled = fill_string;
    if filled.is_empty() {
        filled = String::from(" ");
    }

    let fill_len = max_length - length;
    while filled.chars().count() < fill_len as usize {
        filled = format!("{}{}", filled, filled);
    }

    let truncated = if filled.chars().count() > fill_len as usize {
        String::from(&filled.as_str()[0..(fill_len as usize)])
    } else {
        filled
    };

    return string + truncated.as_str();
}

/// Formats the given number to the given number of significant digits.
pub fn to_fixed(num: f64, places: u32) -> String {
    format!("{:.*}", places as usize, num)
}

/// Formats the given number to the given number of significant digits and parses it back to a number.
pub fn to_fixed_num(num: f64, places: u32) -> f64 {
    to_fixed(num, places).parse::<f64>().unwrap()
}

/// Returns the power of 10 with the given exponent from the cache.
pub(crate) fn power_of_10(power: i32) -> f64 {
    CACHED_POWERS[(power - NUMBER_EXP_MIN) as usize]
}

/// Creates a new instance of Decimal with the given mantissa and exponent without normalizing them.
#[inline]
pub fn from_mantissa_exponent_no_normalize(mantissa: f64, exponent: f64) -> Decimal {
    Decimal { mantissa, exponent }
}

/// Creates a new instance of Decimal with the given mantissa and exponent with normalizing them.
#[inline]
pub fn from_mantissa_exponent(mantissa: f64, exponent: f64) -> Decimal {
    if !f64::is_finite(mantissa) || !f64::is_finite(exponent) {
        return NAN;
    }
    let decimal = from_mantissa_exponent_no_normalize(mantissa, exponent);
    decimal.normalize()
}

/// If you're willing to spend 'resourcesAvailable' and want to buy something
/// with exponentially increasing cost each purchase (start at priceStart,
/// multiply by priceRatio, already own currentOwned), how much of it can you buy?
///
/// Adapted from Trimps source code.
pub fn afford_geometric_series(
    resources_available: &Decimal,
    price_start: &Decimal,
    price_ratio: &Decimal,
    current_owned: &Decimal,
) -> Decimal {
    let actual_start = price_start * price_ratio.pow(current_owned);
    Decimal::new(
        (resources_available / actual_start * (price_ratio - ONE) + ONE).log10()
            / price_ratio.log10(),
    )
    .floor()
}

/// How much resource would it cost to buy (numItems) items if you already have currentOwned,
/// the initial price is priceStart and it multiplies by priceRatio each purchase?
pub fn sum_geometric_series(
    num_items: &Decimal,
    price_start: &Decimal,
    price_ratio: &Decimal,
    current_owned: &Decimal,
) -> Decimal {
    price_start * price_ratio.pow(current_owned) * (ONE - price_ratio.pow(num_items))
        / (ONE - price_ratio)
}

/// If you're willing to spend 'resourcesAvailable' and want to buy something with additively
/// increasing cost each purchase (start at priceStart, add by priceAdd, already own currentOwned),
/// how much of it can you buy?
pub fn afford_arithmetic_series(
    resources_available: &Decimal,
    price_start: &Decimal,
    price_add: &Decimal,
    current_owned: &Decimal,
) -> Decimal {
    //  n = (-(a-d/2) + sqrt((a-d/2)^2+2dS))/d
    //  where a is actual_start, d is price_add and S is resources_available
    //  then floor it and you're done!
    let actual_start = price_start + (current_owned * price_add);
    let b = actual_start - (price_add / TWO);
    let b2 = b.pow(&TWO);
    (b.neg() + ((b2 + ((price_add * resources_available) * TWO)).sqrt() / price_add)).floor()
}

/// How much resource would it cost to buy (numItems) items if you already have currentOwned,
/// the initial price is priceStart and it adds priceAdd each purchase?
/// Adapted from http://www.mathwords.com/a/arithmetic_series.htm
pub fn sum_arithmetic_series(
    num_items: &Decimal,
    price_start: &Decimal,
    price_add: &Decimal,
    current_owned: &Decimal,
) -> Decimal {
    let actual_start = price_start + (current_owned * price_add); // (n/2)*(2*a+(n-1)*d)

    num_items / TWO * (actual_start * TWO + (num_items - ONE) + num_items - ONE) * price_add
}

/// When comparing two purchases that cost (resource) and increase your resource/sec by (deltaRpS),
/// the lowest efficiency score is the better one to purchase.
///
/// From Frozen Cookies:
/// https://cookieclicker.wikia.com/wiki/Frozen_Cookies_(JavaScript_Add-on)#Efficiency.3F_What.27s_that.3F
pub fn efficiency_of_purchase(
    cost: &Decimal,
    current_rp_s: &Decimal,
    delta_rp_s: &Decimal,
) -> Decimal {
    cost / (current_rp_s + (cost / delta_rp_s))
}
