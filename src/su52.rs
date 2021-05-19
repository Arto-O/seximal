use crate::{
    si12::Si12, si144::Si144, si24::Si24, si332::Si332, si52::Si52, sisize::Sisize, su12::Su12,
    su144::Su144, su24::Su24, su332::Su332, susize::Susize,
};
use num::pow::checked_pow;
use std::{fmt, ops::*};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Su52 {
    value: u32,
}

impl Su52 {
    pub fn new(value: u32) -> Su52 {
        Self { value }
    }

    pub fn from(input: &str) -> Result<Su52, String> {
        match checked_pow(6, input.len() - 1) {
            Some(_) => (),
            None => return Err(String::from("overflow")),
        }

        let mut v = Vec::new();

        for c in input.chars() {
            v.push(c);
        }

        let mut value = 0;
        let mut i = v.len();
        let mut multiplier = 1;
        while i > 0 {
            let c = v[i - 1];

            if c > '5' || c < '0' {
                return Err(String::from("Input must be a seximal whole number."));
            }

            value += (c as u32 - '0' as u32) * multiplier;
            i -= 1;
            if i > 0 {
                multiplier *= 6
            }
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> u32 {
        self.value
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

    pub fn as_su24(&self) -> Su24 {
        Su24::new(self.value as u16)
    }

    pub fn as_su12(&self) -> Su12 {
        Su12::new(self.value as u8)
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

    pub fn as_si12(&self) -> Si12 {
        Si12::new(self.value as i8)
    }
}

impl fmt::Display for Su52 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut dec_value = self.value;
        let mut s = String::from("");

        while dec_value >= 6 {
            s.insert(0, ((dec_value % 6) as u8 + '0' as u8) as char);
            dec_value /= 6;
        }
        s.insert(0, (dec_value as u8 + '0' as u8) as char);

        write!(f, "{}", s)
    }
}

// ----- Native Arithmetic Operators -----

impl Add for Su52 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Su52 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Su52 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Su52 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Su52 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Su52 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Su52 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Su52 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Su52 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Su52 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Su52 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Su52 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Su52 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Su52 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Su52 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<u32> for Su52 {
    type Output = Self;

    fn add(self, rhs: u32) -> Self {
        Su52 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<u32> for Su52 {
    fn add_assign(&mut self, rhs: u32) {
        self.value += rhs;
    }
}

impl Sub<u32> for Su52 {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self {
        Su52 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<u32> for Su52 {
    fn sub_assign(&mut self, rhs: u32) {
        self.value -= rhs;
    }
}

impl Mul<u32> for Su52 {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        Su52 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<u32> for Su52 {
    fn mul_assign(&mut self, rhs: u32) {
        self.value *= rhs;
    }
}

impl Div<u32> for Su52 {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        Su52 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<u32> for Su52 {
    fn div_assign(&mut self, rhs: u32) {
        self.value /= rhs;
    }
}

impl Rem<u32> for Su52 {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self {
        Su52 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<u32> for Su52 {
    fn rem_assign(&mut self, rhs: u32) {
        self.value %= rhs;
    }
}
