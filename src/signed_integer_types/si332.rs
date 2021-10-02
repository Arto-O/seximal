use super::{Si12, Si144, Si24, Si52, Sisize};
use crate::{Su12, Su144, Su24, Su332, Su52, Susize};
use num::pow::checked_pow;
use std::{fmt, ops::*};

/// `Si332` is the seximal equivalent of `i128`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Si332 {
    value: i128,
}

impl Si332 {
    /// Returns a new instance of `Si332` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Si332;
    ///
    /// let num = Si332::new(13);
    ///
    /// assert_eq!("21", num.to_string());
    /// ```
    pub fn new(value: i128) -> Si332 {
        Self { value }
    }

    /// Returns a result containing a new instance of `Si332` using a string representation of the value in seximal form.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Si332;
    ///
    /// let num = Si332::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input string contains anything besides digits 1 - 5 and `-` - or if `-` is somewhere other than the beginning.
    ///
    /// Returs an `Err` if the value represented by the input string overflows the underlying number type.
    pub fn from(input: &str) -> Result<Si332, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        match checked_pow(6, input.len() - 1 - first_pos) {
            Some(_) => (),
            None => return Err(String::from("overflow")),
        }

        let v: Vec<char> = input.chars().collect();

        let mut value = 0;
        let mut i = v.len();
        let mut multiplier = 1;
        while i > first_pos {
            let c = v[i - 1];

            if c > '5' || c < '0' {
                return Err(String::from("Input must be a seximal integer."));
            }

            value += (c as i128 - '0' as i128) * multiplier;
            i -= 1;
            if i > first_pos {
                multiplier *= 6
            }
        }
        if first_pos == 1 {
            value *= -1;
        }

        Ok(Self { value })
    }

    /// Returns the value of the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Si332;
    ///
    /// let num = Si332::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// ```
    /// use seximal::Si332;
    ///
    /// let num = Si332::new(-36);
    ///
    /// assert_eq!(-36, num.value());
    /// ```
    pub fn value(&self) -> i128 {
        self.value
    }

    /// Returns an instance of `Sisize` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Sisize,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_sisize();
    ///
    /// assert_eq!(a.value() as isize, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `i128` value overflows when converting to `isize`.
    pub fn as_sisize(&self) -> Sisize {
        Sisize::new(self.value as isize)
    }

    /// Returns an instance of `Si144` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Si144,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_si144();
    ///
    /// assert_eq!(a.value() as i64, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `i128` value overflows when converting to `i64`.
    pub fn as_si144(&self) -> Si144 {
        Si144::new(self.value as i64)
    }

    /// Returns an instance of `Si52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Si52,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_si52();
    ///
    /// assert_eq!(a.value() as i32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `i128` value overflows when converting to `i32`.
    pub fn as_si52(&self) -> Si52 {
        Si52::new(self.value as i32)
    }

    /// Returns an instance of `Si24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Si24,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_si24();
    ///
    /// assert_eq!(a.value() as i16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `i128` value overflows when converting to `i16`.
    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    /// Returns an instance of `Si12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Si12,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_si12();
    ///
    /// assert_eq!(a.value() as i8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `i128` value overflows when converting to `i8`.
    pub fn as_si12(&self) -> Si12 {
        Si12::new(self.value as i8)
    }

    // Conversion to unsigned integer types

    /// Returns an instance of `Susize` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Susize,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_susize();
    ///
    /// assert_eq!(a.value() as usize, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `i128` value overflows when converting to `usize`.
    pub fn as_susize(&self) -> Susize {
        Susize::new(self.value as usize)
    }

    /// Returns an instance of `Su332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Su332,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_su332();
    ///
    /// assert_eq!(a.value() as u128, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    pub fn as_su332(&self) -> Su332 {
        Su332::new(self.value as u128)
    }

    /// Returns an instance of `Su144` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Su144,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_su144();
    ///
    /// assert_eq!(a.value() as u64, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `i128` value overflows when converting to `u64`.
    pub fn as_su144(&self) -> Su144 {
        Su144::new(self.value as u64)
    }

    /// Returns an instance of `Su52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Su52,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_su52();
    ///
    /// assert_eq!(a.value() as u32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `i128` value overflows when converting to `u32`.
    pub fn as_su52(&self) -> Su52 {
        Su52::new(self.value as u32)
    }

    /// Returns an instance of `Su24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Su24,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_su24();
    ///
    /// assert_eq!(a.value() as u16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `i128` value overflows when converting to `u16`.
    pub fn as_su24(&self) -> Su24 {
        Su24::new(self.value as u16)
    }

    /// Returns an instance of `Su12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si332,
    ///     Su12,
    /// };
    ///
    /// let a = Si332::new(21);
    /// let b = a.as_su12();
    ///
    /// assert_eq!(a.value() as u8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `i128` value overflows when converting to `u8`.
    pub fn as_su12(&self) -> Su12 {
        Su12::new(self.value as u8)
    }
}

