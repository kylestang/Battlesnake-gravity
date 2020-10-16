use crate::structures::{Board, Coordinate, Battlesnake};
use crate::constants;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn will_collide(board: &Board, pos: &Coordinate) -> bool{
    if pos.get_x() < 0 || pos.get_x() > board.get_width() - 1
        ||pos.get_y() < 0 || pos.get_y() > board.get_height() - 1
    {
        return true;
    }

    for snake in board.get_snakes(){
        for tile in snake.get_body(){
            if tile.equals(pos){
                return true;
            }
        }
    }

    return false;
}

pub fn calculate_value(board: &Board, you: &Battlesnake, pos: &Coordinate) -> Option<f64> {
    if will_collide(board, pos){
        return None;
    }

    let mut value: f64 = 0.0;
    
    // Bottom wall, top wall, right wall, left wall
    value += constants::WALL_G / ((pos.get_y() + 1).pow(2) as f64);
    value += constants::WALL_G / ((board.get_height() - pos.get_y()).pow(2) as f64);
    value += constants::WALL_G / ((board.get_width() - pos.get_x()).pow(2) as f64);
    value += constants::WALL_G / ((pos.get_x() + 1).pow(2) as f64);

    // Snake heads and bodies
    for snake in board.get_snakes(){
        if snake.get_id() != you.get_id(){
            if snake.get_length() > you.get_length(){
                value += constants::LARGER_HEAD / you.get_head().radius_squared(snake.get_head());
            }
            if snake.get_length() < you.get_length(){
                value += constants::SMALLER_HEAD / you.get_head().radius_squared(snake.get_head());
            }
            else {
                value += constants::EQUAL_HEAD / you.get_head().radius_squared(snake.get_head());
            }
        }

        for tile in snake.get_body(){
            if !tile.equals(you.get_head()){
                value += constants::SNAKE_BODY / you.get_head().radius_squared(tile);
            }
        }
    }

    for tile in board.get_food(){
        value += constants::FOOD_G / you.get_head().radius_squared(tile);
    }

    return Some(value);
}

pub fn log_data(data: String, file_name: &String) {
    println!("{}", data);
    if constants::LOGGING{

        let mut file: File = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}{}", constants::LOG_PATH, file_name))
            .unwrap();
        
        file.write_all(data.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod function_tests {

    #[test]
    #[ignore]
    fn test_will_collide() {
        unimplemented!();
    }
}
