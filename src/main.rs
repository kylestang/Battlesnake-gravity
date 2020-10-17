use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use structures::{MoveRequest, IndexResponse};
use decision::decision;

mod structures;
mod decision;
mod constants;
mod functions;
mod draw;

#[cfg(test)]
mod tests;

#[get("/battlesnake/gravity")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(IndexResponse::new(
        constants::API_VERSION,
        constants::AUTHOR,
        constants::COLOR,
        constants::HEAD,
        constants::TAIL
    ))
}

#[post("/battlesnake/gravity/start")]
async fn start() -> HttpResponse {
    println!("Start");
    HttpResponse::Ok().body("")
}

#[post("/battlesnake/gravity/move")]
async fn game_move(data: web::Json<MoveRequest>) -> HttpResponse {
    println!("Move");
    HttpResponse::Ok().json(decision(data.get_game(), data.get_turn(), data.get_board(), data.get_you()))
}

#[post("/battlesnake/gravity/end")]
async fn end() -> HttpResponse {
    println!("End");
    HttpResponse::Ok().body("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(start)
            .service(game_move)
            .service(end)
    })
    .bind("0.0.0.0:25569")?
    .run()
    .await
}
