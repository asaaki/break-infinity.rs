use break_infinity_extended::*;

#[test]
fn decimal() {
    assert_eq!(Decimal::new(0.0).to_string(), "0");
    assert_eq!(Decimal::new(f64::NAN).to_string(), "NaN");
    assert_eq!(Decimal::new(f64::INFINITY).to_string(), "Infinity");
    assert_eq!(Decimal::new(f64::NEG_INFINITY).to_string(), "-Infinity");

    assert_eq!(Decimal::new(100.0).to_string(), "100");
    assert_eq!(Decimal::new(1e12).to_string(), "1000000000000");
    assert_eq!(Decimal::new(1.79e3).to_string(), "1790");
    assert_eq!(Decimal::new(1e308).to_string(), "1.0000000000000000e+308");

    assert_eq!(consts::ALMOST_ZERO.to_string(), "1.0000000000000000e-178000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
    assert_eq!(consts::ALMOST_NEGATIVE_ZERO.to_string(), "-1.0000000000000000e-178000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
}

#[test]
fn formatters() {
    assert_eq!(format!("{:.3b}", Decimal::new(1e-10)), "1.000e-10");
    assert_eq!(format!("{:b}", Decimal::new(1e-9)), "1 n");
    assert_eq!(format!("{:b}", Decimal::new(0.000001)), "1 µ");
    assert_eq!(format!("{:b}", Decimal::new(0.001)), "1 m");
    assert_eq!(format!("{:b}", Decimal::new(1.0)), "1");
    assert_eq!(format!("{:b}", Decimal::new(1000.0)), "1 k");
    assert_eq!(format!("{:b}", Decimal::new(1000000.0)), "1 M");
    assert_eq!(format!("{:b}", Decimal::new(1e9)), "1 B");
    assert_eq!(format!("{:.1b}", Decimal::new(1e10)), "10.0 B");
    assert_eq!(format!("{:.2b}", Decimal::new(1e11)), "100.00 B");
    assert_eq!(format!("{:.3b}", Decimal::new(1.23456789e11)), "123.457 B");
    assert_eq!(format!("{:b}", Decimal::new(1e12)), "1 T");
    assert_eq!(format!("{:b}", Decimal::new(1e15)), "1 Qa");
    assert_eq!(format!("{:b}", Decimal::new(1e153)), "1 Qq");
    assert_eq!(format!("{:.3b}", Decimal::new(1e154)), "1.000e+154");
}

#[test]
fn ops() {
    let a = from_mantissa_exponent_no_normalize(3.224, 54.0);
    let b = from_mantissa_exponent_no_normalize(1.24, 53.0);
    let c = from_mantissa_exponent_no_normalize(3.1, 52.0);

    assert_eq!(a + b, from_mantissa_exponent_no_normalize(3.348, 54.0));
    assert_eq!(a - b, from_mantissa_exponent_no_normalize(3.1, 54.0));
    assert_eq!(
        a * b,
        from_mantissa_exponent_no_normalize(3.9977600000000004, 107.0)
    );
    assert_eq!(a / b, from_mantissa_exponent_no_normalize(2.6, 1.0));

    assert_eq!(a + c, from_mantissa_exponent_no_normalize(3.255, 54.0));
    assert_eq!(a - c, from_mantissa_exponent_no_normalize(3.193, 54.0));
    assert_eq!(a * c, from_mantissa_exponent_no_normalize(9.9944, 106.0));
    assert_eq!(a / c, from_mantissa_exponent_no_normalize(1.04, 2.0));

    assert_eq!(b + c, from_mantissa_exponent_no_normalize(1.55, 53.0));
    assert_eq!(b - c, from_mantissa_exponent_no_normalize(9.3, 52.0));
    assert_eq!(b * c, from_mantissa_exponent_no_normalize(3.844, 105.0));
    assert_eq!(
        b / c,
        from_mantissa_exponent_no_normalize(3.9999999999999996, 0.0)
    );

    assert_eq!(Decimal::new(1.0) + Decimal::new(0.0), Decimal::new(1.0));
    assert_eq!(Decimal::new(1.0) + Decimal::new(0.0), consts::ONE);
    assert_eq!(consts::ONE + consts::ZERO, Decimal::new(1.0));
}

#[test]
fn cmp() {
    let a = from_mantissa_exponent_no_normalize(3.224, 54.0);
    let b = from_mantissa_exponent_no_normalize(1.24, 53.0);
    let c = from_mantissa_exponent_no_normalize(3.1, 52.0);
    let d = from_mantissa_exponent_no_normalize(3.224, 54.0);

    assert!(a != b);
    assert!(a == d);
    assert!(b != d);

    assert!(a >= b);
    assert!(a >= d);
    assert!(!(b >= d));

    assert!(a > b);
    assert!(!(a > d));
    assert!(!(b > d));

    assert!(!(a <= b));
    assert!(a <= d);
    assert!(b <= d);

    assert!(!(a < b));
    assert!(!(a < d));
    assert!(b < d);

    assert_eq!(a.max(&b), a);
    assert_eq!(a.max(&c), a);
    assert_eq!(b.max(&c), b);

    assert_eq!(a.min(&b), b);
    assert_eq!(a.min(&c), c);
    assert_eq!(b.min(&c), c);

    assert_eq!(a.clamp(&c, &b), b);
    assert_eq!(b.clamp(&c, &a), b);
    assert_eq!(c.clamp(&b, &b), b);
}

#[test]
fn neg_abs() {
    assert_eq!(
        -Decimal::new(456.7),
        from_mantissa_exponent_no_normalize(-4.567, 2.0)
    );
    assert_eq!(
        -Decimal::new(1.23e48),
        from_mantissa_exponent_no_normalize(-1.23, 48.0)
    );

    assert_eq!(
        Decimal::new(-456.7).abs(),
        from_mantissa_exponent_no_normalize(4.567, 2.0)
    );
    assert_eq!(
        Decimal::new(-1.23e48).abs(),
        from_mantissa_exponent_no_normalize(1.23, 48.0)
    );
}

#[test]
fn tolerances() {
    let a = Decimal::new(1.000000001);
    let b = Decimal::new(1.000000002);
    let t0 = Decimal::new(0.0);
    let t1 = Decimal::new(0.000000001);

    assert!(!a.eq_tolerance(&b, &t0));
    assert!(a.eq_tolerance(&b, &t1));

    assert!(a.neq_tolerance(&b, &t0));
    assert!(!a.neq_tolerance(&b, &t1));

    assert!(a.lt_tolerance(&b, &t0));
    assert!(!a.lt_tolerance(&b, &t1));

    assert!(a.lte_tolerance(&b, &t0));
    assert!(a.lte_tolerance(&b, &t1));

    assert!(b.gt_tolerance(&a, &t0));
    assert!(!b.gt_tolerance(&a, &t1));

    assert!(b.gte_tolerance(&a, &t0));
    assert!(b.gte_tolerance(&a, &t1));
}

#[test]
fn constants() {
    assert_eq!(consts::ZERO, Decimal::new(0.0));
    assert_eq!(consts::ONE, Decimal::new(1.0));
    assert_eq!(consts::TWO, Decimal::new(2.0));
    assert_eq!(consts::TEN, Decimal::new(10.0));
    assert_eq!(consts::NEG_ONE, Decimal::new(-1.0));
    assert_eq!(consts::PI.to_number(), std::f64::consts::PI);
    assert_eq!(consts::TAU.to_number(), std::f64::consts::TAU);
    assert_eq!(consts::E.to_number(), std::f64::consts::E);
}
