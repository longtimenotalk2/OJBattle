use crate::{oj::{Passive, BattleInput, main_battle}, art::embellish};

#[allow(warnings)]
pub fn test_main() {
    let mut psv = None;
    let mut psvt = None;
    let mut buff = vec![];
    let mut bufft = vec![];
    let mut psv2 = None;
    let mut psvt2 = None;
    let mut buff2 = vec![];
    let mut bufft2 = vec![];

    let hp = 4;
    let atk = 0;
    let def = 0;
    let evd = 1;
    let hpt = 4;
    let atkt = 1;
    let deft = -2;
    let evdt = 0;

    psv = Some(Passive::Repa);
    psvt = Some(Passive::Msk);
    // buff = vec!(Buff::Acc);
    // buff = vec!(Buff::Acc, Buff::AccH);
    // bufft = vec!(Buff::Acc);

    // psv2 = Some(Passive::Iru);
    psvt2 = Some(Passive::Msk);
    // buff2 = vec!(Buff::Ext);
    // bufft2 = vec!(Buff::Ext);

    let input = BattleInput {
        hp,
        atk,
        def,
        evd,
        psv,
        buff,
        hpt,
        atkt,
        deft,
        evdt,
        psvt,
        bufft,
    };
    let br = main_battle(&input);
    println!("{}", embellish(&input, &br));

    let input = BattleInput {
        hp,
        atk,
        def,
        evd,
        psv: psv2,
        buff: buff2,
        hpt,
        atkt,
        deft,
        evdt,
        psvt: psvt2,
        bufft: bufft2,
    };
    // let input = BattleInput { hp : hpt, atk : atkt, def : deft, evd : evdt, psv : psvt2, buff : bufft2, hpt : hp, atkt : atk, deft : def, evdt : evd, psvt : psv2, bufft : buff2 };
    let br = main_battle(&input);
    println!("{}", embellish(&input, &br));
}