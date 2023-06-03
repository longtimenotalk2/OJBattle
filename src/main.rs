
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

#[allow(warnings)]
fn main() {

    let mut psv = None;
    let mut psvt = None;
    let mut buff = vec!();
    let mut bufft = vec!();
    let mut psv2 = None;
    let mut psvt2 = None;
    let mut buff2 = vec!();
    let mut bufft2 = vec!();

    let hp = 4;
    let atk = 1;
    let def = 0;
    let evd = 1;
    let hpt = 5;
    let atkt = 0;
    let deft = 1;
    let evdt = -3;

    psv = Some(Passive::Iru);
    psvt = Some(Passive::Tql);
    // buff = vec!(Buff::Ext);
    // bufft = vec!(Buff::Ext);

    // psv2 = Some(Passive::Iru);
    // psvt2 = Some(Passive::Tql);
    // buff2 = vec!(Buff::Ext);
    // bufft2 = vec!(Buff::Ext);

    let input = BattleInput { hp, atk, def, evd, psv, buff, hpt, atkt, deft, evdt, psvt, bufft};
    let br = main_battle(&input);
    println!("{}", embellish(&input, &br));
    let input = BattleInput { hp, atk, def, evd, psv: psv2, buff :buff2, hpt, atkt, deft, evdt, psvt: psvt2, bufft: bufft2};
    let br = main_battle(&input);
    println!("{}", embellish(&input, &br));

}