impl fmt::Display for Si332 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dec_value = self.value;
        let mut s;
        let index;

        if dec_value < 0 {
            s = String::from('-');
            index = 1;
            dec_value *= -1;
        } else {
            s = String::new();
            index = 0;
        }

        while dec_value >= 6 {
            s.insert(index, ((dec_value % 6) as u8 + '0' as u8) as char);
            dec_value /= 6;
        }
        s.insert(index, (dec_value as u8 + '0' as u8) as char);

        write!(f, "{}", s)
    }
}

// ----- Native Arithmetic Operators -----

impl Add for Si332 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Si332 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Si332 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Si332 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Si332 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Si332 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Si332 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Si332 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Si332 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Si332 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Si332 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Si332 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Si332 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Si332 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Si332 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<i128> for Si332 {
    type Output = Self;

    fn add(self, rhs: i128) -> Self {
        Si332 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<i128> for Si332 {
    fn add_assign(&mut self, rhs: i128) {
        self.value += rhs;
    }
}

impl Sub<i128> for Si332 {
    type Output = Self;

    fn sub(self, rhs: i128) -> Self {
        Si332 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<i128> for Si332 {
    fn sub_assign(&mut self, rhs: i128) {
        self.value -= rhs;
    }
}

impl Mul<i128> for Si332 {
    type Output = Self;

    fn mul(self, rhs: i128) -> Self {
        Si332 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<i128> for Si332 {
    fn mul_assign(&mut self, rhs: i128) {
        self.value *= rhs;
    }
}

impl Div<i128> for Si332 {
    type Output = Self;

    fn div(self, rhs: i128) -> Self {
        Si332 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<i128> for Si332 {
    fn div_assign(&mut self, rhs: i128) {
        self.value /= rhs;
    }
}

impl Rem<i128> for Si332 {
    type Output = Self;

    fn rem(self, rhs: i128) -> Self {
        Si332 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<i128> for Si332 {
    fn rem_assign(&mut self, rhs: i128) {
        self.value %= rhs;
    }
}

#[cfg(test)]
mod si332_tests {
    use super::Si332;
    use crate::util::ordering_to_string;
    use std::cmp::Ordering::*;

    #[test]
    fn si332_new() {
        let num = Si332::new(13);
        assert_eq!(
            num.to_string(),
            "21",
            "to_string failed, expected 21, got {}",
            num.to_string()
        );

        let num = Si332::new(0);
        assert_eq!(
            num.to_string(),
            "0",
            "to_string failed, expected 0, got {}",
            num.to_string()
        );

        let num = Si332::new(-36);
        assert_eq!(
            num.to_string(),
            "-100",
            "to_string failed, expected -100, got {}",
            num.to_string()
        );
    }

    #[test]
    fn si332_from() {
        let num = Si332::from("21").unwrap();
        assert_eq!(
            num.value(),
            13,
            "from failed, expected 13, got {}",
            num.to_string()
        );

        let num = Si332::from("0").unwrap();
        assert_eq!(
            num.value(),
            0,
            "from failed, expected 0, got {}",
            num.to_string()
        );

        let num = Si332::from("-100").unwrap();
        assert_eq!(
            num.value(),
            -36,
            "from failed, expected -36, got {}",
            num.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn si332_from_panics() {
        let _num = Si332::from("9").unwrap();
    }

    #[test]
    fn si332_native_arithmetic() {
        let mut num = Si332::new(13);
        num += Si332::new(2);
        assert_eq!(
            num.to_string(),
            "23",
            "21 + 2 failed, expected 23, got {}",
            num.to_string()
        );

        num -= Si332::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "23 - 2 failed, expected 21, got {}",
            num.to_string()
        );

        num *= Si332::new(2);
        assert_eq!(
            num.to_string(),
            "42",
            "21 * 2 failed, expected 42, got {}",
            num.to_string()
        );

        num /= Si332::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "42 / 2 failed, expected 21, got {}",
            num.to_string()
        );

        num %= Si332::new(3);
        assert_eq!(
            num.to_string(),
            "1",
            "21 % 3 failed, expected 1, got {}",
            num.to_string()
        );
    }

    #[test]
    fn si332_decimal_arithmetic() {
        let mut num = Si332::new(13);
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
    fn si332_cmp() {
        let a = Si332::new(3);
        let b = Si332::new(5);
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

        let c = Si332::new(3);
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
