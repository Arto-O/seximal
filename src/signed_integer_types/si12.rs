use super::{Si144, Si24, Si332, Si52, Sisize};
use crate::{Su12, Su144, Su24, Su332, Su52, Susize};
use num::pow::checked_pow;
use std::{fmt, ops::*};

/// `Si12` is the seximal equivalent of `i8`.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Si12 {
    value: i8,
}

impl Si12 {
    /// Returns a new instance of `Si12` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Si12;
    ///
    /// let num = Si12::new(13);
    ///
    /// assert_eq!("21", num.to_string());
    /// ```
    pub fn new(value: i8) -> Si12 {
        Self { value }
    }

    /// Returns a result containing a new instance of `Si12` using a string representation of the value in seximal form.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Si12;
    ///
    /// let num = Si12::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input string contains anything besides digits 1 - 5 and `-` - or if `-` is somewhere other than the beginning.
    ///
    /// Returs an `Err` if the value represented by the input string overflows the underlying number type.
    pub fn from(input: &str) -> Result<Si12, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        let pow_result = match checked_pow(6, input.len() - 1 - first_pos) {
            Some(val) => val,
            None => return Err(String::from("overflow")),
        };
        if pow_result > i8::MAX as i16 {
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

            value += (c as i8 - '0' as i8) * multiplier;
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
    /// use seximal::Si12;
    ///
    /// let num = Si12::from("21").unwrap();
    ///
    /// assert_eq!(13, num.value());
    /// ```
    ///
    /// ```
    /// use seximal::Si12;
    ///
    /// let num = Si12::new(-36);
    ///
    /// assert_eq!(-36, num.value());
    /// ```
    pub fn value(&self) -> i8 {
        self.value
    }

    // Conversion to signed integer types

    /// Returns an instance of `Sisize` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si12,
    ///     Sisize,
    /// };
    ///
    /// let a = Si12::new(21);
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
    ///     Si12,
    ///     Si332,
    /// };
    ///
    /// let a = Si12::new(21);
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
    ///     Si12,
    ///     Si144,
    /// };
    ///
    /// let a = Si12::new(21);
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
    ///     Si12,
    ///     Si52,
    /// };
    ///
    /// let a = Si12::new(21);
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
    ///     Si12,
    ///     Si24,
    /// };
    ///
    /// let a = Si12::new(21);
    /// let b = a.as_si24();
    ///
    /// assert_eq!(a.value() as i16, b.value());
    /// ```
    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    // Conversion to unsigned integer types

    /// Returns an instance of `Susize` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si12,
    ///     Susize,
    /// };
    ///
    /// let a = Si12::new(21);
    /// let b = a.as_susize();
    ///
    /// assert_eq!(a.value() as usize, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    pub fn as_susize(&self) -> Susize {
        Susize::new(self.value as usize)
    }

    /// Returns an instance of `Su332` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si12,
    ///     Su332,
    /// };
    ///
    /// let a = Si12::new(21);
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
    ///     Si12,
    ///     Su144,
    /// };
    ///
    /// let a = Si12::new(21);
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
    ///     Si12,
    ///     Su52,
    /// };
    ///
    /// let a = Si12::new(21);
    /// let b = a.as_su52();
    ///
    /// assert_eq!(a.value() as u32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    pub fn as_su52(&self) -> Su52 {
        Su52::new(self.value as u32)
    }

    /// Returns an instance of `Su24` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si12,
    ///     Su24,
    /// };
    ///
    /// let a = Si12::new(21);
    /// let b = a.as_su24();
    ///
    /// assert_eq!(a.value() as u16, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    pub fn as_su24(&self) -> Su24 {
        Su24::new(self.value as u16)
    }

    /// Returns an instance of `Su12` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Si12,
    ///     Su12,
    /// };
    ///
    /// let a = Si12::new(21);
    /// let b = a.as_su12();
    ///
    /// assert_eq!(a.value() as u8, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the starting value is negative.
    pub fn as_su12(&self) -> Su12 {
        Su12::new(self.value as u8)
    }
}

