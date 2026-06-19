mod snake;
mod food;
mod game;
mod ui;

use std::io;
use snake::*;
use game::Game;
fn main() {
    println!("Snake Game!");
    println!("Press any key to start...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut snake= Snake::new(10, 10);
    println!("Snake head at: ({}, {})", snake.get_head().x, snake.get_head().y);
    let mut game= Game::new(40,20);
    loop{
        if game.game_over {
            println!("Game over! Score: {}", game.score);
            println!("Press 'r' to restart, 'q' to quit");
            break;
        }
        game.update();
        game.check_collisions();
        println!("Score: {}", game.score);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

}

