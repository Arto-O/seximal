use super::{Su12, Su144, Su24, Su332, Su52};
use crate::{Si12, Si144, Si24, Si332, Si52, Sisize};
use num::pow::checked_pow;
use std::{fmt, ops::*};

/// `Susize` is the seximal equivalent of `usize`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Susize {
    value: usize,
}

impl Susize {
    /// Returns a new instance of `Susize` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Susize;
    ///
    /// let num = Susize::new(13);
    ///
    /// assert_eq!("21", num.to_string());
    /// ```
    pub fn new(value: usize) -> Susize {
        Self { value }
    }

    /// Returns a result containing a new instance of `Susize` using a string representation of the value in seximal form.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Susize;
    ///
    /// let num = Susize::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input string contains anything besides digits 1 - 5.
    ///
    /// Returs an `Err` if the value represented by the input string overflows the underlying number type.
    pub fn from(input: &str) -> Result<Susize, String> {
        if checked_pow(6, input.len() - 1 - 0).expect("overflow") > usize::MAX as u128 {
            return Err(String::from("overflow"));
        }

        let v: Vec<char> = input.chars().collect();

        let mut value = 0;
        let mut i = v.len();
        let mut multiplier = 1;
        while i > 0 {
            let c = v[i - 1];

            if c > '5' || c < '0' {
                return Err(String::from("Input must be a seximal whole number."));
            }

            value += (c as usize - '0' as usize) * multiplier;
            i -= 1;
            if i > 0 {
                multiplier *= 6
            }
        }

        Ok(Self { value })
    }

    /// Returns the value of the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Susize;
    ///
    /// let num = Susize::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    pub fn value(&self) -> usize {
        self.value
    }

    /// Returns an instance of `Su332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Su332,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_su332();
    ///
    /// assert_eq!(a.value() as u128, b.value());
    /// ```
    pub fn as_su332(&self) -> Su332 {
        Su332::new(self.value as u128)
    }

    /// Returns an instance of `Su144` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Su144,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_su144();
    ///
    /// assert_eq!(a.value() as u64, b.value());
    /// ```
    pub fn as_su144(&self) -> Su144 {
        Su144::new(self.value as u64)
    }

    /// Returns an instance of `Su52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Su52,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_su52();
    ///
    /// assert_eq!(a.value() as u32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `u32`. Applicable only on 64-bit systems.
    pub fn as_su52(&self) -> Su52 {
        Su52::new(self.value as u32)
    }

    /// Returns an instance of `Su24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Su24,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_su24();
    ///
    /// assert_eq!(a.value() as u16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `u16`.
    pub fn as_su24(&self) -> Su24 {
        Su24::new(self.value as u16)
    }

    /// Returns an instance of `Su12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Su12,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_su12();
    ///
    /// assert_eq!(a.value() as u8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `u8`.
    pub fn as_su12(&self) -> Su12 {
        Su12::new(self.value as u8)
    }

    // Conversion to signed integer types

    /// Returns an instance of `Sisize` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Sisize,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_sisize();
    ///
    /// assert_eq!(a.value() as isize, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `isize`.
    pub fn as_sisize(&self) -> Sisize {
        Sisize::new(self.value as isize)
    }

    /// Returns an instance of `Si332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Si332,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_si332();
    ///
    /// assert_eq!(a.value() as i128, b.value());
    /// ```
    pub fn as_si332(&self) -> Si332 {
        Si332::new(self.value as i128)
    }

    /// Returns an instance of `Si144` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Si144,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_si144();
    ///
    /// assert_eq!(a.value() as i64, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `i64`. Applicable only on 64-bit systems.
    pub fn as_si144(&self) -> Si144 {
        Si144::new(self.value as i64)
    }

    /// Returns an instance of `Si52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Si52,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_si52();
    ///
    /// assert_eq!(a.value() as i32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `i32`.
    pub fn as_si52(&self) -> Si52 {
        Si52::new(self.value as i32)
    }

    /// Returns an instance of `Si24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Si24,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_si24();
    ///
    /// assert_eq!(a.value() as i16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `i16`.
    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    /// Returns an instance of `Si12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Susize,
    ///     Si12,
    /// };
    ///
    /// let a = Susize::new(21);
    /// let b = a.as_si12();
    ///
    /// assert_eq!(a.value() as i8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `usize` value overflows when converting to `i8`.
    pub fn as_si12(&self) -> Si12 {
        Si12::new(self.value as i8)
    }
}

impl fmt::Display for Susize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dec_value = self.value;
        let mut s = String::new();

        while dec_value >= 6 {
            s.insert(0, ((dec_value % 6) as u8 + '0' as u8) as char);
            dec_value /= 6;
        }
        s.insert(0, (dec_value as u8 + '0' as u8) as char);

        write!(f, "{}", s)
    }
}

