use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    
    // Rng を初期化する。再現性のためにシード値を指定する。
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    rng.sample_iter(&Standard).take(n).collect()

    // // n 個の要素を格納することができるようにベクタを初期化する。
    // let mut v = Vec::with_capacity(n);

    // for _ in 0..n {
    //     v.push(rng.sample(&Standard))
    // }

}