impl fmt::Display for Si12 {
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

impl Add for Si12 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Si12 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Si12 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Si12 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Si12 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Si12 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Si12 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Si12 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Si12 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Si12 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Si12 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Si12 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Si12 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Si12 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Si12 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<i8> for Si12 {
    type Output = Self;

    fn add(self, rhs: i8) -> Self {
        Si12 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<i8> for Si12 {
    fn add_assign(&mut self, rhs: i8) {
        self.value += rhs;
    }
}

impl Sub<i8> for Si12 {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self {
        Si12 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<i8> for Si12 {
    fn sub_assign(&mut self, rhs: i8) {
        self.value -= rhs;
    }
}

impl Mul<i8> for Si12 {
    type Output = Self;

    fn mul(self, rhs: i8) -> Self {
        Si12 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<i8> for Si12 {
    fn mul_assign(&mut self, rhs: i8) {
        self.value *= rhs;
    }
}

impl Div<i8> for Si12 {
    type Output = Self;

    fn div(self, rhs: i8) -> Self {
        Si12 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<i8> for Si12 {
    fn div_assign(&mut self, rhs: i8) {
        self.value /= rhs;
    }
}

impl Rem<i8> for Si12 {
    type Output = Self;

    fn rem(self, rhs: i8) -> Self {
        Si12 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<i8> for Si12 {
    fn rem_assign(&mut self, rhs: i8) {
        self.value %= rhs;
    }
}

#[cfg(test)]
mod si12_tests {
    use super::Si12;
    use crate::util::ordering_to_string;
    use std::cmp::Ordering::*;

    #[test]
    fn si12_new() {
        let num = Si12::new(13);
        assert_eq!(
            num.to_string(),
            "21",
            "to_string failed, expected 21, got {}",
            num.to_string()
        );

        let num = Si12::new(0);
        assert_eq!(
            num.to_string(),
            "0",
            "to_string failed, expected 0, got {}",
            num.to_string()
        );

        let num = Si12::new(-36);
        assert_eq!(
            num.to_string(),
            "-100",
            "to_string failed, expected -100, got {}",
            num.to_string()
        );
    }

    #[test]
    fn si12_from() {
        let num = Si12::from("21").unwrap();
        assert_eq!(
            num.value(),
            13,
            "from failed, expected 13, got {}",
            num.to_string()
        );

        let num = Si12::from("0").unwrap();
        assert_eq!(
            num.value(),
            0,
            "from failed, expected 0, got {}",
            num.to_string()
        );

        let num = Si12::from("-100").unwrap();
        assert_eq!(
            num.value(),
            -36,
            "from failed, expected -36, got {}",
            num.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn si12_from_panics() {
        let _num = Si12::from("9").unwrap();
    }

    #[test]
    fn si12_native_arithmetic() {
        let mut num = Si12::new(13);
        num += Si12::new(2);
        assert_eq!(
            num.to_string(),
            "23",
            "21 + 2 failed, expected 23, got {}",
            num.to_string()
        );

        num -= Si12::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "23 - 2 failed, expected 21, got {}",
            num.to_string()
        );

        num *= Si12::new(2);
        assert_eq!(
            num.to_string(),
            "42",
            "21 * 2 failed, expected 42, got {}",
            num.to_string()
        );

        num /= Si12::new(2);
        assert_eq!(
            num.to_string(),
            "21",
            "42 / 2 failed, expected 21, got {}",
            num.to_string()
        );

        num %= Si12::new(3);
        assert_eq!(
            num.to_string(),
            "1",
            "21 % 3 failed, expected 1, got {}",
            num.to_string()
        );
    }

    #[test]
    fn si12_decimal_arithmetic() {
        let mut num = Si12::new(13);
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
    fn si12_cmp() {
        let a = Si12::new(3);
        let b = Si12::new(5);
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

        let c = Si12::new(3);
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
