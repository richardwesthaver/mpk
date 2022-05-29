//! MPK_UTIL -- NUM

/// Uses [`EPSILON`](https://doc.rust-lang.org/core/f64/constant.EPSILON.html) to determine equality of two `f64`s.
#[macro_export]
macro_rules! f64_eq {
  ($l:ident, $r:literal) => {
    ($l - $r).abs() <= 8.0 * std::f64::EPSILON
  };
  ($l:ident, $r:ident) => {
    ($l - $r).abs() <= 8.0 * std::f64::EPSILON
  };
  ($l:expr, $r:literal) => {
    ($l - $r).abs() <= 8.0 * std::f64::EPSILON
  };
  ($l:expr, $r:expr) => {
    (($l) - ($r)).abs() <= 8.0 * std::f64::EPSILON
  };
}

/// Uses [`EPSILON`](https://doc.rust-lang.org/core/f64/constant.EPSILON.html) to determine inequality of two `f64`s.
///
/// This is exactly the same as saying `!f64_eq(x,y)` but it is slightly more efficient.
#[macro_export]
macro_rules! f64_ne {
  ($l:ident, $r:literal) => {
    ($l - $r).abs() > 8.0 * std::f64::EPSILON
  };
  ($l:ident, $r:ident) => {
    ($l - $r).abs() > 8.0 * std::f64::EPSILON
  };
  ($l:expr, $r:literal) => {
    ($l - $r).abs() > 8.0 * std::f64::EPSILON
  };
  ($l:expr, $r:expr) => {
    (($l) - ($r)).abs() > 8.0 * std::f64::EPSILON
  };
}
