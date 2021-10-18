use super::{Si12, Si144, Si24, Si332, Si52};
use crate::{Su12, Su144, Su24, Su332, Su52, Susize};
use num::pow::checked_pow;
use std::{fmt, ops::*};

/// `Sisize` is the seximal equivalent of `isize`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sisize {
    value: isize,
}

impl Sisize {
    /// Returns a new instance of `Sisize` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Sisize;
    ///
    /// let num = Sisize::new(13);
    ///
    /// assert_eq!("21", num.to_string());
    /// ```
    pub fn new(value: isize) -> Sisize {
        Self { value }
    }

    /// Returns a result containing a new instance of `Sisize` using a string representation of the value in seximal form.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Sisize;
    ///
    /// let num = Sisize::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input string contains anything besides digits 1 - 5 and `-` - or if `-` is somewhere other than the beginning.
    ///
    /// Returs an `Err` if the value represented by the input string overflows the underlying number type.
    pub fn from(input: &str) -> Result<Sisize, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        if checked_pow(6, input.len() - 1 - first_pos).expect("overflow") > isize::MAX as i128 {
            return Err(String::from("overflow"));
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

            value += (c as isize - '0' as isize) * multiplier;
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
    /// use seximal::Sisize;
    ///
    /// let num = Sisize::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// ```
    /// use seximal::Sisize;
    ///
    /// let num = Sisize::new(-36);
    ///
    /// assert_eq!(-36, num.value());
    /// ```
    pub fn value(&self) -> isize {
        self.value
    }

    /// Returns an instance of `Si332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Si332,
    /// };
    ///
    /// let a = Sisize::new(21);
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
    ///     Sisize,
    ///     Si144,
    /// };
    ///
    /// let a = Sisize::new(21);
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
    ///     Sisize,
    ///     Si52,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_si52();
    ///
    /// assert_eq!(a.value() as i32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `isize` value overflows when converting to `i32`. Applicable only on 64-bit systems.
    pub fn as_si52(&self) -> Si52 {
        Si52::new(self.value as i32)
    }

    /// Returns an instance of `Si24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Si24,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_si24();
    ///
    /// assert_eq!(a.value() as i16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `isize` value overflows when converting to `i16`.
    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    /// Returns an instance of `Si12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Si12,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_si12();
    ///
    /// assert_eq!(a.value() as i8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `isize` value overflows when converting to `i8`.
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
    ///     Sisize,
    ///     Susize,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_susize();
    ///
    /// assert_eq!(a.value() as usize, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `isize` value overflows when converting to `usize`.
    pub fn as_susize(&self) -> Susize {
        Susize::new(self.value as usize)
    }

    /// Returns an instance of `Su332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Su332,
    /// };
    ///
    /// let a = Sisize::new(21);
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
    ///     Sisize,
    ///     Su144,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_su144();
    ///
    /// assert_eq!(a.value() as u64, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    pub fn as_su144(&self) -> Su144 {
        Su144::new(self.value as u64)
    }

    /// Returns an instance of `Su52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Su52,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_su52();
    ///
    /// assert_eq!(a.value() as u32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `isize` value overflows when converting to `u32`. Applicable only on 64-bit systems.
    pub fn as_su52(&self) -> Su52 {
        Su52::new(self.value as u32)
    }

    /// Returns an instance of `Su24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Su24,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_su24();
    ///
    /// assert_eq!(a.value() as u16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `isize` value overflows when converting to `u16`.
    pub fn as_su24(&self) -> Su24 {
        Su24::new(self.value as u16)
    }

    /// Returns an instance of `Su12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sisize,
    ///     Su12,
    /// };
    ///
    /// let a = Sisize::new(21);
    /// let b = a.as_su12();
    ///
    /// assert_eq!(a.value() as u8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    ///
    /// Panics if the underlying `isize` value overflows when converting to `u8`.
    pub fn as_su12(&self) -> Su12 {
        Su12::new(self.value as u8)
    }
}

