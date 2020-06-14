// クロージャを利用した実装。

// lib.rs から SortOrder 列挙型を読み込む。
use super::SortOrder;
// 
use std::cmp::Ordering;

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
        // 成功時はユニット型 () を返す。
        Ok(())
    } else {
        // エラー時 (長さが 2 のべき乗ではない) はエラー文字列を返す。
        Err(format!("The length of x is not a power of two. (x.len(): {}", x.len()))
    }
    
}

// 型パラメータ <F> はクロージャを表す。
// クロージャはすべて別の型として扱われるため、引数にとるときは必ずジェネリクスにする必要がある。
pub fn sort_by<T, F>(x: &mut [T], comparator: &F) -> Result<(), String>
    // where 節で F のトレイト境界を定義する。
    // Fn トレイトは環境へのアクセスが読み出し専用で、何度でも呼ぶことができるもの。
    where F: Fn(&T, &T) -> Ordering
{
    if x.len().is_power_of_two() {
        do_sort(x, true, comparator);
        Ok(())
    } else {
        Err(format!("The length of x is not a power of two. (x.len(): {}", x.len()))
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
        if (x[i] > x[mid_point + i]) == up {
            x.swap(i, mid_point + i);
        }
    }
}

// 単体テスト。
// このモジュールは cargo test を実行したときのみコンパイルされる。
#[cfg(test)]
mod tests {
    // 親モジュールの sort と sort_by 関数をテストのために読み込む。
    // また、SortOrder 列挙型のすべてのバリアントも読み込む。
    use super::{sort, sort_by};
    use crate::SortOrder::*;

    // Student 構造体を定義する。
    struct Student {
        // Student は 3 つのフィールドを持つ。
        first_name: String,
        last_name: String,
        age: u8,
    }
    // Student のを実装する。
    impl Student {
        // 初期化のための関連関数 new を定義する。ここでは Self は Student の別名となる。
        fn new(first_name: &str, last_name: &str, age: u8) -> Self {
            Self {
                first_name: first_name.to_string(),  // &str を String に変換する。
                last_name: last_name.to_string(),
                age,                                 // フィールドと変数が同名の時は
            }                                        // このように省略形にできる。
        }
    }

    // #[test] の付いた関数は cargo test を実行したとき実行される。
    // 整数値のソートのテストを行う。
    #[test]
    fn sort_u32_ascending() {
        // x に型注釈 Vec<u32> をつける。
        let mut x:Vec<u32>  = vec![10, 30, 11, 20, 4, 330, 21, 110];
        // &Ascending は SortOrder 列挙型のバリアントのひとつ。
        // まずは sort 関数が Result 型の Ok (ユニット) を返すかどうかをチェック。
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        // 次にソート結果が想定したものかどうかをチェックする。
        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_decending() {
        let mut x: Vec<u32>  = vec![10, 30, 11, 20, 4, 330, 21, 110];
        assert_eq!(sort(&mut x, &Decending), Ok(()));
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }

    // 文字列のソートのテストを行う。
    #[test]
    fn sort_str_ascending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Ascending), Ok(()));
        assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
    }

    #[test]
    fn sort_str_decending() {
        let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
        assert_eq!(sort(&mut x, &Decending), Ok(()));
        assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"])
    }

    // x の長さが 2 のべき乗でない場合にエラーを返すかどうかのテストを行う。
    #[test]
    fn sort_to_fail() {
        let mut x = vec![10, 30, 11];
        assert!(sort(&mut x, &Ascending).is_err());
    }

    // Student の age フィールドによるソートのテストを行う。
    #[test]
    fn sort_students_by_age_ascending() {
        // 4 つの Student 型のデータを作成する。
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&hanako, &kyoko, &taro, &ryosuke];

        // sort_by 関数のテスト。
        assert_eq!(
            sort_by(&mut x, &|a, b| a.age.cmp(&b.age)),
            Ok(())
        );

        assert_eq!(x, expected);
    }

    #[test]
    fn sort_students_by_name_ascending() {
        let taro = Student::new("Taro", "Yamada", 16);
        let hanako = Student::new("Hanako", "Yamada", 14);
        let kyoko = Student::new("Kyoko", "Ito", 15);
        let ryosuke = Student::new("Ryosuke", "Hayashi", 17);

        let mut x = vec![&taro, &hanako, &kyoko, &ryosuke];

        let expected = vec![&ryosuke, &kyoko, &hanako, &taro];

        assert_eq!(
            sort_by(&mut x, 
                    &|a, b| a.last_name.cmp(&b.last_name)
                    .then_with(|| a.first_name.cmp(&b.first_name))),
            Ok(())
        );

        assert_eq!(x, expected);
    }
}