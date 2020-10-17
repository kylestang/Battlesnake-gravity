use crate::structures::{Coordinate, Board, Battlesnake};
use image::{RgbImage, Rgb};
use crate::functions::calculate_value;
use crate::constants::DRAW_PATH;

pub fn draw_board(board: &Board, you: &Battlesnake, name: &str){
    let tile_size = 50;

    let imgx = tile_size * board.get_width() as u32;
    let imgy = tile_size * board.get_height() as u32;

    let mut img = RgbImage::new(imgx, imgy);

    for x in 0..board.get_width() as u32 {
        for y in 0..board.get_height() as u32 {
            let mut value = calculate_value(&board, &you, &Coordinate::new(x as i64, y as i64)).unwrap_or(10.0);
            value = (value + 10.0) * 25.5;
            if value > 510.0 { value = 510.0 }
            else if value < 0.0 { value = 0.0 }

            let b = if value <= 255.0 {255.0 - value} else {0.0};
            let g = if value <= 255.0 {value/ 2.0} else {(510.0 - value) / 2.0};
            let r = if value - 255.0 > 0.0 {value - 255.0} else {0.0};

            for tile_x in 0..tile_size {
                for tile_y in 0..tile_size {
                    let x_pixel= x * tile_size + tile_x;
                    let y_pixel = y * tile_size + tile_y;
                    img.put_pixel(x_pixel, y_pixel, Rgb([r as u8, g as u8, b as u8]));
                }
            }

        }
    }

    for tile in board.get_food() {
        draw_food(&mut img, tile_size as i64, tile);
    }

    img.save(format!("{}{}.png", DRAW_PATH, name));
    
}

fn draw_food(img: &mut RgbImage, tile_size: i64, pos: &Coordinate){
    let food_radius = 20.0;

    for x in 0..tile_size{
        for y in 0..tile_size{
            let radius = (((x - tile_size / 2).pow(2) + (y - tile_size / 2).pow(2)) as f64).sqrt();
            let x_pixel = x + tile_size * pos.get_x();
            let y_pixel = y + tile_size * pos.get_y();
            if radius <= food_radius {
                img.put_pixel(x_pixel as u32, y_pixel as u32, Rgb([255, 0, 0]));
            }
        }
    }
}

#[cfg(test)]
mod test_draw{
    use super::*;

    #[test]
    fn test_basic(){
        let board = Board::new(
            11,
            11,
            vec![
                Coordinate::new(5, 5),
                Coordinate::new(9, 0),
                Coordinate::new(2, 6)
            ],
            vec![
                Coordinate::new(0, 0)
            ],
            vec![
                Battlesnake::new(
                    String::from("snake-508e96ac-94ad-11ea-bb37"),
                    String::from("My Snake"),
                    54,
                    vec![
                        Coordinate::new(0, 0),
                        Coordinate::new(1, 0),
                        Coordinate::new(2, 0)
                    ],
                    String::from("111"),
                    Coordinate::new(0, 0),
                    3,
                    String::from("why are we shouting??"),
                ),
                Battlesnake::new(
                    String::from("snake-b67f4906-94ae-11ea-bb37"),
                    String::from("Another Snake"),
                    16,
                    vec![
                        Coordinate::new(5, 4),
                        Coordinate::new(5, 3),
                        Coordinate::new(6, 3),
                        Coordinate::new(6, 2)
                    ],
                    String::from("222"),
                    Coordinate::new(5, 4),
                    4,
                    String::from("I'm not really sure..."),
                )
            ]
        );

        let you = Battlesnake::new(
            String::from("snake-508e96ac-94ad-11ea-bb37"),
            String::from("My Snake"),
            54,
            vec![
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0)
            ],
            String::from("111"),
            Coordinate::new(0, 0),
            3,
            String::from("why are we shouting??"),
        );

        draw_board(&board, &you, "Test");
        
        assert!(true);
    }
}
