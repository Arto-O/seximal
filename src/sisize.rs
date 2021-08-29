use crate::{
    si12::Si12, si144::Si144, si24::Si24, si332::Si332, si52::Si52, su12::Su12, su144::Su144,
    su24::Su24, su332::Su332, su52::Su52, susize::Susize,
};
use num::pow::checked_pow;
use std::{fmt, ops::*};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sisize {
    value: isize,
}

impl Sisize {
    pub fn new(value: isize) -> Sisize {
        Self { value }
    }

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

    pub fn value(&self) -> isize {
        self.value
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

impl fmt::Display for Sisize {
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
