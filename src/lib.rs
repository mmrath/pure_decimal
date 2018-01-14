extern crate decimal;

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "serde")]
#[cfg(test)]
extern crate serde_json;

#[macro_use]
mod macros {
    /// A macro to construct Decimal literals.
    /// Note: that it will panic if invalid literals are provided as input.
    ///
    /// # Examples:
    /// ```
    /// # #[macro_use]
    /// # extern crate pure_decimal;
    /// # fn main() {
    /// assert!(decimal!(0).is_zero());
    /// assert!(decimal!(-0.1).is_negative());
    /// # }
    /// ```
    #[macro_export]
    macro_rules! decimal {
        ($lit:expr) => {{
            use std::str::FromStr;
            $crate::Decimal::from_str(stringify!($lit)).expect("Invalid decimal float literal")
        }}
    }
}

mod error;
mod pure_decimal;

pub use error::Error;
pub use pure_decimal::Decimal;