impl fmt::Display for Sisize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dec_value = self.value;
        let mut s;
        let mut index = 0;

        if dec_value < 0 {
            s = String::from('-');
            index = 1;
            dec_value *= -1;
        } else if dec_value > 0 {
            s = String::new();
        } else {
            s = String::from('0');
        }

        while dec_value > 0 {
            s.insert(index, ((dec_value % 6) as u8 + '0' as u8) as char);
            dec_value /= 6;
        }

        write!(f, "{}", s)
    }
}

// ----- Native Arithmetic Operators -----

impl Add for Sisize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Sisize {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Sisize {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Sisize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Sisize {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Sisize {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Sisize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Sisize {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Sisize {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Sisize {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Sisize {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Sisize {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Sisize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Sisize {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Sisize {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<isize> for Sisize {
    type Output = Self;

    fn add(self, rhs: isize) -> Self {
        Sisize {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<isize> for Sisize {
    fn add_assign(&mut self, rhs: isize) {
        self.value += rhs;
    }
}

impl Sub<isize> for Sisize {
    type Output = Self;

    fn sub(self, rhs: isize) -> Self {
        Sisize {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<isize> for Sisize {
    fn sub_assign(&mut self, rhs: isize) {
        self.value -= rhs;
    }
}

impl Mul<isize> for Sisize {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self {
        Sisize {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<isize> for Sisize {
    fn mul_assign(&mut self, rhs: isize) {
        self.value *= rhs;
    }
}

impl Div<isize> for Sisize {
    type Output = Self;

    fn div(self, rhs: isize) -> Self {
        Sisize {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<isize> for Sisize {
    fn div_assign(&mut self, rhs: isize) {
        self.value /= rhs;
    }
}

impl Rem<isize> for Sisize {
    type Output = Self;

    fn rem(self, rhs: isize) -> Self {
        Sisize {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<isize> for Sisize {
    fn rem_assign(&mut self, rhs: isize) {
        self.value %= rhs;
    }
}

#[cfg(test)]
mod sisize_tests {
    use super::Sisize;
    use crate::util::ordering_to_string;
    use std::cmp::Ordering::*;

    #[test]
    fn sisize_new() {
        let num = Sisize::new(13);
        assert_eq!(
            num.to_string(),
            "21",
            "to_string failed, expected 21, got {}",
            num.to_string()
        );

        let num = Sisize::new(0);
        assert_eq!(
            num.to_string(),
            "0",
            "to_string failed, expected 0, got {}",
            num.to_string()
        );

        let num = Sisize::new(-36);
        assert_eq!(
            num.to_string(),
            "-100",
            "to_string failed, expected -100, got {}",
            num.to_string()
        );
    }

    #[test]
    fn sisize_from() {
        let num = Sisize::from("21").unwrap();
        assert_eq!(
            num.value(),
            13,
            "from failed, expected 13, got {}",
            num.to_string()
        );

        let num = Sisize::from("0").unwrap();
        assert_eq!(
            num.value(),
            0,
            "from failed, expected 0, got {}",
            num.to_string()
        );

        let num = Sisize::from("-100").unwrap();
        assert_eq!(
            num.value(),
            -36,
            "from failed, expected -36, got {}",
            num.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn sisize_from_panics() {
        let _num = Sisize::from("9").unwrap();
    }

    #[test]
    fn sisize_native_arithmetic() {
        let mut num = Sisize::new(13);
        num += Sisize::new(2);
        assert_eq!(
            num.to_string(),
            "23",
            "21 + 2 failed, expected 23, got {}",
            num.to_string()
        );

        num -= Sisize::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "23 - 2 failed, expected 21, got {}",
            num.to_string()
        );

        num *= Sisize::new(2);
        assert_eq!(
            num.to_string(),
            "42",
            "21 * 2 failed, expected 42, got {}",
            num.to_string()
        );

        num /= Sisize::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "42 / 2 failed, expected 21, got {}",
            num.to_string()
        );

        num %= Sisize::new(3);
        assert_eq!(
            num.to_string(),
            "1",
            "21 % 3 failed, expected 1, got {}",
            num.to_string()
        );
    }

    #[test]
    fn sisize_decimal_arithmetic() {
        let mut num = Sisize::new(13);
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
    fn sisize_cmp() {
        let a = Sisize::new(3);
        let b = Sisize::new(5);
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

        let c = Sisize::new(3);
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
