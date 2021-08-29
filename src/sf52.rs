use crate::sf144::Sf144;
use crate::su332::Su332;
use std::{cmp::Ordering, fmt, ops::*};

#[derive(Copy, Clone)]
pub struct Sf52 {
    value: f32,
}

impl Sf52 {
    pub fn new(value: f32) -> Sf52 {
        Self { value }
    }

    pub fn from(input: &str) -> Result<Sf52, String> {
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

            int_value += (c as u8 as f32 - '0' as u8 as f32) * multiplier;
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

                fractional_value += (c as u8 as f32 - '0' as u8 as f32) * multiplier;
                i -= 1;
                if i > 0 {
                    multiplier *= 6.0
                }
            }

            let six: f32 = 6.0;
            value = int_value + fractional_value * six.powi(-(fractional_part.len() as i32));
        } else {
            value = int_value;
        }

        if first_pos == 1 {
            value *= -1.0;
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn as_sf144(&self) -> Sf144 {
        Sf144::new(self.value as f64)
    }
}

impl fmt::Display for Sf52 {
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

        while dec_value > u128::MAX as f32 {
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
                s.push(get_last_fract_digit(fract_part));
            } else {
                let mut exact = false;

                s.push(match get_next_fract_digit(fract_part) {
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

enum FractDigit {
    Exact(char),
    Continue(char),
}

fn get_next_fract_digit(fraction: f32) -> FractDigit {
    let mut digit: u8 = 3;
    if digit as f32 / 6.0 < fraction {
        digit += 1;

        loop {
            if digit as f32 / 6.0 < fraction {
                digit += 1;
            } else if digit == 5 {
                break;
            } else if digit as f32 == fraction {
                return FractDigit::Exact((digit + '0' as u8) as char);
            } else {
                digit -= 1;
                break;
            }
        }
    } else if digit as f32 / 6.0 > fraction {
        digit -= 1;

        while digit > 0 {
            if digit as f32 / 6.0 > fraction {
                digit -= 1;
            } else if digit as f32 == fraction {
                return FractDigit::Exact((digit + '0' as u8) as char);
            } else {
                break;
            }
        }
    } else {
        return FractDigit::Exact((digit + '0' as u8) as char);
    }

    FractDigit::Continue((digit + '0' as u8) as char)
}

fn get_last_fract_digit(fraction: f32) -> char {
    let mut digit: u8 = 3;
    if digit as f32 / 6.0 < fraction {
        let mut previous = digit;
        digit += 1;

        loop {
            if digit as f32 / 6.0 < fraction {
                previous = digit;
                digit += 1;
            } else if digit == 5 {
                break;
            } else if digit as f32 == fraction {
                return (digit + '0' as u8) as char;
            } else {
                if fraction - previous as f32 / 6.0 < digit as f32 / 6.0 - fraction {
                    digit = previous;
                }

                break;
            }
        }
    } else if digit as f32 / 6.0 > fraction {
        let mut previous = digit;
        digit -= 1;

        while digit > 0 {
            if digit as f32 / 6.0 > fraction {
                previous = digit;
                digit -= 1;
            } else if digit as f32 == fraction {
                return (digit + '0' as u8) as char;
            } else {
                if previous as f32 / 6.0 - fraction <= fraction - digit as f32 / 6.0 {
                    digit = previous;
                }

                break;
            }
        }
    }

    (digit + '0' as u8) as char
}

impl Ord for Sf52 {
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

impl PartialOrd for Sf52 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Sf52 {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Sf52 {}

// ----- Native Arithmetic Operators -----

impl Add for Sf52 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Sf52 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Sf52 {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
    }
}

impl Sub for Sf52 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Sf52 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Sf52 {
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}

impl Mul for Sf52 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Sf52 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign for Sf52 {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
    }
}

impl Div for Sf52 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Sf52 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign for Sf52 {
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl Rem for Sf52 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Sf52 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign for Sf52 {
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value;
    }
}

// ----- Decimal Arithmetic Operators -----

impl Add<f32> for Sf52 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self {
        Sf52 {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<f32> for Sf52 {
    fn add_assign(&mut self, rhs: f32) {
        self.value += rhs;
    }
}

impl Sub<f32> for Sf52 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self {
        Sf52 {
            value: self.value - rhs,
        }
    }
}

impl SubAssign<f32> for Sf52 {
    fn sub_assign(&mut self, rhs: f32) {
        self.value -= rhs;
    }
}

impl Mul<f32> for Sf52 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Sf52 {
            value: self.value * rhs,
        }
    }
}

impl MulAssign<f32> for Sf52 {
    fn mul_assign(&mut self, rhs: f32) {
        self.value *= rhs;
    }
}

impl Div<f32> for Sf52 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Sf52 {
            value: self.value / rhs,
        }
    }
}

impl DivAssign<f32> for Sf52 {
    fn div_assign(&mut self, rhs: f32) {
        self.value /= rhs;
    }
}

impl Rem<f32> for Sf52 {
    type Output = Self;

    fn rem(self, rhs: f32) -> Self {
        Sf52 {
            value: self.value % rhs,
        }
    }
}

impl RemAssign<f32> for Sf52 {
    fn rem_assign(&mut self, rhs: f32) {
        self.value %= rhs;
    }
}
