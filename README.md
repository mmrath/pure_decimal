# Pure Decimal

[![Travis](https://img.shields.io/travis/mmrath/pure_decimal.svg)](https://travis-ci.org/mmrath/pure_decimal)
![Downloads](https://img.shields.io/crates/d/pure_decimal.svg)
[![Crates.io](https://img.shields.io/crates/v/pure_decimal.svg)](https://crates.io/crates/pure_decimal)
![Apache license](https://img.shields.io/crates/l/pure_decimal.svg)

This crate provides a `Decimal` type which is a wrapper around [decimal](https://crates.io/crates/decimal). This `Decimal` does not contain [infinity](https://en.wikipedia.org/wiki/Infinity) and [NaN](https://en.wikipedia.org/wiki/NaN). The objects of this type is can be used as keys in Maps and can be ordered.


# Important notice

- I am sure of the correctness of this library and its usage as keys or sorting. There could be lot of edge case bugs.
- Performance is not the primary goal of this library. If you need the fastest decimal then this is not the right library

# Example

```rust
#[macro_use]
extern crate pure_decimal;

fn main() {
    let x = decimal!(1.234);
    let y = decimal!(1.111);
    let z = decimal!(2.345);
    assert_eq(x + y, z);
}
```


