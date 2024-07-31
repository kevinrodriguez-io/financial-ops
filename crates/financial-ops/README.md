# Financial Ops

This crate provides a set of operations for working with financial data, more specifically, avoiding
the usage of floating point types.

## Usage

```rust
use financial_ops::CheckedDecimalOperations;

fn test_add_decimals() {
    let a: u64 = 1_0000;
    let a_decimals = 4;
    let b: u64 = 2_00;
    let b_decimals = 2;

    let (result, decimals) = a.add_decimals_checked(b, a_decimals, b_decimals)?;
    assert_eq!(result, 3_0000);
    assert_eq!(decimals, 4);

    let a: u32 = 123_45;
    let a_decimals = 2;
    let b: u32 = 0_45;
    let b_decimals = 2;

    let (result, decimals) = a.add_decimals_checked(b, a_decimals, b_decimals)?;
    assert_eq!(result, 123_90);
    assert_eq!(decimals, 2);
}
```

Very useful when dealing with money or blockchain transactions.
