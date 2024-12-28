use std::ops::{Add, Sub, Mul, Div};
use std::fmt::Display;

use crate::operations::Operations;

#[derive(Debug, Copy, Clone)]
pub struct Complex {
	r: f32,
	i: f32,
}

impl Complex {
    pub fn new(r: f32, i: f32) -> Complex {
        return Complex { r, i };
    }
}

impl From<(f32, f32)> for Complex {
    fn from(data: (f32, f32)) -> Complex {
        return Complex { r: data.0, i: data.1 };
    }
}

impl Operations for Complex {
    fn abs(&self) -> Complex {
        let r = (self.r * self.r + self.i * self.i).sqrt();
        let i = 0.0;
        return Complex { r, i };
    }

    fn sqrt(&self) -> Self {
        // This is not the correct way to calculate the square root of a complex number
        // However, it will only be used with complex numbers with no imaginary part
        if self.i != 0. {
            panic!("The square root of a complex number with an imaginary part is not supported");
        }
        return Complex {
            r: self.abs().r.sqrt(),
            i: 0.,
        };
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "({} + {}i)", self.r, self.i);
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, c: Complex) -> Complex {
        let r = self.r + c.r;
        let i = self.i + c.i;
        return Complex { r, i };
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, c: Complex) -> Complex {
        let r = self.r - c.r;
        let i = self.i - c.i;
        return Complex { r, i };
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, c: Complex) -> Complex {
        let r = self.r * c.r - self.i * c.i;
        let i = self.r * c.i + self.i * c.r;
        return Complex { r, i };
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, c: Complex) -> Complex {
        let r = (self.r * c.r + self.i * c.i) / (c.r * c.r + c.i * c.i);
        let i = (self.i * c.r - self.r * c.i) / (c.r * c.r + c.i * c.i);
        return Complex { r, i };
    }
}

impl PartialEq for Complex {
    fn eq(&self, c: &Complex) -> bool {
        return self.r == c.r && self.i == c.i;
    }
}

impl PartialOrd for Complex {
    fn partial_cmp(&self, c: &Complex) -> Option<std::cmp::Ordering> {
        if self.r == c.r && self.i == c.i {
            return Some(std::cmp::Ordering::Equal);
        } else if self.r > c.r && self.i > c.i {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

impl std::ops::Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        return Complex { r: -self.r, i: -self.i };
    }
}

impl Default for Complex {
    fn default() -> Complex {
        return Complex { r: 0.0, i: 0.0 };
    }
}
