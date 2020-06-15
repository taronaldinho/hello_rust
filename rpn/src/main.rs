fn main() {
    // exp 変数を RPN (Reverse Polish Notation) 記法の文字列に束縛する。
    // この RPN は数式 6.1 + 5.2 x 4.3 - 3.4 / 2.5 x 1.6 に等しい。
    // let 文は新しい変数を用意し、右辺の式を評価した値に束縛する。
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    // rpn 関数を呼び出して計算する。返された値に ans 変数を束縛する。
    let ans = rpn(exp);

    // デバッグビルド時のみ、答えが正しいかチェックする。
    // 浮動小数点の計算誤差を考慮し、ここでは小数点以下 4 桁までの値を文字列に変換している。
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    // exp と ans の値を表示する。ans は小数点以下 4 桁まで表示する。
    println!("{} = {:.4}", exp, ans);
}

// RPN 形式の文字列 exp を受け取り、f64 型の計算結果を返す関数。
fn rpn(exp: &str) -> f64 {
    // ミュータブルな変数 stack を空のスタックに束縛する。
    let mut stack = Vec::new();
    
    // exp の要素を半角スペースで分割し、それらに token を順に束縛する。
    for token in exp.split_whitespace() {

        // token が f64 型にパースできるならば (→ 数値) スタックに push する。
        if let Ok(num) = token.parse::<f64>() {            
            stack.push(num);

        } else {

            // token が演算子なら apply2 関数で計算する。
            // |x, y| x + y の部分はクロージャ (無名関数のようなもの)。
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                // token が演算子でないなら、エラー (panic!) を起こして終了。
                _ => panic!("Unknown operator: {}", token),
            }
        }
    }

    // スタックから数値を 1 つ取り出す。
    // expect によって、エラーが発生した場合は指定した文字列を表示して終了する。
    stack.pop().expect("Stack underflow")

}

// スタックから数値を 2 つ取り出し、F 型のクロージャ fun で計算し、結果を stack に push する。
fn apply2<F>(stack: &mut Vec<f64>, fun: F)
// F 型のトレイト境界を定義する。
    where F: Fn(f64, f64) -> f64,
{
    // 変数 y と x を stack の最後の 2 要素に束縛する。
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {

        // クロージャ fun で計算し、その結果に変数 z を束縛する。
        let z =fun(x, y);

        // stack に z を push する。
        stack.push(z);

    } else {

        panic!("Stack underflow");

    }
}