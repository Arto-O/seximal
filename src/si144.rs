use crate::{si12::Si12, si24::Si24, si332::Si332, si52::Si52, sisize::Sisize};
use num::pow::checked_pow;
use std::{fmt, ops::*};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Si144 {
    value: i64,
}

impl Si144 {
    pub fn new(value: i64) -> Si144 {
        Self { value }
    }

    pub fn from(input: &str) -> Result<Si144, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        if checked_pow(6, input.len() - 1 - first_pos).expect("overflow") > i64::MAX as i128 {
            return Err(String::from("overflow"));
        }

        let mut v = Vec::new();

        for c in input.chars() {
            v.push(c);
        }

        let mut value = 0;
        let mut i = v.len();
        let mut multiplier = 1;
        while i > first_pos {
            let c = v[i - 1];

            if c > '5' || c < '0' {
                return Err(String::from("Input must be a seximal integer."));
            }

            value += (c as i64 - '0' as i64) * multiplier;
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

    pub fn value(&self) -> i64 {
        self.value
    }

    pub fn as_sisize(&self) -> Sisize {
        Sisize::new(self.value as isize)
    }

    pub fn as_si332(&self) -> Si332 {
        Si332::new(self.value as i128)
    }

    pub fn as_si52(&self) -> Si52 {
        Si52::new(self.value as i32)
    }

    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    pub fn as_si12(&self) -> Si12 {
        Si12::new(self.value as i8)
    }
}

impl fmt::Display for Si144 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dec_value = self.value;
        let mut s;
        let index;

        if dec_value < 0 {
            s = String::from('-');
            index = 1;
            dec_value *= -1;
        } else {
            s = String::from("");
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

impl Add for Si144 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Si144 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Si144 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Si144 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Si144 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Si144 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Si144 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Si144 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Si144 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Si144 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Si144 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Si144 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Si144 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Si144 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Si144 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<i64> for Si144 {
    type Output = Self;

    fn add(self, rhs: i64) -> Self {
        Si144 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<i64> for Si144 {
    fn add_assign(&mut self, rhs: i64) {
        self.value += rhs;
    }
}

impl Sub<i64> for Si144 {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self {
        Si144 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<i64> for Si144 {
    fn sub_assign(&mut self, rhs: i64) {
        self.value -= rhs;
    }
}

impl Mul<i64> for Si144 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Si144 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<i64> for Si144 {
    fn mul_assign(&mut self, rhs: i64) {
        self.value *= rhs;
    }
}

impl Div<i64> for Si144 {
    type Output = Self;

    fn div(self, rhs: i64) -> Self {
        Si144 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<i64> for Si144 {
    fn div_assign(&mut self, rhs: i64) {
        self.value /= rhs;
    }
}

impl Rem<i64> for Si144 {
    type Output = Self;

    fn rem(self, rhs: i64) -> Self {
        Si144 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<i64> for Si144 {
    fn rem_assign(&mut self, rhs: i64) {
        self.value %= rhs;
    }
}