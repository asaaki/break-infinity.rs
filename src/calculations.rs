use crate::{consts::*, ops::*, Decimal};

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
