use crate::board;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let res = Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .body::<Body>("Hello World!".into())
        .unwrap();
    Ok(res)
}

async fn stringify_body(req: Request<Body>) -> String {
    let body = req.into_body();
    let body_bytes = hyper::body::to_bytes(body).await.unwrap().to_vec();
    let body_string = String::from_utf8(body_bytes).unwrap();
    return body_string;
}

async fn boogie_board(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.uri().path(), req.method()) {
        ("/board", &Method::POST) => {
            // let body_string = stringify_body(req).await;
            println!("Sending NYT Sudoku Board");
            let starting_board = board::Board::new(crate::consts::nyt_hard_map());
            // let solved = board::solve(nyt_easy_starting_board);
            let serialized_board = serde_json::to_string(&starting_board).unwrap();
            let res = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body::<Body>(serialized_board.into())
                .unwrap();
            Ok(res)
        }
        (_, &Method::OPTIONS) => {
            println!("Negotiating request options");
            let res = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body::<Body>("I'm Alive!".into())
                .unwrap();
            Ok(res)
        }
        ("/board/solve_all", &Method::GET) => {
            let body_string = stringify_body(req).await;
            let board = board::Board::from_str(body_string);
            let solved_board = board::solve(board);
            let serialized_solved_board = serde_json::to_string(&solved_board).unwrap();
            let res = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Content-Type")
                .body::<Body>(serialized_solved_board.into())
                .unwrap();
            Ok(res)
        }
        ("/board/solve_square", &Method::POST) => {
            let square = match req.uri().query() {
                None => None,
                Some(str) => match str.parse::<u8>() {
                    Ok(box_index) => Some(box_index),
                    Err(_) => None,
                },
            };
            let body_string = stringify_body(req).await;
            let solver = board::Solver::init_with_board(board::Board::from_str(body_string));
            let worked_board = board::work_one_box(solver, square);
            let serialized_worked_board = serde_json::to_string(&worked_board.get_board()).unwrap();
            let res = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .body::<Body>(serialized_worked_board.into())
                .unwrap();
            Ok(res)
        }
        _ => {
            let res = Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .body::<Body>("I'm Alive!".into())
                .unwrap();
            Ok(res)
        }
    }
}

pub async fn server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let hello_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(boogie_board)) });
    let hello_server = Server::bind(&addr).serve(hello_service);
    if let Err(e) = hello_server.await {
        eprintln!("server error: {}", e);
    }
}
