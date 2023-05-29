//git push -u origin main


//cargo run -- -p [port]
//http://127.0.0.1:[port]/battle/4/1/1/-1/4/0/0/1



pub mod oj;
use crate::oj::main_battle;

// fn main() {
//     println!("{}", main_battle(
//         5,
//         -1,
//         -1,
//         2,
//         5,
//         -1,
//         -1,
//         2,
//     ))
// }


use std::env;
use actix_web::{get, web, Result};
use serde::Deserialize;



#[derive(Deserialize)]
struct Info {
    hp : i32,
    atk : i32,
    def : i32,
    evd : i32,
    hpt : i32,
    atkt : i32,
    deft : i32,
    evdt : i32,
}

/// extract path info using serde
#[get("/battle/{hp}/{atk}/{def}/{evd}/{hpt}/{atkt}/{deft}/{evdt}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    let txt = main_battle(info.hp, info.atk, info.def, info.evd, info.hpt, info.atkt, info.deft, info.evdt);
    Ok(txt)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut port_str = String::from("8080");
    for i in 1..args.len() {
        if args[i] == "-p" {
            port_str = String::from(&args[i + 1]);
        }
    }
    let port : u16 = port_str.parse().unwrap();
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
