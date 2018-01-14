use decimal::d128;
use std::str::FromStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::default::Default;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use serde;

use super::error::Error;

/// Decimal is a struct which encloses a `d128` from `decimal` create. There is no NaN and infinity
/// represented by this crate. All the method from Decimal are delegated to `d128`
#[derive(Clone, Copy)]
pub struct Decimal(d128);

impl Decimal {
    /// Creates a Decimal with `0` as value
    pub fn zero() -> Self {
        Decimal(d128::zero())
    }

    /// returns the larger of `self` and `other`
    pub fn max<O: AsRef<Decimal>>(self, other: O) -> Decimal {
        Decimal(self.0.max(other.as_ref().0))
    }

    /// returns the smaller of `self` and `other`
    pub fn min<O: AsRef<Decimal>>(self, other: O) -> Decimal {
        Decimal(self.0.min(other.as_ref().0))
    }

    /// returns absolute value of `self`
    pub fn abs(&self) -> Decimal {
        Decimal(self.0.abs())
    }

    /// Calculates the fused multiply-add `self` × `a` + `b` and returns the result. The multiply
    /// is carried out first and is exact, so this operation has only the one, final, rounding.
    pub fn mul_add<O: AsRef<Decimal>>(self, a: O, b: O) -> Decimal {
        Decimal(self.0.mul_add(a.as_ref().0, b.as_ref().0))
    }

    /// returns true if `self` is less than zero
    pub fn is_negative(&self) -> bool {
        self.0.is_negative()
    }

    /// returns true if `self` is zero
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// see ```[decimal::d128::pow]```
    pub fn pow<O: AsRef<Decimal>>(self, exp: O) -> Decimal {
        Decimal(self.as_ref().0.pow(exp.as_ref().0))
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self::zero()
    }
}

impl Hash for Decimal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl FromStr for Decimal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let val = d128::from_str(s);
        if val.is_ok() {
            let dec = val.unwrap();
            if dec.is_nan() {
                Err(Error::new("NaN is not supported"))
            } else if dec.is_infinite() {
                Err(Error::new("Infinity is not supported"))
            } else {
                Ok(Decimal(dec))
            }
        } else {
            Err(Error::new("Failed to parse"))
        }
    }
}

/// Delegates to d128.
impl fmt::Display for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

/// Delegates to d128.
impl fmt::Debug for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

/// Delegates to d128.
impl fmt::LowerExp for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

/// Delegates to d128.
impl fmt::LowerHex for Decimal {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::de::Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(SerdeDecimalVisitor)
    }
}

#[cfg(feature = "serde")]
#[allow(non_camel_case_types)]
struct SerdeDecimalVisitor;

#[cfg(feature = "serde")]
impl<'de> serde::de::Visitor<'de> for SerdeDecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a Decimal value")
    }

    fn visit_str<E>(self, s: &str) -> Result<Decimal, E>
    where
        E: serde::de::Error,
    {
        use serde::de::Unexpected;
        Decimal::from_str(s).map_err(|_| E::invalid_value(Unexpected::Str(s), &self))
    }
}

impl PartialEq<Decimal> for Decimal {
    fn eq(&self, other: &Decimal) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for Decimal {}

impl PartialOrd<Decimal> for Decimal {
    fn partial_cmp(&self, other: &Decimal) -> Option<::std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Decimal) -> ::std::cmp::Ordering {
        let res = self.0.partial_cmp(&other.0);
        match res {
            None => panic!("Ordering not possible. Possible bug"),
            Some(ord) => ord,
        }
    }
}

impl From<i32> for Decimal {
    fn from(val: i32) -> Decimal {
        Decimal(d128::from(val))
    }
}

/// Converts an u32 to d128. The result is exact and no error is possible.
impl From<u32> for Decimal {
    fn from(val: u32) -> Decimal {
        Decimal(d128::from(val))
    }
}

/// Converts an u64 to d128. The result is exact and no error is possible.
impl From<u64> for Decimal {
    fn from(val: u64) -> Decimal {
        Decimal(d128::from(val))
    }
}

/// Converts an i64 to d128. The result is exact and no error is possible.
impl From<i64> for Decimal {
    fn from(val: i64) -> Decimal {
        Decimal(d128::from(val))
    }
}

impl AsRef<Decimal> for Decimal {
    fn as_ref(&self) -> &Decimal {
        &self
    }
}

