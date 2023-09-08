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