// ----- Native Arithmetic Operators -----

impl Add for Susize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Susize {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Susize {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Susize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Susize {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Susize {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Susize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Susize {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Susize {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Susize {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Susize {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Susize {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Susize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Susize {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Susize {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<usize> for Susize {
    type Output = Self;

    fn add(self, rhs: usize) -> Self {
        Susize {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<usize> for Susize {
    fn add_assign(&mut self, rhs: usize) {
        self.value += rhs;
    }
}

impl Sub<usize> for Susize {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self {
        Susize {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<usize> for Susize {
    fn sub_assign(&mut self, rhs: usize) {
        self.value -= rhs;
    }
}

impl Mul<usize> for Susize {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Susize {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<usize> for Susize {
    fn mul_assign(&mut self, rhs: usize) {
        self.value *= rhs;
    }
}

impl Div<usize> for Susize {
    type Output = Self;

    fn div(self, rhs: usize) -> Self {
        Susize {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<usize> for Susize {
    fn div_assign(&mut self, rhs: usize) {
        self.value /= rhs;
    }
}

impl Rem<usize> for Susize {
    type Output = Self;

    fn rem(self, rhs: usize) -> Self {
        Susize {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<usize> for Susize {
    fn rem_assign(&mut self, rhs: usize) {
        self.value %= rhs;
    }
}

#[cfg(test)]
mod susize_tests {
    use super::Susize;
    use crate::util::ordering_to_string;
    use std::cmp::Ordering::*;

    #[test]
    fn susize_new() {
        let num = Susize::new(13);
        assert_eq!(
            num.to_string(),
            "21",
            "to_string failed, expected 21, got {}",
            num.to_string()
        );

        let num = Susize::new(0);
        assert_eq!(
            num.to_string(),
            "0",
            "to_string failed, expected 0, got {}",
            num.to_string()
        );
    }

    #[test]
    fn susize_from() {
        let num = Susize::from("21").unwrap();
        assert_eq!(
            num.value(),
            13,
            "from failed, expected 13, got {}",
            num.to_string()
        );

        let num = Susize::from("0").unwrap();
        assert_eq!(
            num.value(),
            0,
            "from failed, expected 0, got {}",
            num.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn susize_from_panics() {
        let _num = Susize::from("9").unwrap();
    }

    #[test]
    fn susize_native_arithmetic() {
        let mut num = Susize::new(13);
        num += Susize::new(2);
        assert_eq!(
            num.to_string(),
            "23",
            "21 + 2 failed, expected 23, got {}",
            num.to_string()
        );

        num -= Susize::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "23 - 2 failed, expected 21, got {}",
            num.to_string()
        );

        num *= Susize::new(2);
        assert_eq!(
            num.to_string(),
            "42",
            "21 * 2 failed, expected 42, got {}",
            num.to_string()
        );

        num /= Susize::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "42 / 2 failed, expected 21, got {}",
            num.to_string()
        );

        num %= Susize::new(3);
        assert_eq!(
            num.to_string(),
            "1",
            "21 % 3 failed, expected 1, got {}",
            num.to_string()
        );
    }

    #[test]
    fn susize_decimal_arithmetic() {
        let mut num = Susize::new(13);
        num += 2;
        assert_eq!(
            num.to_string(),
            "23",
            "21 + 2 failed, expected 23, got {}",
            num.to_string()
        );

        num -= 2;
        assert_eq!(
            num.to_string(),
            "21",
            "23 - 2 failed, expected 21, got {}",
            num.to_string()
        );

        num *= 2;
        assert_eq!(
            num.to_string(),
            "42",
            "21 * 2 failed, expected 42, got {}",
            num.to_string()
        );

        num /= 2;
        assert_eq!(
            num.to_string(),
            "21",
            "42 / 2 failed, expected 21, got {}",
            num.to_string()
        );

        num %= 3;
        assert_eq!(
            num.to_string(),
            "1",
            "21 % 3 failed, expected 1, got {}",
            num.to_string()
        );
    }

    #[test]
    fn susize_cmp() {
        let a = Susize::new(3);
        let b = Susize::new(5);
        let mut result;

        result = a.cmp(&b);
        assert_eq!(
            result,
            Less,
            "{}.cmp(&{}) failed, expected Less, got {}",
            a,
            b,
            ordering_to_string(result)
        );

        result = b.cmp(&a);
        assert_eq!(
            result,
            Greater,
            "{}.cmp(&{}) failed, expected Greater, got {}",
            b,
            a,
            ordering_to_string(result)
        );

        let c = Susize::new(3);
        result = a.cmp(&c);
        assert_eq!(
            result,
            Equal,
            "{}.cmp({}) failed, expected Equal, got {}",
            a,
            c,
            ordering_to_string(result)
        );
    }
}
