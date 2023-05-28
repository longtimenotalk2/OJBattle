//http://127.0.0.1:8080/battle/4/1/1/-1/4/0/0/1

pub mod oj;

use actix_web::{get, web, Result};
use serde::Deserialize;

use crate::oj::main_battle;

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
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}