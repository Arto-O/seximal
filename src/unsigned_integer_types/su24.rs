use super::{Su12, Su144, Su332, Su52, Susize};
use crate::{Si12, Si144, Si24, Si332, Si52, Sisize};
use num::pow::checked_pow;
use std::{fmt, ops::*};

/// `Su24` is the seximal equivalent of `u16`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Su24 {
    value: u16,
}

impl Su24 {
    /// Returns a new instance of `Su24` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Su24;
    ///
    /// let num = Su24::new(13);
    ///
    /// assert_eq!("21", num.to_string());
    /// ```
    pub fn new(value: u16) -> Su24 {
        Self { value }
    }

    /// Returns a result containing a new instance of `Su24` using a string representation of the value in seximal form.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Su24;
    ///
    /// let num = Su24::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input string contains anything besides digits 1 - 5.
    ///
    /// Returs an `Err` if the value represented by the input string overflows the underlying number type.
    pub fn from(input: &str) -> Result<Su24, String> {
        match checked_pow(6, input.len() - 1) {
            Some(_) => (),
            None => return Err(String::from("overflow")),
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

            value += (c as u16 - '0' as u16) * multiplier;
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
    /// use seximal::Su24;
    ///
    /// let num = Su24::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    pub fn value(&self) -> u16 {
        self.value
    }

    /// Returns an instance of `Susize` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Susize,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_susize();
    ///
    /// assert_eq!(a.value() as usize, b.value());
    /// ```
    pub fn as_susize(&self) -> Susize {
        Susize::new(self.value as usize)
    }

    /// Returns an instance of `Su332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Su332,
    /// };
    ///
    /// let a = Su24::new(21);
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
    ///     Su24,
    ///     Su144,
    /// };
    ///
    /// let a = Su24::new(21);
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
    ///     Su24,
    ///     Su52,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_su52();
    ///
    /// assert_eq!(a.value() as u32, b.value());
    /// ```
    pub fn as_su52(&self) -> Su52 {
        Su52::new(self.value as u32)
    }

    /// Returns an instance of `Su12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Su12,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_su12();
    ///
    /// assert_eq!(a.value() as u8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `u16` value overflows when converting to `u8`.
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
    ///     Su24,
    ///     Sisize,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_sisize();
    ///
    /// assert_eq!(a.value() as isize, b.value());
    /// ```
    pub fn as_sisize(&self) -> Sisize {
        Sisize::new(self.value as isize)
    }

    /// Returns an instance of `Si332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Si332,
    /// };
    ///
    /// let a = Su24::new(21);
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
    ///     Su24,
    ///     Si144,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_si144();
    ///
    /// assert_eq!(a.value() as i64, b.value());
    /// ```
    pub fn as_si144(&self) -> Si144 {
        Si144::new(self.value as i64)
    }

    /// Returns an instance of `Si52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Si52,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_si52();
    ///
    /// assert_eq!(a.value() as i32, b.value());
    /// ```
    pub fn as_si52(&self) -> Si52 {
        Si52::new(self.value as i32)
    }

    /// Returns an instance of `Si24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Si24,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_si24();
    ///
    /// assert_eq!(a.value() as i16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `u16` value overflows when converting to `i16`.
    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    /// Returns an instance of `Si12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Su24,
    ///     Si12,
    /// };
    ///
    /// let a = Su24::new(21);
    /// let b = a.as_si12();
    ///
    /// assert_eq!(a.value() as i8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `u16` value overflows when converting to `i8`.
    pub fn as_si12(&self) -> Si12 {
        Si12::new(self.value as i8)
    }
}

impl fmt::Display for Su24 {
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

impl Add for Su24 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Su24 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Su24 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Su24 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Su24 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Su24 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Su24 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Su24 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Su24 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Su24 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Su24 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Su24 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Su24 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Su24 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Su24 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<u16> for Su24 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self {
        Su24 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<u16> for Su24 {
    fn add_assign(&mut self, rhs: u16) {
        self.value += rhs;
    }
}

impl Sub<u16> for Su24 {
    type Output = Self;

    fn sub(self, rhs: u16) -> Self {
        Su24 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<u16> for Su24 {
    fn sub_assign(&mut self, rhs: u16) {
        self.value -= rhs;
    }
}

impl Mul<u16> for Su24 {
    type Output = Self;

    fn mul(self, rhs: u16) -> Self {
        Su24 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<u16> for Su24 {
    fn mul_assign(&mut self, rhs: u16) {
        self.value *= rhs;
    }
}

impl Div<u16> for Su24 {
    type Output = Self;

    fn div(self, rhs: u16) -> Self {
        Su24 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<u16> for Su24 {
    fn div_assign(&mut self, rhs: u16) {
        self.value /= rhs;
    }
}

impl Rem<u16> for Su24 {
    type Output = Self;

    fn rem(self, rhs: u16) -> Self {
        Su24 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<u16> for Su24 {
    fn rem_assign(&mut self, rhs: u16) {
        self.value %= rhs;
    }
}

#[cfg(test)]
mod su24_tests {
    use super::Su24;
    use crate::util::ordering_to_string;
    use std::cmp::Ordering::*;

    #[test]
    fn su24_new() {
        let num = Su24::new(13);
        assert_eq!(
            num.to_string(),
            "21",
            "to_string failed, expected 21, got {}",
            num.to_string()
        );

        let num = Su24::new(0);
        assert_eq!(
            num.to_string(),
            "0",
            "to_string failed, expected 0, got {}",
            num.to_string()
        );
    }

    #[test]
    fn su24_from() {
        let num = Su24::from("21").unwrap();
        assert_eq!(
            num.value(),
            13,
            "from failed, expected 13, got {}",
            num.to_string()
        );

        let num = Su24::from("0").unwrap();
        assert_eq!(
            num.value(),
            0,
            "from failed, expected 0, got {}",
            num.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn su24_from_panics() {
        let _num = Su24::from("9").unwrap();
    }

    #[test]
    fn su24_native_arithmetic() {
        let mut num = Su24::new(13);
        num += Su24::new(2);
        assert_eq!(
            num.to_string(),
            "23",
            "21 + 2 failed, expected 23, got {}",
            num.to_string()
        );

        num -= Su24::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "23 - 2 failed, expected 21, got {}",
            num.to_string()
        );

        num *= Su24::new(2);
        assert_eq!(
            num.to_string(),
            "42",
            "21 * 2 failed, expected 42, got {}",
            num.to_string()
        );

        num /= Su24::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "42 / 2 failed, expected 21, got {}",
            num.to_string()
        );

        num %= Su24::new(3);
        assert_eq!(
            num.to_string(),
            "1",
            "21 % 3 failed, expected 1, got {}",
            num.to_string()
        );
    }

    #[test]
    fn su24_decimal_arithmetic() {
        let mut num = Su24::new(13);
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
    fn su24_cmp() {
        let a = Su24::new(3);
        let b = Su24::new(5);
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

        let c = Su24::new(3);
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
