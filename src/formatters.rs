use crate::{consts::*, Decimal};
use std::fmt::{self, Binary, Formatter};

// adjust values if more terms are added to fn scale_term(...)
const BEYOND_NEG_OFFSET: f64 = -10.0; // 3 * -3 - 1
const BEYOND_OFFSET: f64 = 154.0; // 3 * 51 + 1

// we abuse this formatter for scale term usage, since binary output of a decimal is not useful
impl Binary for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        } else if self.exponent < BEYOND_OFFSET && self.exponent > BEYOND_NEG_OFFSET {
            let (scale_index, factor) = scale_factor(self.exponent);
            let number = self.mantissa * (10.0_f64.powi(factor));
            let scale_term = short_scale_term(scale_index);
            let gap = if scale_index == 0 { "" } else { " " };
            return if let Some(places) = f.precision() {
                f.write_fmt(format_args!("{number:.places$}{gap}{scale_term}"))
            } else {
                f.write_fmt(format_args!("{number}{gap}{scale_term}"))
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

// returns a tuple with the scale term index and the factor to multiply the mantissa by
fn scale_factor(exponent: f64) -> (i32, i32) {
    (
        exponent.div_euclid(3.0) as i32,
        exponent.rem_euclid(3.0).abs() as i32,
    )
}

// https://swarmsim.fandom.com/wiki/Numbers?oldid=5918
fn short_scale_term(scale: i32) -> &'static str {
    match scale {
        // smaller presentations are usually not required in incremental/clicker games
        -3 => "n",  // 1000^-3
        -2 => "µ", // 1000^-2
        -1 => "m",  // 1000^-1
        0 => "",    // 1000^0
        1 => "k",   // 1000^1
        2 => "M",   // 1000^2
        3 => "B",   // 1000^3 ...
        4 => "T",
        5 => "Qa",
        6 => "Qi",
        7 => "Sx",
        8 => "Sp",
        9 => "Oc",
        10 => "No",
        11 => "Dc",
        12 => "UDc",
        13 => "DDc",
        14 => "TDc",
        15 => "QaDc",
        16 => "QiDc",
        17 => "SxDc",
        18 => "SpDc",
        19 => "OcDc",
        20 => "NDc",
        21 => "Vi",
        22 => "UVi",
        23 => "DVi",
        24 => "TVi",
        25 => "QaVi",
        26 => "QiVi",
        27 => "SxVi",
        28 => "SpVi",
        29 => "OcVi",
        30 => "NVi",
        31 => "Tg",
        32 => "UTg",
        33 => "DTg",
        34 => "TTg",
        35 => "QaTg",
        36 => "QiTg",
        37 => "SxTg",
        38 => "SpTg",
        39 => "OcTg",
        40 => "NTg",
        41 => "Qd",
        42 => "UQd",
        43 => "DQd",
        44 => "TQd",
        45 => "QaQd",
        46 => "QiQd",
        47 => "SxQd",
        48 => "SpQd",
        49 => "OcQd",
        50 => "NQd",
        51 => "Qq",
        // TODO: fill more terms
        // rule after each decade of a major -illion (x):
        // 0 x, 1 Ux, 2 Dx, 3 Tx, 4 Qax, 5 Qix, 6 Sxx, 7 Spx, 8 Ocx, 9 Nx
        // terms after scale 50:
        // 60 Sg, 70 St, 80 Og, 90 Ag, 100 Ct
        // after scale 100 combos: 110 DcCt, 111 UDcCt, ...
        // terms after scale 100:
        // 200 De, 300 Tc, 400 Qt, 500 Qn, 600 Ss, 700 Si, 800 Oe, 900 Ni, 1000 Mi
        _ => unreachable!(),
    }
}
