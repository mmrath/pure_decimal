use serde;
use std::str::FromStr;
use std::fmt;

use super::pure_decimal::Decimal;


// Copied from https://github.com/paupino/rust-decimal/blob/master/src/serde_types.rs
// MIT LICENSE

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
        deserializer.deserialize_any(SerdeDecimalVisitor)
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


    fn visit_f64<E>(self, value: f64) -> Result<Decimal, E>
        where
            E: serde::de::Error,
    {
        use serde::de::Unexpected;
        Decimal::from_str(&value.to_string()).map_err(|_| E::invalid_value(Unexpected::Float(value), &self))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Decimal, E>
        where
            E: serde::de::Error,
    {
        Ok(Decimal::from(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Decimal, E>
        where
            E: serde::de::Error,
    {
        Ok(Decimal::from(value))
    }
}

#[cfg(test)]
mod tests {


    extern crate serde_json;
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct Record {
        amount: Decimal,
    }

    #[test]
    fn deserialize_valid_decimal() {
        let data = [
            ("{\"amount\":\"1.234\"}", "1.234"),
            ("{\"amount\":1234}", "1234"),
            ("{\"amount\":1234.56}", "1234.56"),
        ];
        for &(serialized, value) in data.iter() {
            let result = serde_json::from_str(serialized);
            assert_eq!(
                true,
                result.is_ok(),
                "expected successful deserialization for {}. Error: {:?}",
                serialized,
                result.err().unwrap()
            );
            let record: Record = result.unwrap();
            assert_eq!(
                value,
                record.amount.to_string(),
                "expected: {}, actual: {}",
                value,
                record.amount.to_string()
            );
        }
    }

    #[test]
    #[should_panic]
    fn deserialize_invalid_decimal() {
        let serialized = "{\"amount\":\"foo\"}";
        let _: Record = serde_json::from_str(serialized).unwrap();
    }

    #[test]
    fn serialize_decimal() {
        let record = Record { amount: dec!(1.234) };
        let serialized = serde_json::to_string(&record).unwrap();
        assert_eq!("{\"amount\":\"1.234\"}", serialized);
    }
}