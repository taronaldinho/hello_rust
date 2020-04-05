// Re:FizzBuzzから始めるRust生活 - Qiita
// https://qiita.com/hinastory/items/543ae9749c8bccb9afbc

fn main() {
    fizz_buzz_8(100);
}




fn fizz_buzz_0(limit: i32) {
    let mut i: i32 = 1;                // i は ミュータブルな値に束縛する変数名なので mut を付ける。
                                       // ここで型アノテーションがなくてもよい。 (Rust の型推論がはたらく)

    loop {                             // loop による無限ループ。
        if i % 3 == 0 && i % 5 == 0 {  // if による条件分岐処理。&& 演算子で複数条件 AND で判断する。
            println!("FizzBuzz");      // || 演算子で OR となる。
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        if i == limit {                // 無限ループを抜けるための条件分岐処理。
            break;                     // break でループを抜ける。
        }
        i += 1;                        // カウントアップ。
    }
}


fn fizz_buzz_1(limit: i32) {
    let mut i = 1;                     // Rust の型推論を利用。

    while i <= limit {                 // while による継続判断付きのループ処理。
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
        i += 1;
    }
}


fn fizz_buzz_2(limit: i32) {
    for x in 1..limit+1 {              // for in による集合要素ひとつひとつに対する処理。
        if x % 3 == 0 && x % 5 == 0 {  // これによりカウンタおよびカウントアップが不要になる。
            println!("FizzBuzz");      // また、スライス表記 .. で範囲 (整数値の集合) を表している。
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }        
    }
}

fn fizz_buzz_3(limit: i32) {
    for x in 1..=limit {                     // ..=endpoint で終端を含む集合に。
        match x % 15 {                       // match を利用して x の 15 による剰余を分類してマッチ。
            0              => println!("FizzBuzz"),  // 単式パターン
            3 | 6 | 9 | 12 => println!("Fizz"),      // 複式パターン
            5 | 10         => println!("Buzz"),
            _              => println!("{}", x),     // _ を用いるとワイルドカードパターンになる。
        }                                    // ひとつの match のアーム (=>) は同じ型でなければならない。
    }
}


fn fizz_buzz_4(limit: i32) {
    for x in 1..=limit {
        match x {
            e if e % 15 == 0 => println!("FizzBuzz"),  // パターンの列挙をしない方法。
            e if e % 3 == 0  => println!("Fizz"),      // if 以降はパターンガードと呼ばれる。
            e if e % 5 == 0  => println!("Buzz"),
            e                => println!("{}", e),
        }
    }
}


fn fizz_buzz_5(limit: i32) {
    for x in 1..=limit {
        match (x % 3, x % 5) {            // タプルとのマッチング。
        (0, 0) => println!("FizzBuzz"), 
        (0, _) => println!("Fizz"),     
        (_, 0) => println!("Buzz"),     
        _      => println!("{}", x),
        }
    }
}


fn fizz_buzz_6(limit: i32) {
    for x in 1..=limit {
        let s = match (x % 3, x % 5) {        // match は式であるため、評価された値を返す。
            (0, 0) => "FizzBuzz".to_string(), // (ここではそれを s に代入する)
            (0, _) => "Fizz".to_string(),     // "" で囲われたリテラルは &str (文字列スライス型の参照) 
            (_, 0) => "Buzz".to_string(),     // であり、to_string() によって　String 型となる。
            _      => x.to_string(),
        };                                    // let キーワードを使って代入するとき、末尾に ; が必要。
        println!("{}", s);
    }
}


fn fizz_buzz_7(limit: i32) {
    for x in 1..=limit {
        let tmp;                      // 後述。
        let s = match (x % 3, x % 5) {
            (0, 0) => "FizzBuzz",     // &str
            (0, _) => "Fizz",         // &str
            (_, 0) => "Buzz",         // &str
            _      => {
                tmp = x.to_string();  // String 型の tmp の参照は Rust の型強制によって &str になる。
                &tmp                  // このアームを抜けても参照が有効でなければならないので、
            },                        // 前述の行での tmp の宣言が必要になる。
        };
        println!("{}", s);
    }
}


fn fizz_buzz_8(limit: i32) {
    let fz = |x: i32| {       // クロージャ (無名関数) の定義
        match (x % 3, x % 5) {
            (0, 0) => format!("FizzBuzz"),
            (0, _) => format!("Fizz"),
            (_, 0) => format!("Buzz"),
                _  => x.to_string(),
        }
    };
    // map はイテレータを返すイテレータアダプタ、for_ezch はユニットを返すコンシューマと呼ばれるもの。
    (1..=limit).map(fz).for_each(|x| println!("{}", x)); // 高階関数とクロージャの利用
}