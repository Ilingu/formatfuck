use std::ops::RangeInclusive;

use super::Animation;

pub struct MakeItRain {
    window_size: RangeInclusive<usize>,
    
}

impl Animation for MakeItRain {
    fn new() -> Self {
        todo!()
    }

    fn compute_next_frame(&mut self) {
        todo!()
    }
}
