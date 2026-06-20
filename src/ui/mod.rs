use crossterm::{
    cursor, execute, terminal,
    style::Stylize,
};
use std::io::{stdout, Write};
use super::game::Game;

pub fn render(game: &Game){
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    ).unwrap();
    //Draw border
    let width= game.max_x;
    let height= game.max_y;

    for y in 0..=height{
        for x in 0..=width{
            if x==0|| x==width|| y== 0 || y==height{
                print!("{}", "#".blue());
            }else if game.snake.get_head().x== x && game.snake.get_head().y==y{
                print!("{}", "O".green().bold());
            }else if game.snake.body.iter().any(|p| p.x == x && p.y ==y){
                print!("{}", "o".green());
            }else if game.food.position.x ==x && game.food.position.y == y {
                print!("{}", "*".red().bold());
            }else{
                print!(" ");
            }
        }
        print!("\r\n");
    }
    print!("Score: {}\r\n", game.score.to_string().yellow().bold());
    if game.game_over{
        print!("{}\r\n", "GAME OVER! Press 'r' to restart".red().bold());
    }
    stdout.flush().unwrap();
}

