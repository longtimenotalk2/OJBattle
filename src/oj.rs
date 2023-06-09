use std::collections::HashMap;

pub struct BattleInput {
    pub hp : i32,
    pub atk : i32,
    pub def : i32,
    pub evd : i32,
    pub psv : Option<Passive>,
    pub buff : Vec<Buff>,
    pub hpt : i32,
    pub atkt : i32,
    pub deft : i32,
    pub evdt : i32,
    pub psvt : Option<Passive>,
    pub bufft : Vec<Buff>,
}
pub struct BattleResult {
    pub kill_rate : f32,
    pub be_kill_rate : f32,
    pub you_alive_remain_hp : f32,
    pub opp_alive_remain_hp : f32,
    pub fb_10_win : f32,
    pub fb_10_draw : f32,
    pub fb_10_lose : f32,
    pub challenge_advantage : f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Passive {
    Iru,
    Tql,
    Sherry,
    Msk,
    Repa,
}

impl Passive {
    pub fn str(&self) -> &str {
        match self {
            Passive::Iru => "Iru",
            Passive::Tql => "Tql",
            Passive::Sherry => "Sherry",
            Passive::Msk => "Msk",
            Passive::Repa => "Repa",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buff {
    Ext,
    Acc,
    AccH,
}

impl Buff {
    pub fn str(&self) -> &str {
        match self {
            Buff::Ext => "Ext",
            Buff::Acc => "Acc",
            Buff::AccH => "AccH",
        }
    }
}

pub fn main_battle (input : &BattleInput) -> BattleResult {
    let mut hp = input.hp;
    let mut atk = input.atk;
    let mut def = input.def;
    let mut evd = input.evd;
    let mut psv = input.psv;
    let mut buff = &input.buff;
    let mut hpt = input.hpt;
    let mut atkt = input.atkt;
    let mut deft = input.deft;
    let mut evdt = input.evdt;
    let mut psvt = input.psvt;
    let mut bufft = &input.bufft;

    assert!(hp > 0);
    assert!(hpt > 0);

    // Iru如是后手，取消之
    if let Some(Passive::Iru) = psvt {
        psvt = None;
    }

    let mut inverse = false;

    // 如果先手是雪莉且后手不是，则需要调转双方的属性，计算完后结果取反
    if let Some(Passive::Sherry) = psv {
        if let Some(Passive::Sherry) = psvt {}else{
            inverse = true;

            let t_hp = hp;
            let t_atk = atk;
            let t_def = def;
            let t_evd = evd;
            let t_psv = psv;
            let t_buff = buff;

            hp = hpt;
            atk = atkt;
            def = deft;
            evd = evdt;
            psv = psvt;
            buff = bufft;

            hpt = t_hp;
            atkt = t_atk;
            deft = t_def;
            evdt = t_evd;
            psvt = t_psv;
            bufft = t_buff;
        }
    }

    // 单次开战
    let once_result = battle_once(atk, hp, def, evd, psv, &buff, atkt, hpt, deft, evdt, psvt, &bufft);

    let kill_rate = (once_result.kill_rate * 100.0).round() / 100.0;
    let be_kill_rate = (once_result.be_kill_rate * 100.0).round() / 100.0;
    let you_alive_remain_hp =  (once_result.you_alive_remain_hp * 10.0).round() / 10.0;
    let opp_alive_remain_hp = (once_result.opp_alive_remain_hp * 10.0).round() / 10.0;

    // 最终决战
    let (fbwin, fblose) = fb_10(atk, hp, def, evd, psv, &buff, atkt, hpt, deft, evdt, psvt, &bufft);
    let fb_10_win = (fbwin * 100.0).round() / 100.0;
    let fb_10_lose = (fblose * 100.0).round() / 100.0;
    let fb_10_draw = (1.0 - fb_10_win - fb_10_lose).max(0.0);

    let decay = 0.5;
    let result = fb_decay(atk, hp, def, evd, psv, &buff, atkt, hpt, deft, evdt, psvt, &bufft, decay);
    let r = result.last().unwrap().last().unwrap().0;

    let challenge_advantage = (r*100.0).round()/100.0;

    if inverse {
        BattleResult{
            kill_rate : be_kill_rate,
            be_kill_rate : kill_rate,
            you_alive_remain_hp : opp_alive_remain_hp,
            opp_alive_remain_hp : you_alive_remain_hp,
            fb_10_win : fb_10_lose,
            fb_10_draw,
            fb_10_lose : fb_10_win,
            challenge_advantage : - challenge_advantage,
        }
    }else{
        BattleResult{
            kill_rate,
            be_kill_rate,
            you_alive_remain_hp,
            opp_alive_remain_hp,
            fb_10_win,
            fb_10_draw,
            fb_10_lose,
            challenge_advantage,
        }
    }
}

fn fb_10(
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    psv : Option<Passive>,
    buff : &Vec<Buff>,
    atkt : i32,
    hpt : i32,
    deft : i32,
    evdt : i32,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> (f32, f32) {
    let stat = fb_given_turn(10, atk, hp, def, evd, psv, &buff, atkt, hpt, deft, evdt, psvt, &bufft);

        stat.last().unwrap().last().unwrap().clone()
}

fn fb_given_turn(
    turn : i32,
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    mut psv : Option<Passive>,
    buff : &Vec<Buff>,
    mut atkt : i32,
    mut hpt : i32,
    deft : i32,
    evdt : i32,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> Vec<Vec<(f32, f32)>> {

    // 最终决战Iru只能射一次，改成减对面一血后取消之
    if let Some(Passive::Iru) = psv {
        hpt -= 1;
        psv = None;
        // 如果此时后手刚好是船长，则直接+1攻
        if let Some(Passive::Tql) = psvt {
            atkt += 1;
        }
    }
    
    // init
    let mut stat : Vec<Vec<(f32, f32)>> = vec!();
    for h in 0..(hp+1) {
        let mut newvec = vec!();
        for ht in 0..(hpt+1) {
            let left = if ht == 0 {
                1.0
            }else{
                0.0
            };
            let right = if h == 0 {
                1.0
            }else{
                0.0
            };
            newvec.push((left, right));
        }
        stat.push(newvec);
    }

    for _ in 0..turn {
        let mut stat_next = stat.clone();
        // opp hit you
        for h in 1..hp+1 {
            let ih : usize = h.try_into().unwrap();
            for ht in 1..hpt+1 {
                let iht : usize = ht.try_into().unwrap();
                let mut atktp = atkt;

                // 船长会根据已损失血量加攻
                if let Some(Passive::Tql) = psvt {
                    atktp += hpt - ht;
                }
                let hp_dist = onceatk(atktp, h, def, evd, psvt, bufft, psv, buff);
                let mut result = (0.0, 0.0);
                for (hh, r) in hp_dist.data.iter().enumerate() {
                    // 如果自己是Repa且没掉血
                    let mut repa = false;
                    if let Some(Passive::Repa) = psv {
                        if hh == ih {
                            repa = true;
                        }
                    }
                    if repa {
                        let ihtr : usize = (ht - 1).try_into().unwrap();
                        result.0 += stat[hh][ihtr].0 * r;
                        result.1 += stat[hh][ihtr].1 * r;
                    }else{
                        result.0 += stat[hh][iht].0 * r;
                        result.1 += stat[hh][iht].1 * r;
                    }
                }
                stat_next[ih][iht] = result;
            }
        }
        stat = stat_next.clone();

        // you hit opp
        for h in 1..hp+1 {
            let ih : usize = h.try_into().unwrap();
            for ht in 1..hpt+1 {
                let iht : usize = ht.try_into().unwrap();
                let mut atkp = atk;

                // 船长会根据已损失血量加攻
                if let Some(Passive::Tql) = psv {
                    atkp += hp - h;
                }
                let hp_distt = onceatk(atkp, ht, deft, evdt, psv, buff, psvt, bufft);
                let mut result = (0.0, 0.0);
                for (hht, r) in hp_distt.data.iter().enumerate() {
                    // 如果自己是Repa且没掉血
                    let mut repa = false;
                    if let Some(Passive::Repa) = psvt {
                        if hht == iht {
                            repa = true;
                        }
                    }
                    if repa {
                        let ihr : usize = (h - 1).try_into().unwrap();
                        result.0 += stat[ihr][hht].0 * r;
                        result.1 += stat[ihr][hht].1 * r;
                    }else{
                        result.0 += stat[ih][hht].0 * r;
                        result.1 += stat[ih][hht].1 * r;
                    }
                }
                stat_next[ih][iht] = result;
            }
        }

        stat = stat_next
    }

stat

}



struct ButtleOnceInfo {
    kill_rate : f32,
    be_kill_rate : f32,
    you_alive_remain_hp : f32,
    opp_alive_remain_hp : f32,
}

fn battle_once(
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    psv : Option<Passive>,
    buff : &Vec<Buff>,
    atkt : i32,
    hpt : i32,
    deft : i32,
    evdt : i32,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> ButtleOnceInfo {

    let hp_distt = onceatk(atk, hpt, deft, evdt, psv, buff, psvt, bufft);
    // 击杀率（反击前）
    let mut kill = *hp_distt.data.get(0).unwrap();
    // 对方幸存余血（反击前）
    let mut remain_hpt = 0. ;
    if (kill * 100.).round() as i32 != 100 {
        for (hpt, r) in hp_distt.data.iter().enumerate() {
            remain_hpt += (hpt as f32) * r;
        }
        remain_hpt /= 1. - kill;
    }

    let mut hp_dist = onceatk(atkt, hp, def, evd, psvt, bufft, psv, buff);

    // 船长
    if let Some(Passive::Tql) = psvt {
        if (kill * 100.).round() as i32 != 100 {
            hp_dist.data = vec![0.0];
            for (ht, r) in hp_distt.data.iter().enumerate() {
                if ht != 0 {
                    hp_dist += onceatk(atkt + hpt - (ht as i32), hp, def, evd, psvt, bufft, psv, buff) * *r;
                }
            }
            hp_dist = hp_dist * (1. / (1. - kill));
        }
    }

    // Repa 

    // （反击发生后）攻击方闪避成功率
    let mut you_miss = hp_dist.get(hp).unwrap_or(0.);
    
    if let Some(Passive::Repa) = psvt {
        let repa_dist = onceatk(atkt, hp-1, def, evd, psvt, bufft, psv, buff);
        
        let ihpt : usize = hpt.try_into().unwrap();
        // （反击方存活）闪避率
        let mut miss = 0.;
        if (kill * 100.).round() as i32 != 100 {
            miss = *hp_distt.data.get(ihpt).unwrap_or(&0.) / (1. - kill);
        }
        you_miss *= 1. - miss;
        if hp != 1 {
            you_miss += repa_dist.get(hp-1).unwrap_or(0.) * miss;
        }
        hp_dist = hp_dist * (1. - miss);
        hp_dist += repa_dist * miss;
    }

    // （反击发生后）反杀率
    let be_kill: f32 = *hp_dist.data.get(0).unwrap();
    // (绝对)反杀率
    let be_kill_abs = be_kill * (1. - kill);

    // （反击发生后）我方幸存余血
    let mut remain_hp = 0.;
    // （绝对）我方幸存余血
    let mut remain_hp_abs = 0.;
    if (be_kill * 100.).round() as i32 != 100 {
        for (hp, r) in hp_dist.data.iter().enumerate() {
            remain_hp += (hp as f32) * r;
        }
        remain_hp /= 1. - be_kill;
    }

    if ((kill + (1. - kill) * (1. - be_kill)) * 100.0).round() as i32 != 0 {
        remain_hp_abs = (hp as f32 * kill + remain_hp * (1. - kill) * (1. - be_kill)) / (kill + (1. - kill) * (1. - be_kill));
    }



    // Repa最终补刀
    if let Some(Passive::Repa) = psv {
        // （绝对）攻击方闪避率
        let repa_rate = (1. - kill) * you_miss;
        // 击杀率（repa加成）
        if (kill * 100.).round() as i32 != 100 {
            kill += hp_distt.get(1).unwrap_or(0.) * repa_rate / (1. - kill);
        }
        // （repa触发后）对方幸存余血
        let mut remain_hpt_repa = 0. ;
        let kill_repa = hp_distt.get(0).unwrap_or(0.) + hp_distt.get(1).unwrap_or(0.);

        if (kill_repa * 100.).round() as i32 != 100 {
            for (hpt, r) in hp_distt.enumerate() {
                if hpt > 1 {
                    remain_hpt_repa += (hpt - 1) as f32 * r;
                }
            }
            remain_hpt_repa /= 1. - kill_repa;
        }

        // （绝对）对方残余血量
        if (repa_rate * kill_repa * 100.).round() as i32 != 100 {
            remain_hpt = (remain_hpt * (1. - repa_rate) + remain_hpt_repa * repa_rate * (1. - kill_repa)) / (1. - repa_rate * kill_repa);
        } else {
            remain_hpt = 0.;
        }
    }

    // （绝对）击杀率
    // （绝对）反杀率
    // （我方存活）我方残余血量
    // （敌方存活）敌方残余血量
    
    ButtleOnceInfo{
        kill_rate: kill,
        be_kill_rate: be_kill_abs,
        you_alive_remain_hp: remain_hp_abs,
        opp_alive_remain_hp: remain_hpt,
    }
}

fn fb_decay(
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    psv : Option<Passive>,
    buff : &Vec<Buff>,
    atkt : i32,
    hpt : i32,
    deft : i32,
    evdt : i32,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
    decay : f32,
) -> Vec<Vec<(f32, f32)>> {

    
    let mut result : Vec<Vec<(f32, f32)>> = vec!();

    // 船长专用数据
    let mut tql : Vec<Vec<Vec<f32>>> = vec!();
    
    for h in 0..(hp+1) {
        let ih : usize = h.try_into().unwrap();
        result.push(vec!());
        tql.push(vec!());
        for ht in 0..(hpt+1) {
            tql[ih].push(vec!());
            let iht : usize = ht.try_into().unwrap();
            if h * ht == 0 {
                if h == 0 {
                    result[ih].push((-2.0, -1.0));
                }else{
                    result[ih].push((2.0, 1.0));
                }
                
                // 船长专用数据
                if let Some(Passive::Tql) = psvt {
                    for _ in 0..hpt {
                        tql[ih][iht].push(1.0);
                    }
                }
            }else{
                let distt = onceatk(atk, ht, deft, evdt, psv, buff, psvt, bufft);
                let dist = onceatk(atkt, h, def, evd, psvt, bufft, psv, buff); 
                let mut sumright = 0.0;
                let mut cycler = 0.0;
                for (hht, r) in distt.data.iter().enumerate() {
                    if hht == iht {
                        // Repa 
                        if let Some(Passive::Repa) = psvt {
                            let ind : usize = (h - 1).try_into().unwrap();
                            sumright += r * result[ind][hht].1;} else {
                        cycler = *r;
                        }
                    }else{
                        let mut adder = r * result[ih][hht].1;
                        // 船长映射到船长数据
                        if let Some(Passive::Tql) = psvt {
                            let index : usize = iht - hht - 1;
                            adder = r * tql[ih][hht][index];
                        }
                        sumright += adder;
                    }
                }
                let mut sumleft = 0.0;
                let mut cyclel = 0.0;
                for (hh, r) in dist.data.iter().enumerate() {
                    if hh == ih {
                        // Repa 
                        if let Some(Passive::Repa) = psv {
                            let ind : usize = (ht - 1).try_into().unwrap();
                            sumleft += r * result[hh][ind].0;
                        }else{
                            cyclel = *r;
                        }
                    }else{
                        sumleft += r * result[hh][iht].0;
                    }
                }

                // 船长多种sumleft可能性
                let mut tql_c = vec!();
                let mut tql_sl = vec!();
                for aa in 1..(hpt - ht + 1) {
                    let dist = onceatk(atkt + aa, h, def, evd, psvt, bufft, psv, buff); 
                    let mut c = 0.0;
                    let mut sumleft = 0.0;
                    for (hh, r) in dist.data.iter().enumerate() {
                        if hh == ih {
                            c = *r;
                        }else{
                            sumleft += r * result[hh][iht].0;
                        }
                    }
                    tql_c.push(c);
                        tql_sl.push(sumleft);
                }
                
                // left = sumright + cycler*right
                // right = decay * (sumleft + cyclel*left)
                let left = (sumright + cycler * decay * sumleft) / (1.0 - cycler * decay * cyclel);
                let right = decay * (sumleft + cyclel * left);
                result[ih].push((left, right));

                // 船长多种right可能性
                for i in 0..tql_sl.len() {
                    let right = decay * (tql_sl[i] + tql_c[i] * left);
                    tql[ih][iht].push(right);
                }
            }
        }
    }
    result
}


fn onceatk(
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    psv : Option<Passive>,
    buff : &Vec<Buff>,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> HpDist {
    let mut result = HpDist::new();
    let mut roll = Roll::new();
    if buff.contains(&Buff::Ext) {
        roll.set_dice(Dice::F6);
    }
    if buff.contains(&Buff::Acc) {
        roll.double_dice();
    }
    if buff.contains(&Buff::AccH) {
        roll.double_dice();
    }
    for (point, r) in roll.get_dist() {
        let a = 1.max(atk + point);
        result += oncebeatk(a, hp, def, evd, psv, buff, psvt, bufft) * r;
    }
    result
}
    
fn oncebeatk(
    a : i32,
    hp : i32,
    def : i32,
    evd : i32,
    psv : Option<Passive>,
    buff : &Vec<Buff>,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> HpDist {
    let dresult = oncedef(a, hp, def, psv, buff, psvt, bufft);
    let eresult = onceevd(a, hp, evd, psv, buff, psvt, bufft);
    if dresult.alive_rate() > eresult.alive_rate() {
        dresult
    }else if dresult.alive_rate() < eresult.alive_rate() {
        eresult
    }else{
        if dresult.expect() >= eresult.expect() {
            dresult
        }else{
            eresult
        }
    }
}

fn oncedef(
    a : i32,
    hp : i32,
    def : i32,
    psv : Option<Passive>,
    _buff : &Vec<Buff>,
    psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> HpDist {
    let mut result = HpDist::new();
    let mut roll = Roll::new();
    if bufft.contains(&Buff::Ext) {
        roll.set_dice(Dice::F6);
    }
    if bufft.contains(&Buff::Acc) {
        roll.double_dice();
    }
    for (point, r) in roll.get_dist() {
        let d = 1.max(def + point);
        let mut dmg = 1.max(a - d);

        if let Some(Passive::Iru) = psv {
            dmg += 1;
        }

        // Msk防御时至多扣2血
        if let Some(Passive::Msk) = psvt {
            dmg = dmg.min(2);
        }
        
        let hp_remain = 0.max(hp - dmg);
        result.insert(hp_remain, r);
    }
    result
}

fn onceevd(
    a : i32,
    hp : i32,
    evd : i32,
    psv : Option<Passive>,
    _buff : &Vec<Buff>,
    _psvt : Option<Passive>,
    bufft : &Vec<Buff>,
) -> HpDist {
    let mut result = HpDist::new();
    let mut roll = Roll::new();
    if bufft.contains(&Buff::Ext) {
        roll.set_dice(Dice::F6);
    }
    if bufft.contains(&Buff::Acc) {
        roll.double_dice();
    }
    for (point, r) in roll.get_dist() {
        let e = 1.max(evd + point);
        let mut hp_remain = hp;
        let mut dmg = if e <= a {a} else {0};

        if let Some(Passive::Iru) = psv {
            dmg += 1;
        }
        
        hp_remain = 0.max(hp_remain - dmg);
        result.insert(hp_remain, r);
    }
    result
}

#[derive(Debug, Clone, Copy)]
enum Dice {
    D6,
    F6,
}

struct Roll {
    dice : Dice,
    num : i32,
}

impl Roll {
    fn new() -> Self {
        Self {
            dice : Dice::D6,
            num : 1,
        }
    }

    fn set_dice(&mut self, dice : Dice) {
        self.dice = dice;
    }

    fn double_dice(&mut self) {
        self.num *= 2;
    }

    fn get_dist(&self) -> HashMap<i32, f32> {
        let mut r = HashMap::new();
        match self.dice {
            Dice::D6 => {
                for i in 1..7 {
                    r.insert(i, 1.0/6.0);
                }
            },
            Dice::F6 => {
                r.insert(6, 1.);
            },
        }

        if self.num > 1 {
            let mut result = r.clone();
            for _ in 1..self.num {
                let mut rnew : HashMap<i32, f32> = HashMap::new();
                for (p1, r1) in result {
                    for (p2, r2) in &r {
                        let key = p1+p2;
                        let v = r1*r2;
                        match rnew.get_mut(&key) {
                            None => {
                                rnew.insert(key, v);
                            },
                            Some(vv) => {
                                *vv += v;
                            },
                        }
                    }
                }
            result = rnew;
            }

        result
        } else {
            r
        }
    }
}


#[derive(Clone, Debug)]
struct HpDist {
    data : Vec<f32>,
}

impl HpDist {
    fn expect(&self) -> f32 {
        let mut result = 0.0;
        for (i, r) in self.data.iter().enumerate() {
            result += (i as f32) * r;
        }
        (result * 100.0).round() / 100.0
    }
    
    fn alive_rate(&self) -> f32 {
        let a = 1. - self.data.get(0).unwrap();
        (a * 100.0).round() / 100.0
    }
    
    fn new() -> Self {
        Self {data : vec!(0.)}
    }

    fn insert(&mut self, hp : i32, rate : f32) {
        let len = self.data.len() as i32;
        let diff = hp + 1 - len;
        if diff > 0 {
            for _ in 0..diff {
                self.data.push(0.)
            }
        }
        let i : usize = hp.try_into().unwrap();
        *self.data.get_mut(i).unwrap() += rate;
    }

    fn get(&self, hp:i32) -> Option<f32> {
        let i : usize = hp.try_into().unwrap();
        self.data.get(i).map(|x| *x)
    }

    fn enumerate(&self) -> Vec<(i32, f32)> {
        let mut result = vec!();
        for (i, r) in self.data.iter().enumerate() {
            result.push((i as i32, *r));
        }
        result
    }

    // fn show(&self) {
    //     for (i, r) in self.data.iter().enumerate() {
    //         let num = (r * 100.).round() / 100.0;
    //         println!("{} : {:.2}", i, num);
    //     }
    // }
}

impl std::ops::Mul<f32> for HpDist {
    type Output = HpDist;
    fn mul(mut self, other: f32) -> HpDist {
        for i in 0..self.data.len() {
            *self.data.get_mut(i).unwrap() *= other
        }
        self
    }
}

impl std::ops::AddAssign<HpDist> for HpDist {
    fn add_assign(&mut self, other : HpDist) {
        for (i, r) in other.data.iter().enumerate() {
            self.insert(i as i32, *r);
        }
    }
}

