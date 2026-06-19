mod snake;
mod food;
mod game;
mod ui;

use std::io;
use snake::*;

fn main() {
    println!("Snake Game!");
    println!("Press any key to start...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut snake= Snake::new(10, 10);
    println!("Snake head at: ({}, {})", snake.get_head().x, snake.get_head().y);
}

