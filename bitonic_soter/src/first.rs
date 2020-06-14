// もっとも単純な実装。

// pub 修飾子でモジュール (このファイル) 外から呼び出し可能となる。
// 引数は可変な u32 型の値からなるスライス (配列) の参照と、真理値。
pub fn sort(x: &mut [u32], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut [u32], up: bool) {
    compare_and_swap(x, up);
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut [u32], up: bool) {
    let mid_point = x.len() / 2;
    for i in 0..mid_point {
        // スライスの持つ swap メソッドで必要に応じて値を交換する。
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

// 単体テスト。
// このモジュールは cargo test を実行したときのみコンパイルされる。
#[cfg(test)]
mod tests {
    // 親モジュールの sort 関数をテストのために読み込む。
    use super::sort;

    // #[test] の付いた関数は cargo test を実行したとき実行される。     
    #[test]
    fn sort_u32_ascending() {
        let mut x  = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, true);
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_decending() {
        let mut x  = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, false);
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

}