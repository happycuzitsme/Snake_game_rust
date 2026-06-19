use super::snake::{Snake, Position};
use super::food::Food;

pub struct Game{
    pub snake: Snake,
    pub food: Food,
    pub score: u32,
    pub max_x: u16,
    pub max_y: u16,
    pub game_over: bool,
}

impl Game{
    pub fn new(width: u16, height: u16) -> Self{
        let snake= Snake::new(width/2, height/2);
        let food= Food::new(width-1, height-1);

        Game{
            snake, 
            food,
            score:0,
            max_x: width-1,
            max_y: height-1
            game_over: false,
        }
    }

    pub fn update(&mut self){
        if self.game_over{
            return;
        }
        self.snake.r#move();

        //Check if snake ate food
        let head= self.snake.get_head();
        if head.x==self.food.position.x && head.y== self.food.position.y{
            self.snake.grow();
            self.score +=1;
            self.food.respawn(self.max_x, self.max_y);
        }
    }
}
