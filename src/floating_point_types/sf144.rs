use super::{util::FractDigit, Sf52};
use crate::Su332;
use std::{cmp::Ordering, fmt, ops::*};

/// `Sf144` is the seximal equivalent of `f64`.
#[derive(Copy, Clone)]
pub struct Sf144 {
    value: f64,
}

impl Sf144 {
    /// Returns a new instance of `Sf144` with the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Sf144;
    ///
    /// let num = Sf144::new(2.5);
    ///
    /// assert_eq!("2.3", num.to_string());
    /// ```
    pub fn new(value: f64) -> Sf144 {
        Self { value }
    }

    /// Returns a `Result` containing a new instance of `Sf144` using a string representation of the value in seximal form.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Sf144;
    ///
    /// let num = Sf144::from("2.3").unwrap();
    ///
    /// assert_eq!(2.5, num.value());
    /// ```
    ///
    /// # Panics
    ///
    /// It is theoretically possible for `from` to panic if the input string contains such a large or small number that the underlying f64 type overflows. This is, however, very unlikely.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if the input string contains anything besides digits 1 - 5, `-`, and `.` - or if `-` is somewhere other than the beginning or `.` appears more than once.
    pub fn from(input: &str) -> Result<Sf144, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        let parts: Vec<&str> = input.split('.').collect();

        if parts.len() > 2 {
            return Err(String::from("Input must be a seximal real number."));
        }

        let int_part: Vec<char> = parts[0].chars().collect();

        let mut int_value = 0.0;
        let mut i = int_part.len();
        let mut multiplier = 1.0;
        while i > first_pos {
            let c = int_part[i - 1];

            if c > '5' || c < '0' {
                return Err(String::from("Input must be a seximal real number."));
            }

            int_value += (c as u8 as f64 - '0' as u8 as f64) * multiplier;
            i -= 1;
            if i > first_pos {
                multiplier *= 6.0
            }
        }

        let mut value;
        if parts.len() == 2 {
            let fractional_part: Vec<char> = parts[1].chars().collect();

            let mut fractional_value = 0.0;
            i = fractional_part.len();
            multiplier = 1.0;
            while i > 0 {
                let c = fractional_part[i - 1];

                if c > '5' || c < '0' {
                    return Err(String::from("Input must be a seximal real number."));
                }

                fractional_value += (c as u8 as f64 - '0' as u8 as f64) * multiplier;
                i -= 1;
                if i > 0 {
                    multiplier *= 6.0
                }
            }

            let six: f64 = 6.0;
            value = int_value + fractional_value * six.powi(-(fractional_part.len() as i32));
        } else {
            value = int_value;
        }

        if first_pos == 1 {
            value *= -1.0;
        }

        Ok(Self { value })
    }

    /// Returns the value of the instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::Sf144;
    ///
    /// let num = Sf144::from("2.3").unwrap();
    ///
    /// assert_eq!(2.5, num.value());
    /// ```
    ///
    /// ```
    /// use seximal::Sf144;
    ///
    /// let num = Sf144::new(-1.3);
    ///
    /// assert_eq!(-1.3, num.value());
    /// ```
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Returns an instance of `Sf52` with the value of this instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use seximal::{
    ///     Sf144,
    ///     Sf52,
    /// };
    ///
    /// let a = Sf144::new(2.5);
    /// let b = a.as_sf52();
    ///
    /// assert_eq!(a.value() as f32, b.value());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the underlying `f64` value overflows when converting to `f32`.
    pub fn as_sf52(&self) -> Sf52 {
        Sf52::new(self.value as f32)
    }
}

impl fmt::Display for Sf144 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.value == 0.0 {
            return write!(f, "0");
        }

        let mut dec_value = self.value;
        let mut s;
        let index;

        let negative;
        if dec_value < 0.0 {
            s = String::from('-');
            index = 1;
            dec_value *= -1.0;
            negative = true;
        } else {
            s = String::new();
            index = 0;
            negative = false;
        }

        while dec_value > u128::MAX as f64 {
            dec_value /= 6.0;
            s.insert(index, '0');
        }
        s.insert_str(index, &format!("{}", Su332::new(dec_value as u128)));

        if s.len() < 19 || negative && s.len() < 20 {
            s.push('.');
        }

        let mut fract_part = dec_value.fract();
        while s.len() < if negative { 21 } else { 20 } {
            if s.len() == 19 {
                s.push(FractDigit::get_last_fract_digit(fract_part));
            } else {
                let mut exact = false;

                s.push(match FractDigit::get_next_fract_digit(fract_part) {
                    FractDigit::Exact(c) => {
                        exact = true;
                        c
                    }
                    FractDigit::Continue(c) => c,
                });

                if exact {
                    break;
                }
            }

            fract_part *= 6.0;
            fract_part = fract_part.fract();
        }

        write!(f, "{}", s)
    }
}

