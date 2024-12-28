
pub trait Operations {
	fn abs(&self) -> Self;
    fn sqrt(&self) -> Self;
}

impl Operations for f32 {
    fn abs(&self) -> Self {
        if *self < 0. {
            return -self;
        }
        return *self;
    }

    fn sqrt(&self) -> Self {
        return self.powf(0.5);
    }
}

impl Operations for f64 {
    fn abs(&self) -> Self {
        if *self < 0. {
            return -self;
        }
        return *self;
    }

    fn sqrt(&self) -> Self {
        return self.powf(0.5);
    }
}

impl Operations for i32 {
    fn abs(&self) -> Self {
        if *self < 0 {
            return -*self;
        }
        return *self;
    }

    fn sqrt(&self) -> Self {
        return (*self as f64).sqrt() as i32;
    }
}

impl Operations for i64 {
    fn abs(&self) -> Self {
        if *self < 0 {
            return -*self;
        }
        return *self;
    }

    fn sqrt(&self) -> Self {
        return (*self as f64).sqrt() as i64;
    }
}

impl Operations for i128 {
    fn abs(&self) -> Self {
        if *self < 0 {
            return -*self;
        }
        return *self;
    }

    fn sqrt(&self) -> Self {
        return (*self as f64).sqrt() as i128;
    }
}

impl Operations for u32 {
    fn abs(&self) -> Self {
        return *self;
    }

    fn sqrt(&self) -> Self {
        return (*self as f64).sqrt() as u32;
    }
}

impl Operations for u64 {
    fn abs(&self) -> Self {
        return *self;
    }

    fn sqrt(&self) -> Self {
        return (*self as f64).sqrt() as u64;
    }
}

impl Operations for u128 {
    fn abs(&self) -> Self {
        return *self;
    }

    fn sqrt(&self) -> Self {
        return (*self as f64).sqrt() as u128;
    }
}

