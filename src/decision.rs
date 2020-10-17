use crate::structures::{Coordinate, Game, Battlesnake, Board, MoveResponse};
use crate::functions::{calculate_value, log_data};
use crate::constants::DRAWING;
use crate::draw::draw_board;

pub fn decision(game: &Game, turn: i64, board: &Board, you: &Battlesnake) -> MoveResponse {

    if DRAWING {
        draw_board(board, you, format!("{}", turn));
    }

    let down: Coordinate = you.get_head().down();
    let up: Coordinate = you.get_head().up();
    let right: Coordinate = you.get_head().right();
    let left: Coordinate = you.get_head().left();
    
    let down_value = calculate_value(board, you, &down);
    let up_value = calculate_value(board, you, &up);
    let right_value = calculate_value(board, you, &right);
    let left_value = calculate_value(board, you, &left);

    let mut direction = String::from("down");
    let mut current_value: Option<f64> = None;

    if down_value.is_some() && (current_value.is_none() || down_value.unwrap() < current_value.unwrap()) {
        current_value = down_value;
        direction = String::from("down");
    }
    if up_value.is_some() && (current_value.is_none() || up_value.unwrap() < current_value.unwrap()) {
        current_value = up_value;
        direction = String::from("up");
    }
    if right_value.is_some() && (current_value.is_none() || right_value.unwrap() < current_value.unwrap()) {
        current_value = right_value;
        direction = String::from("right");
    }
    if left_value.is_some() && (current_value.is_none() || left_value.unwrap() < current_value.unwrap()) {
        current_value = left_value;
        direction = String::from("left");
    }

    log_data(format!("\nTurn: {}\n", turn), game.get_id());
    log_data(format!("Down: {:?}\n", down_value), game.get_id());
    log_data(format!("Up: {:?}\n", up_value), game.get_id());
    log_data(format!("Right: {:?}\n", right_value), game.get_id());
    log_data(format!("Left: {:?}\n", left_value), game.get_id());
    log_data(format!("Direction: {}\n", direction), game.get_id());

    MoveResponse::new(direction, String::from("AHHHHHHHHHH"))
}
