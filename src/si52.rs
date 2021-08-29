use crate::{
    si12::Si12, si144::Si144, si24::Si24, si332::Si332, sisize::Sisize, su12::Su12, su144::Su144,
    su24::Su24, su332::Su332, su52::Su52, susize::Susize,
};
use num::pow::checked_pow;
use std::{fmt, ops::*};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Si52 {
    value: i32,
}

impl Si52 {
    pub fn new(value: i32) -> Si52 {
        Self { value }
    }

    pub fn from(input: &str) -> Result<Si52, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        if checked_pow(6, input.len() - 1 - first_pos).expect("overflow") > i32::MAX as i64 {
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

            value += (c as i32 - '0' as i32) * multiplier;
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

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn as_sisize(&self) -> Sisize {
        Sisize::new(self.value as isize)
    }

    pub fn as_si332(&self) -> Si332 {
        Si332::new(self.value as i128)
    }

    pub fn as_si144(&self) -> Si144 {
        Si144::new(self.value as i64)
    }

    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
    }

    pub fn as_si12(&self) -> Si12 {
        Si12::new(self.value as i8)
    }

    // Conversion to unsigned integer types

    pub fn as_susize(&self) -> Susize {
        Susize::new(self.value as usize)
    }

    pub fn as_su332(&self) -> Su332 {
        Su332::new(self.value as u128)
    }

    pub fn as_su144(&self) -> Su144 {
        Su144::new(self.value as u64)
    }

    pub fn as_su52(&self) -> Su52 {
        Su52::new(self.value as u32)
    }

    pub fn as_su24(&self) -> Su24 {
        Su24::new(self.value as u16)
    }

    pub fn as_su12(&self) -> Su12 {
        Su12::new(self.value as u8)
    }
}

impl fmt::Display for Si52 {
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

impl Add for Si52 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Si52 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Si52 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Si52 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Si52 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Si52 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Si52 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Si52 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Si52 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Si52 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Si52 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Si52 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Si52 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Si52 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Si52 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<i32> for Si52 {
    type Output = Self;

    fn add(self, rhs: i32) -> Self {
        Si52 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<i32> for Si52 {
    fn add_assign(&mut self, rhs: i32) {
        self.value += rhs;
    }
}

impl Sub<i32> for Si52 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self {
        Si52 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<i32> for Si52 {
    fn sub_assign(&mut self, rhs: i32) {
        self.value -= rhs;
    }
}

impl Mul<i32> for Si52 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Si52 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<i32> for Si52 {
    fn mul_assign(&mut self, rhs: i32) {
        self.value *= rhs;
    }
}

impl Div<i32> for Si52 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        Si52 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<i32> for Si52 {
    fn div_assign(&mut self, rhs: i32) {
        self.value /= rhs;
    }
}

impl Rem<i32> for Si52 {
    type Output = Self;

    fn rem(self, rhs: i32) -> Self {
        Si52 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<i32> for Si52 {
    fn rem_assign(&mut self, rhs: i32) {
        self.value %= rhs;
    }
}
