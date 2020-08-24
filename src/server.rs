use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

use crate::board;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello World".into()))
}

async fn boogie_board(req: Request<String>) -> Result<Response<Body>, Infallible> {
    match (req.uri().path(), req.method()) {
        ("/board", &Method::POST) => {
            println!("Received Sudoko Board: {}", req.body());
            Ok(Response::new("Thanks for the board".into()))
        }
        ("/board", &Method::GET) => {
            let board_string = req.body().clone();
            let board = board::Board::from_str(board_string);
            let solved_board = board::solve(board);
            let serialized_solved_board = serde_json::to_string(&solved_board).unwrap();
            Ok(Response::new(serialized_solved_board.into()))
        }
        _ => Ok(Response::new("I'm alive!".into())),
    }
}

pub async fn server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let hello_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(hello_world)) });
    let hello_server = Server::bind(&addr).serve(hello_service);
    if let Err(e) = hello_server.await {
        eprintln!("server error: {}", e);
    }
}
