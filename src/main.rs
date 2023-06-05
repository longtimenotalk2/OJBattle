pub mod art;
pub mod test;
pub mod web;
/**
*   test instance in http://127.0.0.1:8080/battle?hp=4&atk=0&def=0&evd=1&psv=Repa&hpt=4&atkt=1&deft=-2&evdt=0&psvt=Msk
*   api in http://127.0.0.1:8080/apis/battle?hp=4&atk=0&def=0&evd=1&psv=Repa&hpt=4&atkt=1&deft=-2&evdt=0&psvt=Msk
*   optional with &format=json
**/
pub mod oj;
#[allow(unused_imports)]
use crate::oj::{main_battle, BattleInput, Buff, Passive};

// use test::test_main;
use web::web_main;

#[allow(warnings)]
fn main() {
    // test_main();
    web_main("0.0.0.0");
    // web_main("127.0.0.1");
}


