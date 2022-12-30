use break_infinity as bi;
use break_infinity_extended::*;

const TEST_VALUES: [f64; 9] = [
    0.0,
    1.0,
    2.0,
    -1.0,
    42e9,
    f64::MIN,
    f64::MAX,
    f64::INFINITY,
    f64::NEG_INFINITY,
];

#[test]
fn from_bi_test() {
    for f in TEST_VALUES {
        let bi = bi::Decimal::new(f);
        let bie = from_bi(&bi);
        assert_eq!(bie, Decimal::new(f));
    }
}

#[test]
fn from_bi_str_test() {
    for f in TEST_VALUES {
        let bi = bi::Decimal::new(f);
        let bie = from_bi_str(&bi);
        assert_eq!(bie, Decimal::new(f));
    }
}

#[test]
fn to_bi_test() {
    for f in TEST_VALUES {
        let bie = Decimal::new(f);
        let bi = to_bi(bie);
        assert_eq!(bi, bi::Decimal::new(f));
    }
}
