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

pub fn main_battle(
    hp : i32,
    atk : i32,
    def : i32,
    evd : i32,
    hpt : i32,
    atkt : i32,
    deft : i32,
    evdt : i32,
)  -> BattleResult {

    // 单次开战
    let once_result = battle_once(atk, hp, def, evd, atkt, hpt, deft, evdt);

    let kill_rate = (once_result.kill_rate * 100.0).round() / 100.0;
    let be_kill_rate = (once_result.be_kill_rate * 100.0).round() / 100.0;
    let you_alive_remain_hp =  (once_result.you_alive_remain_hp * 10.0).round() / 10.0;
    let opp_alive_remain_hp = (once_result.opp_alive_remain_hp * 10.0).round() / 10.0;

    // 最终决战
    let (fbwin, fblose) = fb_10(atk, hp, def, evd, atkt, hpt, deft, evdt);
    let fb_10_win = (fbwin * 100.0).round() / 100.0;
    let fb_10_lose = (fblose * 100.0).round() / 100.0;
    let fb_10_draw = (1.0 - fb_10_win - fb_10_lose).max(0.0);

    let decay = 0.5;
    let result = fb_decay(atk, hp, def, evd, atkt, hpt, deft, evdt, decay);
    let r = result.last().unwrap().last().unwrap().0;

    let challenge_advantage = (r*100.0).round()/100.0;

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

fn fb_10(
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    atkt : i32,
    hpt : i32,
    deft : i32,
    evdt : i32,
) -> (f32, f32) {
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

    for _ in 0..10 {
        let mut stat_next = stat.clone();
        // opp hit you
        for h in 1..hp+1 {
            let ih : usize = h.try_into().unwrap();
            for ht in 1..hpt+1 {
                let iht : usize = ht.try_into().unwrap();
                let hp_dist = onceatk(atkt, h, def, evd);
                let mut result = (0.0, 0.0);
                for (hh, r) in hp_dist.data.iter().enumerate() {
                    result.0 += stat[hh][iht].0 * r;
                    result.1 += stat[hh][iht].1 * r;
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
                let hp_distt = onceatk(atk, ht, deft, evdt);
                let mut result = (0.0, 0.0);
                for (hht, r) in hp_distt.data.iter().enumerate() {
                    result.0 += stat[ih][hht].0 * r;
                    result.1 += stat[ih][hht].1 * r;
                }
                stat_next[ih][iht] = result;
            }
        }

        stat = stat_next
    }

    
    stat.last().unwrap().last().unwrap().clone()
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
    atkt : i32,
    hpt : i32,
    deft : i32,
    evdt : i32,
) -> ButtleOnceInfo {
    let hp_distt = onceatk(atk, hpt, deft, evdt);
    let kill = *hp_distt.data.get(0).unwrap();
    let mut remain_hpt = 0. ;
    if (kill * 100.).round() as i32 != 100 {
        for (hpt, r) in hp_distt.data.iter().enumerate() {
            remain_hpt += (hpt as f32) * r;
        }
        remain_hpt /= 1. - kill;
    }

    let hp_dist = onceatk(atkt, hp, def, evd);
    let be_kill: f32 = *hp_dist.data.get(0).unwrap();
    let mut remain_hp = 0. ;
    if (be_kill * 100.).round() as i32 != 100 {
        for (hp, r) in hp_dist.data.iter().enumerate() {
            remain_hp += (hp as f32) * r;
        }
        remain_hp /= 1. - be_kill;
    }
    ButtleOnceInfo{
        kill_rate: kill,
        be_kill_rate: be_kill * (1.0 - kill),
        you_alive_remain_hp: remain_hp,
        opp_alive_remain_hp: remain_hpt,
    }
}

fn fb_decay(
    atk : i32,
    hp : i32,
    def : i32,
    evd : i32,
    atkt : i32,
    hpt : i32,
    deft : i32,
    evdt : i32,
    decay : f32,
) -> Vec<Vec<(f32, f32)>> {
    let mut result : Vec<Vec<(f32, f32)>> = vec!();
    for h in 0..(hp+1) {
        let ih : usize = h.try_into().unwrap();
        result.push(vec!());
        for ht in 0..(hpt+1) {
            let iht : usize = ht.try_into().unwrap();
            if h * ht == 0 {
                result[ih].push((-2.0, 1.0));
            }else{
                let distt = onceatk(atk, ht, deft, evdt);
                let dist = onceatk(atkt, h, def, evd);
                let mut sumright = 0.0;
                let mut cycler = 0.0;
                for (hht, r) in distt.data.iter().enumerate() {
                    if hht == iht {
                        cycler = *r;
                    }else{
                        sumright += r * result[ih][hht].1;
                    }
                }
                let mut sumleft = 0.0;
                let mut cyclel = 0.0;
                for (hh, r) in dist.data.iter().enumerate() {
                    if hh == ih {
                        cyclel = *r;
                    }else{
                        sumleft += r * result[hh][iht].0;
                    }
                }
                // left = sumright + cycler*right; right = decay * (sumleft + cyclel*left)
                let left = (sumright + cycler * decay * sumleft) / (1.0 - cycler * decay * cyclel);
                let right = decay * (sumleft + cyclel * left);
                result[ih].push((left, right));
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
) -> HpDist {
    let mut result = HpDist::new();
    for dice in 1..7 {
        let a = 1.max(atk + dice);
        result += oncebeatk(a, hp, def, evd) * (1.0/6.0);
    }
    result
}
    
fn oncebeatk(
    a : i32,
    hp : i32,
    def : i32,
    evd : i32,
) -> HpDist {
    let dresult = oncedef(a, hp, def);
    let eresult = onceevd(a, hp, evd);
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
) -> HpDist {
    let mut result = HpDist::new();
    for dice in 1..7 {
        let d = 1.max(def + dice);
        let dmg = 1.max(a - d);
        let hp_remain = 0.max(hp - dmg);
        result.insert(hp_remain, 1.0/6.0);
    }
    result
}

fn onceevd(
    a : i32,
    hp : i32,
    evd : i32,
) -> HpDist {
    let mut result = HpDist::new();
    for dice in 1..7 {
        let e = 1.max(evd + dice);
        let mut hp_remain = hp;
        if e <= a {
            hp_remain = 0.max(hp_remain - a);
        }
        result.insert(hp_remain, 1.0/6.0);
    }
    result
}

#[derive(Clone)]
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


