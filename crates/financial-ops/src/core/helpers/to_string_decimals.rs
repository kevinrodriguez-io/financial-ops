use std::fmt;

use crate::core::PadToWidth;

/// A trait for converting a value to a string representation with a specified number of decimals.
pub trait ToStringDecimals {
    /// Converts the value to a string representation with the specified number of decimals.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to convert.
    /// * `decimals` - The number of decimals to include in the string representation.
    ///
    /// # Returns
    ///
    /// A string representation of the value with the specified number of decimals.
    fn to_string_decimals(self, decimals: u32) -> String;
}
impl<T> ToStringDecimals for T
where
    T: Copy + Into<f64> + fmt::Display,
{
    fn to_string_decimals(self, decimals: u32) -> String {
        let ten = 10f64;
        let value: f64 = self.into();
        let integer_part = (value / ten.powi(decimals as i32)) as u64;
        let fractional_part = (value % ten.powi(decimals as i32)) as u64;
        format!(
            "{}.{}",
            integer_part,
            fractional_part
                .to_string()
                .pad_to_width(decimals as usize, '0')
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_decimals() {
        // Test with positive integer value
        let value1: u32 = 123456789;
        assert_eq!(value1.to_string_decimals(2), "1234567.89");

        // Test with zero value
        let value4: u32 = 0;
        assert_eq!(value4.to_string_decimals(5), "0.00000");
    }
}
