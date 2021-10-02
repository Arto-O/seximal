pub enum FractDigit {
    Exact(char),
    Continue(char),
}

impl FractDigit {
    pub fn get_next_fract_digit(fraction: f64) -> FractDigit {
        let mut digit: u8 = 3;
        if digit as f64 / 6.0 < fraction {
            digit += 1;

            loop {
                if digit as f64 / 6.0 < fraction {
                    digit += 1;
                } else if digit == 5 {
                    break;
                } else if digit as f64 == fraction {
                    return Self::Exact((digit + '0' as u8) as char);
                } else {
                    digit -= 1;
                    break;
                }
            }
        } else if digit as f64 / 6.0 > fraction {
            digit -= 1;

            while digit > 0 {
                if digit as f64 / 6.0 > fraction {
                    digit -= 1;
                } else if digit as f64 == fraction {
                    return Self::Exact((digit + '0' as u8) as char);
                } else {
                    break;
                }
            }
        } else {
            return Self::Exact((digit + '0' as u8) as char);
        }

        Self::Continue((digit + '0' as u8) as char)
    }

    pub fn get_last_fract_digit(fraction: f64) -> char {
        let mut digit: u8 = 3;
        if digit as f64 / 6.0 < fraction {
            let mut previous = digit;
            digit += 1;

            loop {
                if digit as f64 / 6.0 < fraction {
                    previous = digit;
                    digit += 1;
                } else if digit == 5 {
                    break;
                } else if digit as f64 == fraction {
                    return (digit + '0' as u8) as char;
                } else {
                    if fraction - previous as f64 / 6.0 < digit as f64 / 6.0 - fraction {
                        digit = previous;
                    }

                    break;
                }
            }
        } else if digit as f64 / 6.0 > fraction {
            let mut previous = digit;
            digit -= 1;

            while digit > 0 {
                if digit as f64 / 6.0 > fraction {
                    previous = digit;
                    digit -= 1;
                } else if digit as f64 == fraction {
                    return (digit + '0' as u8) as char;
                } else {
                    if previous as f64 / 6.0 - fraction <= fraction - digit as f64 / 6.0 {
                        digit = previous;
                    }

                    break;
                }
            }
        }

        (digit + '0' as u8) as char
    }
}