impl Ord for Sf144 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value > other.value {
            Ordering::Greater
        } else if self.value < other.value {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Sf144 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sf144 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Sf144 {}

// ----- Native Arithmetic Operators -----

impl Add for Sf144 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Sf144 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Sf144 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Sf144 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Sf144 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Sf144 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Sf144 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Sf144 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Sf144 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Sf144 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Sf144 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Sf144 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Sf144 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Sf144 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Sf144 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<f64> for Sf144 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Sf144 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<f64> for Sf144 {
    fn add_assign(&mut self, rhs: f64) {
        self.value += rhs;
    }
}

impl Sub<f64> for Sf144 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Sf144 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<f64> for Sf144 {
    fn sub_assign(&mut self, rhs: f64) {
        self.value -= rhs;
    }
}

impl Mul<f64> for Sf144 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Sf144 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<f64> for Sf144 {
    fn mul_assign(&mut self, rhs: f64) {
        self.value *= rhs;
    }
}

impl Div<f64> for Sf144 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Sf144 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<f64> for Sf144 {
    fn div_assign(&mut self, rhs: f64) {
        self.value /= rhs;
    }
}

impl Rem<f64> for Sf144 {
    type Output = Self;

    fn rem(self, rhs: f64) -> Self {
        Sf144 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<f64> for Sf144 {
    fn rem_assign(&mut self, rhs: f64) {
        self.value %= rhs;
    }
}

#[cfg(test)]
mod sf144_tests {
    use super::Sf144;
    use crate::util::ordering_to_string;
    use std::cmp::Ordering::*;

    #[test]
    fn sf144_new() {
        let num = Sf144::new(2.5);
        assert_eq!(
            num.to_string(),
            "2.3",
            "to_string failed, expected 2.3, got {}",
            num.to_string()
        );

        let num = Sf144::new(0.0);
        assert_eq!(
            num.to_string(),
            "0",
            "to_string failed, expected 0, got {}",
            num.to_string()
        );

        let num = Sf144::new(-6.25);
        assert_eq!(
            num.to_string(),
            "-10.13",
            "to_string failed, expected -10.13, got {}",
            num.to_string()
        );
    }

    #[test]
    fn sf144_from() {
        let num = Sf144::from("2.3").unwrap();
        assert_eq!(
            num.value(),
            2.5,
            "from failed, expected 2.5, got {}",
            num.value()
        );

        let num = Sf144::from("0").unwrap();
        assert_eq!(
            num.value(),
            0.0,
            "from failed, expected 0.0, got {}",
            num.value()
        );

        let num = Sf144::from("-10.13").unwrap();
        assert_eq!(
            num.value(),
            -6.25,
            "from failed, expected -6.25, got {}",
            num.value()
        );
    }

    #[test]
    #[should_panic]
    fn sf144_from_panics() {
        let _num = Sf144::from("6.6").unwrap();
    }

    #[test]
    fn sf144_native_arithmetic() {
        let mut num = Sf144::new(2.2);
        let mut reference = 2.2;
        num += Sf144::new(1.4);
        reference += 1.4;
        assert_eq!(
            num.value(),
            reference,
            "2.2 + 1.4 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num -= Sf144::new(1.4);
        reference -= 1.4;
        assert_eq!(
            num.value(),
            reference,
            "3.6 - 1.4 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num *= Sf144::new(0.7);
        reference *= 0.7;
        assert_eq!(
            num.value(),
            reference,
            "2.2 * 0.7 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num /= Sf144::new(0.7);
        reference /= 0.7;
        assert_eq!(
            num.value(),
            reference,
            "1.54 / 0.7 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num %= Sf144::new(1.1);
        reference %= 1.1;
        assert_eq!(
            num.value(),
            reference,
            "2.2 % 1.1 failed, expected {}, got {}",
            reference,
            num.value()
        );
    }

    #[test]
    fn sf144_decimal_arithmetic() {
        let mut num = Sf144::new(2.2);
        let mut reference = 2.2;
        num += 1.4;
        reference += 1.4;
        assert_eq!(
            num.value(),
            reference,
            "2.2 + 1.4 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num -= 1.4;
        reference -= 1.4;
        assert_eq!(
            num.value(),
            reference,
            "3.6 - 1.4 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num *= 0.7;
        reference *= 0.7;
        assert_eq!(
            num.value(),
            reference,
            "2.2 * 0.7 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num /= 0.7;
        reference /= 0.7;
        assert_eq!(
            num.value(),
            reference,
            "1.54 / 0.7 failed, expected {}, got {}",
            reference,
            num.value()
        );

        num %= 1.1;
        reference %= 1.1;
        assert_eq!(
            num.value(),
            reference,
            "2.2 % 1.1 failed, expected {}, got {}",
            reference,
            num.value()
        );
    }

    #[test]
    fn sf144_cmp() {
        let a = Sf144::new(3.5);
        let b = Sf144::new(5.4);
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

        let c = Sf144::new(3.5);
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
