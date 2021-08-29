use crate::{
    si144::Si144, si24::Si24, si332::Si332, si52::Si52, sisize::Sisize, su12::Su12, su144::Su144,
    su24::Su24, su332::Su332, su52::Su52, susize::Susize,
};
use num::pow::checked_pow;
use std::{fmt, ops::*};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Si12 {
    value: i8,
}

impl Si12 {
    pub fn new(value: i8) -> Si12 {
        Self { value }
    }

    pub fn from(input: &str) -> Result<Si12, String> {
        let first_pos = if input.starts_with('-') { 1 } else { 0 };

        if checked_pow(6, input.len() - 1 - first_pos).expect("overflow") > i8::MAX as i16 {
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

    pub fn value(&self) -> i8 {
        self.value
    }

    // Conversion to signed integer types

    pub fn as_sisize(&self) -> Sisize {
        Sisize::new(self.value as isize)
    }

    pub fn as_si332(&self) -> Si332 {
        Si332::new(self.value as i128)
    }

    pub fn as_si144(&self) -> Si144 {
        Si144::new(self.value as i64)
    }

    pub fn as_si52(&self) -> Si52 {
        Si52::new(self.value as i32)
    }

    pub fn as_si24(&self) -> Si24 {
        Si24::new(self.value as i16)
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
