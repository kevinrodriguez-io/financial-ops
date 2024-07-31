use crate::{
    core::{CheckedAdd, CheckedDiv, CheckedMul, CheckedRem, CheckedSub, DecimalOperationError},
    impl_checked_arithmetic,
};

impl_checked_arithmetic! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize }

/// A trait for performing checked decimal operations.
pub trait CheckedDecimalOperations {
    /// Adds two values with decimals and returns the result along with the number of decimals in the result.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value to add.
    /// * `other` - The second value to add.
    /// * `self_decimals` - The number of decimals in the first value.
    /// * `other_decimals` - The number of decimals in the second value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the sum of the values and the number of decimals in the result,
    /// or a `DecimalOperationError` if the operation fails.
    fn add_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError>
    where
        Self: Sized;

    /// Subtracts two values with decimals and returns the result along with the number of decimals in the result.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to subtract from.
    /// * `other` - The value to subtract.
    /// * `self_decimals` - The number of decimals in the first value.
    /// * `other_decimals` - The number of decimals in the second value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the difference of the values and the number of decimals in the result,
    /// or a `DecimalOperationError` if the operation fails.
    fn sub_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError>
    where
        Self: Sized;

    /// Multiplies two values with decimals and returns the result along with the number of decimals in the result.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value to multiply.
    /// * `other` - The second value to multiply.
    /// * `self_decimals` - The number of decimals in the first value.
    /// * `other_decimals` - The number of decimals in the second value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the product of the values and the number of decimals in the result,
    /// or a `DecimalOperationError` if the operation fails.
    fn multiply_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError>
    where
        Self: Sized;

    /// Divides two values with decimals and returns the result along with the number of decimals in the result.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to divide.
    /// * `other` - The value to divide by.
    /// * `self_decimals` - The number of decimals in the first value.
    /// * `other_decimals` - The number of decimals in the second value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the quotient of the values and the number of decimals in the result,
    /// or a `DecimalOperationError` if the operation fails.
    fn divide_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError>
    where
        Self: Sized;

    /// Calculates the remainder of dividing two values with decimals and returns the result along with the number of decimals in the result.
    ///
    /// # Arguments
    ///
    /// * `self` - The value to calculate the remainder for.
    /// * `other` - The value to divide by.
    /// * `self_decimals` - The number of decimals in the first value.
    /// * `other_decimals` - The number of decimals in the second value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the remainder of the division and the number of decimals in the result,
    /// or a `DecimalOperationError` if the operation fails.
    fn rem_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError>
    where
        Self: Sized;
}

// Blanket implementation of the DecimalOps trait for all types implementing numeric operations
impl<T> CheckedDecimalOperations for T
where
    T: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + CheckedRem + From<u32>,
{
    fn add_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError> {
        if self_decimals > other_decimals {
            let factor = T::from(10u32.pow(self_decimals - other_decimals));
            match self.checked_add(
                &other
                    .checked_mul(&factor)
                    .ok_or(DecimalOperationError::Overflow)?,
            ) {
                Some(value) => Ok((value, self_decimals)),
                None => Err(DecimalOperationError::Overflow),
            }
        } else {
            let factor = T::from(10u32.pow(other_decimals - self_decimals));
            match self
                .checked_mul(&factor)
                .and_then(|x| x.checked_add(&other))
            {
                Some(value) => Ok((value, other_decimals)),
                None => Err(DecimalOperationError::Overflow),
            }
        }
    }

    fn sub_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError> {
        if self_decimals > other_decimals {
            let factor = T::from(10u32.pow(self_decimals - other_decimals));
            match self.checked_sub(
                &other
                    .checked_mul(&factor)
                    .ok_or(DecimalOperationError::Overflow)?,
            ) {
                Some(value) => Ok((value, self_decimals)),
                None => Err(DecimalOperationError::Overflow),
            }
        } else {
            let factor = T::from(10u32.pow(other_decimals - self_decimals));
            match self
                .checked_mul(&factor)
                .and_then(|x| x.checked_sub(&other))
            {
                Some(value) => Ok((value, other_decimals)),
                None => Err(DecimalOperationError::Overflow),
            }
        }
    }

    fn multiply_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError> {
        match self.checked_mul(&other) {
            Some(value) => Ok((value, self_decimals + other_decimals)),
            None => Err(DecimalOperationError::Overflow),
        }
    }

    fn divide_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError> {
        let factor = T::from(10u32.pow(other_decimals));
        let adjusted_value = self
            .checked_mul(&factor)
            .ok_or(DecimalOperationError::Overflow)?;
        match adjusted_value.checked_div(&other) {
            Some(value) => Ok((value, self_decimals)),
            None => Err(DecimalOperationError::DivisionByZero),
        }
    }

    fn rem_decimals_checked(
        self,
        other: Self,
        self_decimals: u32,
        _other_decimals: u32,
    ) -> Result<(Self, u32), DecimalOperationError> {
        let factor = T::from(10u32.pow(self_decimals));
        let adjusted_value = self
            .checked_mul(&factor)
            .ok_or(DecimalOperationError::Overflow)?;
        match adjusted_value.checked_rem(&other) {
            Some(value) => Ok((value, self_decimals)),
            None => Err(DecimalOperationError::DivisionByZero),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_decimals() -> Result<(), Box<dyn std::error::Error>> {
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

        Ok(())
    }

    #[test]
    fn test_sub_decimals() -> Result<(), Box<dyn std::error::Error>> {
        let a: u64 = 3_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.sub_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 1_0000);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.sub_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 123_00);
        assert_eq!(decimals, 2);

        Ok(())
    }

    #[test]
    fn test_mul_decimals() -> Result<(), Box<dyn std::error::Error>> {
        let a: u64 = 3_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.multiply_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 6_000000);
        assert_eq!(decimals, 6);

        let a: u32 = 12345;
        let a_decimals = 2;
        let b: u32 = 45;
        let b_decimals = 2;

        let (result, decimals) = a.multiply_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 555525);
        assert_eq!(decimals, 4);

        Ok(())
    }

    #[test]
    fn test_div_decimals() -> Result<(), Box<dyn std::error::Error>> {
        let a: u64 = 6_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.divide_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 3_0000);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.divide_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 27433);
        assert_eq!(decimals, 2);

        Ok(())
    }

    #[test]
    fn test_rem_decimals() -> Result<(), Box<dyn std::error::Error>> {
        let a: u64 = 6_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.rem_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 0);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.rem_decimals_checked(b, a_decimals, b_decimals)?;
        assert_eq!(result, 15);
        assert_eq!(decimals, 2);

        Ok(())
    }
}
