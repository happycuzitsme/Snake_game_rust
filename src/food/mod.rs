use rand::Rng;
use super::snake::Position;

pub struct Food{
    pub position: Position,
}

impl Food{
    pub fn new(max_x: u16, max_y: u16)-> Self{
        let mut rng= rand::thread_rng();
        Food{
            position: Position{
                x: rng.gen_range(1..max_x),
                y: rng.gen_range(1..max_y),
            }
        };
    }
