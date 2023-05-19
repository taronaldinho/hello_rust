// ← スラッシュ2つはコメントとなる。
use std::io;              // 標準のstdライブラリからioモジュールをインポート。
use std::cmp::Ordering;   // 同じくcmpのOrdering列挙をインポート。
use rand::Rng;            // Cargo.ioからダウンロードしたrandのRngトレイトをインポート（Cargo.tomlに記載が必要）。

fn main() {
    println!("Guess the number!");

    // 1から101未満の範囲の整数からランダムにひとつ抜き出す。
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    // loopで繰り返し。
    loop {

        println!("Please input your guess.");

        // ミュータブル（変更可能）なguessという変数を作成し、
        // String型の関連関数new()が返す新しいStringインスタンスに束縛する。
        // (関連関数 -> associated function: Static Methodのようなもの)
        // この時点で型推論によりRustはguessをString型と推定する。
        let mut guess = String::new();

        // ioモジュールの関連関数stdin()は標準入力へのハンドルを返す。
        // さらにハンドルに対してread_line()メソッドを呼んで文字列へのミュータブルな参照（Refernce）を渡す。
        // これにより、所有権はguessのまま、メソッドでその値を書き換えることができる。
        // ※引数に参照を渡すことを借用（Borrowing）と呼ぶ。
        // 同メソッドはio::ResultというOkまたはErrの列挙子を持つ型を返す。
        // expect()メソッドは呼び出された値が成功を表すもの（Ok）でなければ、与えたメッセージとともにpanic!する。
        io::stdin().read_line(&mut guess).expect("faild to read line!");

        // String型のguessから前後の余分なスペースや改行文字を除いて符号なし32 bit整数の
        // u32型にキャストする。→ シャドーイング（Shadowing）
        // mutな変数への再代入は元の値と同じ型でなければならないが、シャドーイングでは型が変わってもよい。
        // parse()はResultを返す。
        // Ok(num)はOkをアンラップして得られた値(整数値)をnumという名前に設定し、
        // 続く右側でその値をそのまま返している。
        let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,  // continueでloopの先頭に戻る。
                };

        println!("You guessed: {}", guess);

        // guessとsecret_number（の参照）を比較する。
        // u32型のguessと比較されることにより、secret_numberもu32型と推測される。
        // cmp()は比較可能な全てのものに対して呼べるメソッドで、引数として、比較したい相手の参照を取る。
        // そして、useしたOrdering型の値を返す。
        // match文を使って、正確にOrderingのどれであるかを判断している。
        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;  // breakでloopから抜ける。
                }
        }
    }
}
