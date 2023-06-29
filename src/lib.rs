#[cfg(feature = "double-precision")]
pub type Value = f64;

#[cfg(feature = "single-precision")]
pub type Value = f32;

mod dimension;
mod formatter;
mod measurement;
mod unit;
mod unit_converter;
pub mod units;
pub use dimension::*;
pub use formatter::*;
pub use measurement::*;
pub use unit::*;
pub use unit_converter::*;
