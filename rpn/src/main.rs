fn main() {
    // exp 変数を RPN (Reverse Polish Notation) 記法の文字列に束縛する
    // この RPN は数式 6.1 + 5.2 x 4.3 - 3.4 / 2.5 x 1.6 に等しい
    // let 文は新しい変数を用意し、右辺の式を評価した値に束縛する
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";

    // rpn 関数を呼び出して計算する。返された値に ans 変数を束縛する
    let ans = rpn(exp);

    // デバッグビルド時のみ、答えが正しいかチェックする
    // 浮動小数点の計算誤差を考慮し、ここでは小数点以下 4 桁までの値を文字列に変換している
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    // exp と ans の値を表示する。ans は小数点以下 4 桁まで表示する
    println!("{} = {:.4}", exp, ans);
}

// RPN 形式の文字列 exp を受け取り、f64 型の計算結果を返す
fn rpn(exp: &str) -> f64 {
    // ミュータブルな変数 stack を空のスタックに束縛する
    let mut stack = Vec::new();
    
    for token in exp.split_whitespace() {
        // token が f64 にパースできるならばスタックに積む
        if let Ok(num) = token.parse::<f64>() {            
            stack.push(num);
        } else {
            match token {
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                _ => panic!("Unknown operator: {}", token),
            }
        }
    }
    stack.pop().expect("Stack underflow")
}


fn apply2<F>(stack: &mut Vec<f64>, fun: F)
where
    F: Fn(f64, f64) -> f64,
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z =fun(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}