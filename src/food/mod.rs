use rand::Rng;
use super::snake::Position;

pub struct Food{
    pub position: Position,
}

impl Food {
    pub fn new(max_x: u16, max_y: u16, snake_body: &[Position]) -> Self {
        let mut rng= rand::thread_rng();
        let mut position = Position { x: 1, y: 1 };
        for _ in 0..100 {
            let pos = Position {
                x: rng.gen_range(1..max_x),
                y: rng.gen_range(1..max_y),
            };
            if !snake_body.contains(&pos) {
                position = pos;
                break;
            }
        }
        Food { position }
    }

    pub fn respawn(&mut self, max_x: u16, max_y: u16, snake_body: &[Position]){
        let mut rng= rand::thread_rng();
        for _ in 0..100 {
            let pos = Position {
                x: rng.gen_range(1..max_x),
                y: rng.gen_range(1..max_y),
            };
            if !snake_body.contains(&pos) {
                self.position = pos;
                return;
            }
        }
        // Fallback
        self.position = Position {
            x: rng.gen_range(1..max_x),
            y: rng.gen_range(1..max_y),
        };
    }
}
