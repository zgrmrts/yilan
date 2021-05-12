use num_traits::cast::NumCast;
use num_traits::int::PrimInt;

pub struct Swing<T>
where
    T: PrimInt,
{
    pub value: T,
    max: T,
    min: T,
    increasing: bool,
}

impl<T> Swing<T>
where
    T: PrimInt,
{
    pub fn new(value: T, max: T, min: T, increasing: bool) -> Self {
        Swing {
            value,
            max,
            min,
            increasing,
        }
    }
    pub fn inc(&mut self) {
        if self.increasing {
            if self.value < self.max {
                self.value = self.value + NumCast::from(1usize).unwrap();
            } else {
                self.increasing = false;
            }
        } else {
            if self.value > self.min {
                self.value = self.value - NumCast::from(1usize).unwrap();
            } else {
                self.increasing = true;
            }
        }
    }
}
