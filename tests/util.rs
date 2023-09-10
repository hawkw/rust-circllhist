#[macro_export]
macro_rules! assert_approx_eq {
    ($left:expr, $right:expr $(,)?) => {
        let left = $left as f64;
        let right = $right as f64;

        let delta = (left / 100000.0f64).abs();
        assert!(
            left >= right - delta && left <= right + delta,
            "assertion failed: `(left ~= right)`\n  left: `{left:?}`,\n right: `{right:?}`",
        );
    };

    ($left:expr, $right:expr, $($arg:tt)+) => {
        let left = $left as f64;
        let right = $right as f64;

        let delta = (left / 100000.0f64).abs();
        assert!(
            left >= right - delta && left <= right + delta,
            "assertion failed: `(left ~= right)`\n  left: `{left:?}`,\n right: `{right:?}`: {}",
            format_args!($($arg)+)
        );
    };
}

// func helpQTest(t *testing.T, vals, qin, qexpect []float64) {
// 	h := hist.New()
// 	for _, sample := range vals {
// 		_ = h.RecordValue(sample)
// 	}
// 	qout, _ := h.ApproxQuantile(qin)
// 	if len(qout) != len(qexpect) {
// 		t.Errorf("wrong number of quantiles")
// 	}
// 	for i, q := range qout {
// 		if !fuzzyEquals(qexpect[i], q) {
// 			t.Errorf("q(%v) -> %v != %v", qin[i], q, qexpect[i])
// 		}
// 	}
// }
#[track_caller]
#[allow(dead_code)]
pub(crate) fn test_quantiles<const QS: usize>(
    vals: &[f64],
    quantiles: [f64; QS],
    expected_quantiles: [f64; QS],
) {
    let mut histogram = circllhist::Histogram::default();
    eprintln!("--- quantiles: {quantiles:?} ---");
    eprintln!("    vals: {vals:?}");

    for sample in vals {
        histogram
            .record(*sample)
            .expect("value should be recorded successfully");
    }

    eprintln!("expected: {expected_quantiles:?}");

    let actual_quantiles = histogram
        .approx_quantiles(&quantiles)
        .expect("quantiles should be calculated successfully");
    eprintln!("  actual: {actual_quantiles:?}");

    for (&quantile, (&actual, &expected)) in quantiles
        .iter()
        .zip(actual_quantiles.iter().zip(expected_quantiles.iter()))
    {
        assert_approx_eq!(
            actual,
            expected,
            "q({quantile}) -> {expected} != {actual}\n  hist: {histogram:#?}"
        );
    }
    eprintln!("--- ok!\n");
}

#[test]
fn approx_eq_succeeds() {
    assert_approx_eq!(0.1 + 0.2, 0.3);
}

#[test]
#[should_panic]
fn approx_eq_fails() {
    assert_approx_eq!(0.1, 0.2);
}

#[test]
fn approx_eq_msg_succeeds() {
    assert_approx_eq!(0.1 + 0.2, 0.3, "lol floating point");
}

#[test]
#[should_panic]
fn approx_eq_msg_fails() {
    assert_approx_eq!(0.1, 0.2, "lol floating point");
}
