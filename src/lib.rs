#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate decimal;

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(feature = "serde")]
#[cfg(test)]
extern crate serde_json;

#[cfg(feature = "serde")]
#[cfg(test)]
#[macro_use]
extern crate serde_derive;







#[macro_use]
mod macros {
    /// A macro to construct Decimal literals.
    /// Note: that it will panic if invalid literals are provided as input.
    ///
    /// # Examples:
    /// ```
    /// # #[macro_use]
    /// # extern crate pure_decimal;
    ///
    /// # use std::collections::BTreeMap;
    ///
    /// # fn main() {
    /// assert!(dec!(0).is_zero());
    /// assert!(dec!(-0.1).is_negative());
    ///
    /// # }
    /// ```
    #[macro_export]
    macro_rules! dec {
        ($lit:expr) => {{
            use std::str::FromStr;
            $crate::Decimal::from_str(stringify!($lit)).expect("Invalid decimal float literal")
        }}
    }
}

mod error;
mod pure_decimal;
mod serde_impl;

pub use error::Error;
pub use pure_decimal::Decimal;
pub use serde_impl::*;
