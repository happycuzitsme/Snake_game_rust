#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left, 
    Right,
}

impl Direction {
    pub fn opposite (&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}
pub struct Snake {
    pub body: Vec<Position>,
    pub direction: Direction,
    pub next_direction: Direction,
}
impl Snake{
    pub fn new(x: u16, y:u16) -> Self{
        Snake{
            body: vec![Position {x,y}],
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }
    pub fn set_direction(&mut self, dir: Direction){
        if dir != self.direction.opposite(){
            self.next_direction= dir;
        }
    }

    pub fn get_head(&self) -> Position{
        self.body[0]
    }
    pub fn grow(&mut self) {
        let tail = *self.body.last().unwrap();
        self.body.push(tail);
    }
    pub fn r#move(&mut self){
        self.direction = self.next_direction;
        
        let head= self.get_head();
        let new_head= match self.direction{
            Direction::Up => Position {x: head.x, y: head.y+1},
            Direction::Down => Position {x: head.x, y: head.y+1},
            Direction::Left => Position {x: head.x-1, y: head.y},
            Direction::Right => Position {x: head.x+1, y: head.y},
        };
        self.body.insert(0, new_head);
        self.body.pop();
    }
}