macro_rules! unary_op {
    ($(#[$attr:meta])* impl $op:ident, $method:ident) => {
        $(#[$attr])*
        impl $op for Decimal {
            type Output = Decimal;

            fn $method(self) -> Decimal {
                Decimal((self.0).$method())
            }
        }

        impl<'a> $op for &'a Decimal {
            type Output = Decimal;

            fn $method(self) -> Decimal {
                Decimal((self.0).$method())
            }
        }
    }
}

unary_op!(impl Neg, neg);

macro_rules! binary_op {
    ($(#[$attr:meta])* impl $op:ident, $method:ident, $t:ident) => {
        $(#[$attr])*
        impl $op<$t> for $t {
            type Output = $t;

            fn $method(self, other: $t) -> $t {
                Decimal((self.0).$method(other.0))
            }
        }

        impl<'a> $op<$t> for &'a $t {
            type Output = $t;

            fn $method(self, other: $t) -> $t {
                Decimal((self.0).$method(other.0))
            }
        }

        impl<'a> $op<&'a$t> for $t {
            type Output = $t;

            fn $method(self, other: &'a $t) -> $t {
                Decimal((self.0).$method(other.0))
            }
        }

        impl<'a, 'b> $op<&'a $t> for &'b $t {
            type Output = $t;

            fn $method(self, other: &'a $t) -> $t {
                Decimal((self.0).$method(other.0))
            }
        }
    }
}

binary_op!(impl Add, add, Decimal);
binary_op!(impl Sub, sub, Decimal);
binary_op!(impl Mul, mul, Decimal);

macro_rules! guarded_binary_op {
    ($(#[$attr:meta])* impl $op:ident, $method:ident, $t:ident) => {


        $(#[$attr])*
        impl $op<$t> for $t {
            type Output = ::std::result::Result<$t, Error>;

            /// Returns `Ok(Decimal)` if result is finite `Err` otherwise
            fn $method(self, other: $t) -> ::std::result::Result<$t, Error> {
                let val = (self.0).$method(other.0);
                if val.is_finite() {
                    Ok(Decimal(val))
                }else{
                 Err(Error::new("Only finite value are supported"))
                }
            }
        }

        impl<'a> $op<$t> for &'a $t {
            type Output = ::std::result::Result<$t, Error>;

            /// Returns `Ok(Decimal)` if result is finite `Err` otherwise
            fn $method(self, other: $t) -> ::std::result::Result<$t, Error> {
                let val = (self.0).$method(other.0);
                if val.is_finite() {
                    Ok(Decimal(val))
                }else{
                 Err(Error::new("Only finite value are supported"))
                }
            }
        }

        impl<'a> $op<&'a$t> for $t {
            type Output = ::std::result::Result<$t, Error>;

            /// Returns `Ok(Decimal)` if result is finite `Err` otherwise
            fn $method(self, other: &'a $t) -> ::std::result::Result<$t, Error> {
                let val = (self.0).$method(other.0);
                if val.is_finite() {
                    Ok(Decimal(val))
                }else{
                 Err(Error::new("Only finite value are supported"))
                }
            }
        }

        impl<'a, 'b> $op<&'a $t> for &'b $t {
            type Output = ::std::result::Result<$t, Error>;

            /// Returns `Ok(Decimal)` if result is finite `Err` otherwise
            fn $method(self, other: &'a $t) -> ::std::result::Result<$t, Error> {
                let val = (self.0).$method(other.0);
                if val.is_finite() {
                    Ok(Decimal(val))
                }else{
                 Err(Error::new("Only finite value are supported"))
                }
            }
        }
    }
}

guarded_binary_op!(impl Div, div, Decimal);
guarded_binary_op!(impl Rem, rem, Decimal);

#[cfg(test)]
mod tests {

    use super::*;

    #[cfg(feature = "serde")]
    use serde_json::{from_str, to_string};

    #[cfg(feature = "serde")]
    use std::collections::BTreeMap;

    #[test]
    fn default() {
        use std::str::FromStr;

        assert_eq!(Decimal::zero(), Decimal::default());
        assert_eq!(Decimal::zero(), Default::default());
        assert_eq!(Decimal::zero(), Decimal::from_str("0").unwrap());
        assert_eq!(Decimal::zero(), Decimal::from_str("0.0").unwrap());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let mut a = BTreeMap::new();
        a.insert("price".to_string(), d128!(432.232));
        a.insert("amt".to_string(), d128!(9.9));
        assert_eq!(
            &to_string(&a).unwrap(),
            "{\"amt\":\"9.9\",\"price\":\"432.232\"}"
        );
        let b = from_str("{\"price\":\"432.232\",\"amt\":\"9.9\"}").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn unary_op() {
        assert_eq!(decimal!(-1.1), -decimal!(1.1));
        assert_eq!(decimal!(-1.1), -&decimal!(1.1));
    }

    #[test]
    fn binary_op() {
        assert_eq!(decimal!(3.33), decimal!(1.11) + decimal!(2.22));
        assert_eq!(decimal!(3.33), &decimal!(1.11) + decimal!(2.22));
        assert_eq!(decimal!(3.33), decimal!(1.11) + &decimal!(2.22));
        assert_eq!(decimal!(3.33), &decimal!(1.11) + &decimal!(2.22));
        //assert_eq!(decimal!(5) << 2, decimal!(500));
        //assert_eq!(decimal!(500) >> 1, decimal!(50));
    }

    #[test]
    fn as_ref_operand() {
        assert_eq!(decimal!(1.1), decimal!(1.1).min(decimal!(2.2)));
        assert_eq!(decimal!(1.1), decimal!(1.1).min(&decimal!(2.2)));
    }

}
