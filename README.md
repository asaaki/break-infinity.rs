# break-infinity.rs (extended)

_This is a fork of <https://github.com/Redfire75369/break-infinity.rs>; it includes refactorings and a few extensions and optimizations._

A port of [Patashu's break_infinity.js](https://github.com/Patashu/break_infinity.js) to Rust.

It provides the `Decimal` number struct which is able to reach a maximum value of `1e1.79e308` (<code>1\*10<sup>1.79\*10<sup>308</sup></sup></code>) instead of `f64`'s maximum of `1.79e308` (<code>1.79\*10<sup>308</sup></code>).

The **exponent** lies between [Uncentillion] (<code>10<sup>306</sup></code>) and [Duocentillion] (<code>10<sup>309</sup></code>), already an incredibly large number.

[Uncentillion]: https://googology.fandom.com/wiki/Uncentillion
[Duocentillion]: https://googology.fandom.com/wiki/Duocentillion

To write the final number, you would to have to write a `1` followed by (almost) _1.79 * 10<sup>308</sup>_ zeroes.

The biggest/smallest value (which does not parse into Infinity) with completely written out exponent is:

<code>10<sup>±178999999999999980000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000</sup></code>

Yep, that's a lot of digits.

It only needs two `f64`s internally, in other words: 128 bits (16 bytes) total in memory.

## Installation

You can install this package via Cargo by adding these lines to your `Cargo.toml`:

```toml
[dependencies]
break_infinity_extended = { git = "https://github.com/asaaki/break-infinity.rs" }
```

### Features

- `serde`: if you need to (de)serialize the values (useful for saving or transport)
- `compat`: enables break_infinity transformation functions; only useful when you transition between crates

## Usage

This library allows simple creation of `Decimal`'s through many different methods.

```rust
use break_infinity_extended as bie;

fn main() {
    let x = bie::Decimal::new(123.456);
    let y = bie::Decimal::from(123i32);
    let z = bie::from_mantissa_exponent(1.23, 9.0);
    let s = bie::Decimal::from("78.90");
}
```

Methods that return a `Decimal` can also be chained:

```rust
use break_infinity_extended as bie;

fn main() {
    let chained = x.abs().ceil().exp().log10();
}
```

<!-- For a complete list of functions and methods, refer to the [docs](https://docs.rs/break_infinity_extended). -->

## Acknowledgements

- _Redfire75369_ for creating the Rust implementation, which this is a fork of.
- _Patashu_ and _Razenpok_ for creating the original `break_infinity.js` that the original Rust version is based off of.
