use decimal::d128;
use std::str::FromStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::default::Default;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub, AddAssign, SubAssign, MulAssign};
use std::borrow::Borrow;
use std::iter::Sum;

use super::error::Error;


///
/// # Examples:
/// ```
/// # #[macro_use]
/// # extern crate pure_decimal;
///
/// # use std::collections::BTreeMap;
///
/// # fn main() {
///
/// // Use as keys in BTree
/// let mut map = BTreeMap::new();
/// map.insert(dec!(1.0), dec!(1.0));
/// map.insert(dec!(1), dec!(2.0));
///
/// assert!(map.len() == 1);
/// assert!(map.contains_key(&dec!(1.00)));
/// assert!(map.get(&dec!(1.00)) == Some(&dec!(2.0)));
///
/// # }
/// ```
#[derive(Clone, Copy)]
pub struct Decimal(pub(crate) d128);

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
        self
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


macro_rules! unary_assign_op {
    ($(#[$attr:meta])* impl $op:ident, $method:ident, $t:ident) => {
        $(#[$attr])*
        impl $op<$t> for $t {
            fn $method(&mut self, other: $t) {
                (self.0).$method(other.0);
            }
        }
    }
}

unary_assign_op!(impl AddAssign, add_assign, Decimal);
unary_assign_op!(impl SubAssign, sub_assign, Decimal);
unary_assign_op!(impl MulAssign, mul_assign, Decimal);


impl<T> Sum<T> for Decimal where T: Borrow<Decimal> {
    fn sum<I: IntoIterator<Item=T>>(iter: I) -> Decimal {
        iter.into_iter()
            .fold(Decimal::zero(), |acc, val| acc + val.borrow())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        use std::str::FromStr;

        assert_eq!(Decimal::zero(), Decimal::default());
        assert_eq!(Decimal::zero(), Default::default());
        assert_eq!(Decimal::zero(), Decimal::from_str("0").unwrap());
        assert_eq!(Decimal::zero(), Decimal::from_str("0.0").unwrap());
    }

    #[test]
    fn unary_op() {
        assert_eq!(dec!(-1.1), -dec!(1.1));
        assert_eq!(dec!(-1.1), -&dec!(1.1));
    }

    #[test]
    fn binary_op() {
        assert_eq!(dec!(3.33), dec!(1.11) + dec!(2.22));
        assert_eq!(dec!(3.33), &dec!(1.11) + dec!(2.22));
        assert_eq!(dec!(3.33), dec!(1.11) + &dec!(2.22));
        assert_eq!(dec!(3.33), &dec!(1.11) + &dec!(2.22));
        //assert_eq!(dec!(5) << 2, dec!(500));
        //assert_eq!(dec!(500) >> 1, dec!(50));
    }

    #[test]
    fn as_ref_operand() {
        assert_eq!(dec!(1.1), dec!(1.1).min(dec!(2.2)));
        assert_eq!(dec!(1.1), dec!(1.1).min(&dec!(2.2)));
    }


    #[test]
    fn assign_op() {
        let mut x = dec!(1);
        x += dec!(2);
        assert_eq!(x, dec!(3));
        x *= dec!(3);
        assert_eq!(x, dec!(9));
        x -= dec!(1);
        assert_eq!(x, dec!(8));
    }

    #[test]
    fn test_sum() {
        let decimals = vec![dec!(1), dec!(2), dec!(3), dec!(4)];
        assert_eq!(dec!(10), decimals.iter().sum());
        assert_eq!(dec!(10), decimals.into_iter().sum());
    }
}
