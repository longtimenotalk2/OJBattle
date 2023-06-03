// ====== For service =========

// use actix_web::HttpResponse;
// use actix_web::{get, web, Result};
// use serde::Deserialize;
// use serde::Serialize;
// use clap::Parser;

// #[derive(Deserialize)]
// struct Info {
//     hp : i32,
//     atk : i32,
//     def : i32,
//     evd : i32,
//     psv : Option<Passive>,
//     hpt : i32,
//     atkt : i32,
//     deft : i32,
//     evdt : i32,
//     psvt : Option<Passive>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct BattleResponse { 
//     pub be_kill_rate : f32,
//     pub you_alive_remain_hp : f32,
//     pub opp_alive_remain_hp : f32,
//     pub fb_10_win : f32,
//     pub fb_10_draw : f32,
//     pub fb_10_lose : f32,
//     pub challenge_advantage : f32,
// }


// #[get("/battle")]
// async fn index(info: web::Query<Info>) -> Result<String> {
//     let mut txt = String::new();
//     let br = main_battle(info.hp, info.atk, info.def, info.evd, info.psv, info.hpt, info.atkt, info.deft, info.evdt, info.psvt);
//     let psvstr = if let Some(ps) = info.psv {format!(" ({})", ps.str())} else {format!("")};
//     let psvtstr = if let Some(pst) = info.psvt {format!(" ({})", pst.str())} else {format!("")};
//     txt += &format!("【{}{}{}{}{} vs {}{}{}{}{}】 （Hp/Atk/Def/Evd）\n", info.hp, info.atk, info.def, info.evd, psvstr, info.hpt, info.atkt, info.deft, info.evdt, psvtstr);
//     txt += &format!("击杀率 : {:.2}\n", br.kill_rate);
//     txt += &format!("反杀率 : {:.2}\n", br.be_kill_rate);
//     txt += &format!("残余血量（双方均幸存时） : {:.1} / {:.1}\n", br.you_alive_remain_hp, br.opp_alive_remain_hp);
//     txt += &format!("最终决战（10回合，胜/平/负） : {:.2} / {:.2} / {:.2}\n", br.fb_10_win, br.fb_10_draw, br.fb_10_lose);
//     txt += &format!("开战有利度 : {:.2}\n", br.challenge_advantage);
//     Ok(txt)
// }

// #[get("/apis/battle")]
// async fn api(info: web::Query<Info>) -> Result<HttpResponse> {
//     let result = main_battle(info.hp, info.atk, info.def, info.evd, info.psv, info.hpt, info.atkt, info.deft, info.evdt, info.psvt);
//     let response = BattleResponse {
//         be_kill_rate: result.be_kill_rate,
//         you_alive_remain_hp: result.you_alive_remain_hp,
//         opp_alive_remain_hp: result.opp_alive_remain_hp,
//         fb_10_win: result.fb_10_win,
//         fb_10_draw: result.fb_10_draw,
//         fb_10_lose: result.fb_10_lose,
//         challenge_advantage: result.challenge_advantage,
//     };
//     Ok(HttpResponse::Ok().json(response))
// }

// #[derive(Parser)]
// struct Opts {
//     #[arg(short = 'p', long)]
//     port: Option<u16>,
// }


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let opts: Opts = Opts::parse();
//     let port : u16 = opts.port.unwrap_or(8080);
//     use actix_web::{App, HttpServer};

//     println!("listening on : 0.0.0.0:{port}");

//     HttpServer::new(|| {
//         App::new()
//         .service(index)
//         .service(api)
//     })
//         .bind(("0.0.0.0", port))?
//         .run()
//         .await
// }