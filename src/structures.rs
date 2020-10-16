use serde::{Serialize, Deserialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
pub struct Coordinate {
    x: i64,
    y: i64
}

impl Coordinate {
    pub fn new(x: i64, y: i64) -> Coordinate {
        Coordinate {x, y}
    }

    pub fn get_x(&self) -> i64 {
        self.x
    }

    pub fn get_y(&self) -> i64 {
        self.y
    }

    pub fn equals(&self, pos: &Coordinate) -> bool {
        self.x == pos.x && self.y == pos.y
    }

    pub fn distance_to(&self, pos: &Coordinate) -> i64 {
        (self.x - pos.x) + (self.y - pos.y).pow(2)
    }

    pub fn radius_squared(&self, pos: &Coordinate) -> f64 {
        ((self.x - pos.x).pow(2) + (self.y - pos.y).pow(2)) as f64
    }

    pub fn down(&self) -> Coordinate {
        Coordinate::new(self.x, self.y - 1)
    }

    pub fn up(&self) -> Coordinate {
        Coordinate::new(self.x, self.y + 1)
    }

    pub fn right(&self) -> Coordinate {
        Coordinate::new(self.x + 1, self.y)
    }

    pub fn left(&self) -> Coordinate {
        Coordinate::new(self.x - 1, self.y)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: String,
    ruleset: Ruleset,
    timeout: i64
}

impl Game {
    pub fn new(id: String, ruleset: Ruleset, timeout: i64) -> Game {
        Game {id, ruleset, timeout}
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_ruleset(&self) -> &Ruleset {
        &self.ruleset
    }

    pub fn get_timeout(&self) -> i64 {
        self.timeout
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ruleset {
    name: String,
    version: String
}

impl Ruleset {
    pub fn new(name: String, version: String) -> Ruleset {
        Ruleset {name, version}
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_version(&self) -> &String {
        &self.version
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Battlesnake {
    id: String,
    name: String,
    health: i64,
    body: Vec<Coordinate>,
    latency: String,
    head: Coordinate,
    length: i64,
    shout: String,
}

impl Battlesnake {
    pub fn new(
        id: String, name: String, health: i64, body: Vec<Coordinate>, latency: String,
        head: Coordinate, length: i64, shout: String
    ) -> Battlesnake {
        Battlesnake {id, name, health, body, latency, head, length, shout}
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_health(&self) -> i64 {
        self.health
    }

    pub fn get_body(&self) -> &Vec<Coordinate> {
        &self.body
    }

    pub fn get_latency(&self) -> &String {
        &self.latency
    }

    pub fn get_head(&self) -> &Coordinate {
        &self.head
    }

    pub fn get_length(&self) -> i64 {
        self.length
    }

    pub fn get_shout(&self) -> &String {
        &self.shout
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    height: i64,
    width: i64,
    food: Vec<Coordinate>,
    hazards: Vec<Coordinate>,
    snakes: Vec<Battlesnake>
}

impl Board {
    pub fn new(height: i64, width: i64, food: Vec<Coordinate>, hazards: Vec<Coordinate>, snakes: Vec<Battlesnake>) -> Board {
        Board {height, width, food, hazards, snakes}
    }

    pub fn get_height(&self) -> i64 {
        self.height
    }

    pub fn get_width(&self) -> i64 {
        self.width
    }

    pub fn get_food(&self) -> &Vec<Coordinate> {
        &self.food
    }

    pub fn get_hazards(&self) -> &Vec<Coordinate> {
        &self.hazards
    }

    pub fn get_snakes(&self) -> &Vec<Battlesnake> {
        &self.snakes
    }
}

#[derive(Serialize, Deserialize, std::fmt::Debug)]
pub struct MoveRequest {
    game: Game,
    turn: i64,
    board: Board,
    you: Battlesnake
}

impl MoveRequest {
    pub fn new(game: Game, turn: i64, board: Board, you: Battlesnake) -> MoveRequest {
        MoveRequest {game, turn, board, you}
    }

    pub fn get_game(&self) -> &Game {
        &self.game
    }

    pub fn get_turn(&self) -> i64 {
        self.turn
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_you(&self) -> &Battlesnake {
        &self.you
    }
}

#[derive(Serialize)]
pub struct IndexResponse {
    apiversion: &'static str,
    author: &'static str,
    color: &'static str,
    head: &'static str,
    tail: &'static str,
}

impl IndexResponse {
    pub fn new(
        apiversion: &'static str, author: &'static str,
        color: &'static str, head: &'static str, tail: &'static str
    ) -> IndexResponse {
        IndexResponse {apiversion, author, color, head, tail}
    }
}

#[derive(Serialize)]
pub struct MoveResponse {
    r#move: String,
    shout:String
}

impl MoveResponse {
    pub fn new(r#move: String, shout: String) -> MoveResponse {
        MoveResponse {r#move, shout}
    }
}
