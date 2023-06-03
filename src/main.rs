
/**
*   test instance in http://127.0.0.1:8080/battle?hp=4&atk=1&def=1&evd=-1&hpt=4&atkt=0&deft=0&evdt=1
*   api in http://127.0.0.1:8080/apis/battle?hp=4&atk=1&def=1&evd=-1&hpt=4&atkt=0&deft=0&evdt=1
*   optional with &format=json
**/


pub mod oj;
pub mod art;
use crate::oj::{main_battle, Buff, BattleInput};
use oj::Passive;

use art::embellish;

fn main() {
    let psv = None;
    // let psv = Some(Passive::Tql);
    // let psvt = None;
    let psvt = Some(Passive::Tql);

    // let buff = vec!();
    let buff = vec!(Buff::Ext);
    let bufft = vec!();
    // let bufft = vec!(Buff::Ext);

    let hp = 4;
    let atk = 1;
    let def = 0;
    let evd = 1;
    let hpt = 5;
    let atkt = 0;
    let deft = 1;
    let evdt = -3;

    let input = BattleInput { hp, atk, def, evd, psv, buff, hpt, atkt, deft, evdt, psvt, bufft};
    let br = main_battle(&input);
    println!("{}", embellish(&input, &br));

    let psv = None;
    // let psv = Some(Passive::Tql);
    let psvt = None;
    // let psvt = Some(Passive::Tql);

    // let buff = vec!();
    let buff = vec!(Buff::Ext);
    let bufft = vec!();
    // let bufft = vec!(Buff::Ext);

    let input = BattleInput { hp, atk, def, evd, psv, buff, hpt, atkt, deft, evdt, psvt, bufft};
    let br = main_battle(&input);
    println!("{}", embellish(&input, &br));

}


