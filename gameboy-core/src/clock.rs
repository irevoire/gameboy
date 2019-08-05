#[derive(Clone)]
pub struct Clock {
    m: u64,
    t: u64,
}

impl Clock {
    pub fn new() -> Self {
        Clock { m: 0, t: 0 }
    }

    pub fn reset(&mut self) {
        self.m = 0;
        self.t = 0;
    }
}

impl std::ops::Add for Clock {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            m: self.m + other.m,
            t: self.t + other.t,
        }
    }
}

impl std::ops::AddAssign for Clock {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            m: self.m + other.m,
            t: self.t + other.t,
        };
    }
}
