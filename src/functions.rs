use crate::structures::{Board, Coordinate, Battlesnake};
use crate::constants;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn will_collide(board: &Board, pos: &Coordinate) -> bool{
    if pos.get_x() < 0 || pos.get_x() > board.get_width() - 1
        || pos.get_y() < 0 || pos.get_y() > board.get_height() - 1
    {
        return true;
    }

    for snake in board.get_snakes(){
        for tile in &snake.get_body()[..snake.get_length() as usize - 1] {
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
                value += constants::LARGER_HEAD / pos.radius_squared(snake.get_head());
            }
            else if snake.get_length() < you.get_length(){
                value += constants::SMALLER_HEAD / pos.radius_squared(snake.get_head());
            }
            else {
                value += constants::EQUAL_HEAD / pos.radius_squared(snake.get_head());
            }
        }

        for tile in snake.get_body(){
            if !tile.equals(you.get_head()){
                value += constants::SNAKE_BODY / pos.radius_squared(tile);
            }
        }
    }

    for tile in board.get_food(){
        value += constants::FOOD_G / pos.radius_squared(tile);
    }

    return Some(value);
}

pub fn log_data(data: String, file_name: &String) {
    println!("{}", data);
    if constants::LOGGING{

        let mut file: File = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}{}.log", constants::LOG_PATH, file_name))
            .unwrap();
        
        file.write_all(data.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod collision_tests {
    use super::*;

    #[test]
    fn body_collision() {
        let board = Board::new(
            3,
            3,
            Vec::new(),
            Vec::new(),
            vec![
                Battlesnake::new(
                    String::from("snake-id"),
                    String::from("snake-name"),
                    100,
                    vec![
                        Coordinate::new(1, 1),
                        Coordinate::new(1, 0),
                        Coordinate::new(0, 0)
                    ],
                    String::from("100"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("Test!")
                )
            ]
        );

        let body_collision = Coordinate::new(1, 0);
        assert!(will_collide(&board, &body_collision));
    }

    #[test]
    fn wall_collision() {
        let board = Board::new(
            3,
            3,
            Vec::new(),
            Vec::new(),
            vec![
                Battlesnake::new(
                    String::from("snake-id"),
                    String::from("snake-name"),
                    100,
                    vec![
                        Coordinate::new(1, 1),
                        Coordinate::new(1, 0),
                        Coordinate::new(0, 0)
                    ],
                    String::from("100"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("Test!")
                )
            ]
        );

        let wall_collision = Coordinate::new(1, 3);
        assert!(will_collide(&board, &wall_collision));
    }

    #[test]
    fn no_collision() {
        let board = Board::new(
            3,
            3,
            Vec::new(),
            Vec::new(),
            vec![
                Battlesnake::new(
                    String::from("snake-id"),
                    String::from("snake-name"),
                    100,
                    vec![
                        Coordinate::new(1, 1),
                        Coordinate::new(1, 0),
                        Coordinate::new(0, 0)
                    ],
                    String::from("100"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("Test!")
                )
            ]
        );

        let no_collision = Coordinate::new(2, 2);
        assert!(!will_collide(&board, &no_collision));
    }

    #[test]
    fn tail_collision() {
        let board = Board::new(
            3,
            3,
            Vec::new(),
            Vec::new(),
            vec![
                Battlesnake::new(
                    String::from("snake-id"),
                    String::from("snake-name"),
                    100,
                    vec![
                        Coordinate::new(1, 1),
                        Coordinate::new(1, 0),
                        Coordinate::new(0, 0)
                    ],
                    String::from("100"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("Test!")
                )
            ]
        );

        let tail_collision = Coordinate::new(0, 0);
        assert!(!will_collide(&board, &tail_collision));
    }
}

#[cfg(test)]
mod value_tests {
    use super::*;

    #[test]
    #[ignore]
    fn value_1() {
        let board = Board::new(
            3,
            3,
            vec![
                Coordinate::new(2, 1)
            ],
            Vec::new(),
            vec![
                Battlesnake::new(
                    String::from("snake-id"),
                    String::from("snake-name"),
                    100,
                    vec![
                        Coordinate::new(1, 1),
                        Coordinate::new(1, 0),
                        Coordinate::new(0, 0)
                    ],
                    String::from("100"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("Test!")
                )
            ]
        );

        let you = Battlesnake::new(
            String::from("snake-id"),
            String::from("snake-name"),
            100,
            vec![
                Coordinate::new(1, 1),
                Coordinate::new(1, 0),
                Coordinate::new(0, 0)
            ],
            String::from("100"),
            Coordinate::new(0, 0),
            3,
            String::from("Test!")
        );

        let pos = Coordinate::new(2, 1);

        
    }
}
