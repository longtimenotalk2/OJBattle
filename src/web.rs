// ====== For service =========

use actix_web::HttpResponse;
use actix_web::{get, web, Result};
use serde::Deserialize;
use serde::Serialize;
use clap::Parser;

use crate::art::embellish;
use crate::oj::{Passive, main_battle, BattleInput};

#[derive(Deserialize, Clone)]
pub enum PassiveWeb {
    Iru,
    Tql,
    Sherry,
    Msk,
    Repa,
}

impl PassiveWeb {
    fn to(self) -> Passive {
        match self {
            PassiveWeb::Iru => Passive::Iru,
            PassiveWeb::Tql => Passive::Tql,
            PassiveWeb::Sherry => Passive::Sherry,
            PassiveWeb::Msk => Passive::Msk,
            PassiveWeb::Repa => Passive::Repa,
        }
    }
}

#[derive(Deserialize)]
struct Info {
    hp : i32,
    atk : i32,
    def : i32,
    evd : i32,
    psv : Option<PassiveWeb>,
    hpt : i32,
    atkt : i32,
    deft : i32,
    evdt : i32,
    psvt : Option<PassiveWeb>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BattleResponse { 
    pub be_kill_rate : f32,
    pub you_alive_remain_hp : f32,
    pub opp_alive_remain_hp : f32,
    pub fb_10_win : f32,
    pub fb_10_draw : f32,
    pub fb_10_lose : f32,
    pub challenge_advantage : f32,
}


#[get("/battle")]
async fn index(info: web::Query<Info>) -> Result<String> {
    let input = BattleInput {
        hp : info.hp,
        atk : info.atk,
        def : info.def,
        evd : info.evd,
        psv : info.psv.clone().map(|x| x.to()),
        buff: vec!(),
        hpt : info.hpt,
        atkt : info.atkt,
        deft : info.deft,
        evdt : info.evdt,
        psvt: info.psvt.clone().map(|x| x.to()),
        bufft: vec!(),
    };
    let br = main_battle(&input);
    let txt = embellish(&input, &br);
    Ok(txt)
}

#[get("/apis/battle")]
async fn api(info: web::Query<Info>) -> Result<HttpResponse> {
    let input = BattleInput {
        hp : info.hp,
        atk : info.atk,
        def : info.def,
        evd : info.evd,
        psv : info.psv.clone().map(|x| x.to()),
        buff: vec!(),
        hpt : info.hpt,
        atkt : info.atkt,
        deft : info.deft,
        evdt : info.evdt,
        psvt: info.psvt.clone().map(|x| x.to()),
        bufft: vec!(),
    };
    let result = main_battle(&input);
    let response = BattleResponse {
        be_kill_rate: result.be_kill_rate,
        you_alive_remain_hp: result.you_alive_remain_hp,
        opp_alive_remain_hp: result.opp_alive_remain_hp,
        fb_10_win: result.fb_10_win,
        fb_10_draw: result.fb_10_draw,
        fb_10_lose: result.fb_10_lose,
        challenge_advantage: result.challenge_advantage,
    };
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Parser)]
struct Opts {
    #[arg(short = 'p', long)]
    port: Option<u16>,
}


#[actix_web::main]
pub async fn web_main(default : &str) -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let port : u16 = opts.port.unwrap_or(8080);
    use actix_web::{App, HttpServer};

    println!("listening on : {default}:{port}");

    HttpServer::new(|| {
        App::new()
        .service(index)
        .service(api)
    })
        .bind((default, port))?
        .run()
        .await
}