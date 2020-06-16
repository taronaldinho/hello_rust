// ジェネリクスに対応した実装。

// lib.rs から SortOrder 列挙型を読み込む。
use super::SortOrder;

// pub 修飾子でモジュール (このファイル) 外から呼び出し可能となる。
// 型パラメータを <T> で導入する。ただし、大小比較ができるものに型を制限するために、
// <T: Ord> としてトレイト境界に Ord を指定する。
// 引数は可変な Ord を持つ何らかの型の値からなるスライス (配列) の参照と、SortOrder 列挙型。
// 戻り値は、Result 型。関数成功時はユニット型 ()、エラーの場合は String 型となる。
pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
    // is_power_of_two 関数で引数 x の長さが 2 のべき乗かどうかをチェックする。
    if x.len().is_power_of_two() {
        // SortOrder 型の参照である order の参照を外してマッチングする。
        match *order {
            SortOrder::Ascending => do_sort(x, true),
            SortOrder::Decending => do_sort(x, false),
        };

        // 2 のべき乗の時は do_sort 関数の処理の後にユニット型 () を返す。
        Ok(())
    } else {
        // エラー時 (長さが 2 のべき乗ではない) はエラー文字列を返す。
        Err(format!(
            "The length of x is not a power of two. (x.len(): {})",
            x.len()
        ))
    }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        do_sort(&mut x[..mid_point], true);
        do_sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
    compare_and_swap(x, up);
    if x.len() > 1 {
        let mid_point = x.len() / 2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
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
    // また、SortOrder 列挙型のすべてのバリアントも読み込む。
    use super::sort;
    use crate::SortOrder::*;

    // #[test] の付いた関数は cargo test を実行したとき実行される。
    // 整数値のソートのテストを行う。
    #[test]
    fn sort_u32_ascending() {
        // x に型注釈 Vec<u32> をつける。
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];

        // &Ascending は SortOrder 列挙型のバリアントのひとつ。
        // まずは sort 関数が Result 型の Ok (ユニット) を返すかどうかをチェック。
        assert_eq!(sort(&mut x, &Ascending), Ok(()));

        // 次にソート結果が想定したものかどうかをチェックする。
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_decending() {
        let mut x: Vec<u32> = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Decending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    // 文字列のソートのテストを行う。
    #[test]
    fn sort_str_ascending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(
            x,
            vec![
                "GC",
                "Rust",
                "and",
                "fast",
                "is",
                "memory-efficient",
                "no",
                "with"
            ]
        );
    }

    #[test]
    fn sort_str_decending() {
        let mut x = vec![
            "Rust",
            "is",
            "fast",
            "and",
            "memory-efficient",
            "with",
            "no",
            "GC",
        ];
        assert_eq!(sort(&mut x, &Decending), Ok(()));
        assert_eq!(
            x,
            vec![
                "with",
                "no",
                "memory-efficient",
                "is",
                "fast",
                "and",
                "Rust",
                "GC"
            ]
        )
    }

    // x の長さが 2 のべき乗でない場合にエラーを返すかどうかのテストを行う。
    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err());
    }
}
