use crate::oj::{BattleResult, BattleInput};

pub fn embellish(input : &BattleInput, br : &BattleResult) -> String{
    let hp = input.hp;
    let atk = input.atk;
    let def = input.def;
    let evd = input.evd;
    let psv = input.psv;
    let buff = &input.buff;
    let hpt = input.hpt;
    let atkt = input.atkt;
    let deft = input.deft;
    let evdt = input.evdt;
    let psvt = input.psvt;
    let bufft = &input.bufft;
    
    let mut txt = String::new();
    let psvstr = if let Some(ps) = psv {format!(" ({})", ps.str())} else {format!("")};
    let psvtstr = if let Some(pst) = psvt {format!(" ({})", pst.str())} else {format!("")};
    let mut buffstr = String::new();
    for b in buff {
        buffstr += " {";
        buffstr += b.str();
        buffstr += "}";
    }
    let mut bufftstr = String::new();
    for b in bufft {
        bufftstr += " {";
        bufftstr += b.str();
        bufftstr += "}";
    }
    txt += &format!("【{}{}{}{}{}{} vs {}{}{}{}{}{}】 （Hp/Atk/Def/Evd）\n", hp, atk, def, evd, psvstr, buffstr, hpt, atkt, deft, evdt, psvtstr, bufftstr);
    txt += &format!("击杀率 : {:.2}\n", br.kill_rate);
    txt += &format!("反杀率 : {:.2}\n", br.be_kill_rate);
    txt += &format!("残余血量（双方均幸存时） : {:.1} / {:.1}\n", br.you_alive_remain_hp, br.opp_alive_remain_hp);
    txt += &format!("最终决战（10回合，胜/平/负） : {:.2} / {:.2} / {:.2}\n", br.fb_10_win, br.fb_10_draw, br.fb_10_lose);
    txt += &format!("开战有利度 : {:.2}\n", br.challenge_advantage);

    txt
}
