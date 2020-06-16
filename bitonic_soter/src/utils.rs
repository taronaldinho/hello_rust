use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;
use super::SortOrder;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    
    // Rng を初期化する。再現性のためにシード値を指定する。
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    // イテレータチェイン (複数のイテレータ操作からコレクションを生成する) を使って n 個の乱数を作成する。
    rng.sample_iter(&Standard).take(n).collect()

    // 上記のイテレータチェインを使わない場合、下記のような実装になる。
    // // n 個の要素を格納することができるようにベクタを初期化する。
    // let mut v = Vec::with_capacity(n);

    // for _ in 0..n {
    //     v.push(rng.sample(&Standard))
    // }

}

// 下記ふたつの関数をまとめたもの。昇順または降順にソートされているかどうかチェックする。
pub fn is_sorted<T: Ord>(x: &[T], order: &SortOrder) -> bool {
    match *order {
        SortOrder::Ascending => x.windows(2).all(|pair| pair[0] <= pair[1]),
        SortOrder::Decending => x.windows(2).all(|pair| pair[0] >= pair[1]),
    }
}

// pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
//     x.windows(2).all(|pair| pair[0] <= pair[1])
// }

// pub fn is_sorted_decending<T: Ord>(x: &[T]) -> bool {
//     x.windows(2).all(|pair| pair[0] >= pair[1])
// }