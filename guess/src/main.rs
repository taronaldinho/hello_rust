// ← スラッシュ 2 つはコメントとなる。
use std::io;              // 標準 (std) ライブラリから io ライブラリをインポート。
use std::cmp::Ordering;   // 同じく cmp をインポート。
use rand::Rng;            // Cargo.io からダウンロードした rand をインポート。

fn main() {
    println!("Guess the number!");

    // 1 から 101 未満 の範囲の整数からランダムに 1 つ抜き出す。
    // この時点で型推論により Rust は secret_number を i32 型と推定する。
    // i32 は符号付きの 32 bit 整数である。
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    
    // loop で繰り返し。
    loop {

        println!("Please input your guess.");

        // ミュータブルな guess という変数を作成し、
        // String 型の関連関数 new() が返す新しい String インスタンスに束縛する。
        // (関連関数-associated function: Static Method のようなもの)
        // この時点で型推論により Rust は guess を String 型と推定する。
        let mut guess = String::new();

        // io ライブラリの関連関数 stdin() は標準入力へのハンドルを返す。
        // さらにハンドルに対して read_line() メソッドを呼んで文字列へのミュータブルな参照 (Refernce) を渡す。
        // これにより、所有権は guess のままである。→ 借用 (Borrowing)
        // 同メソッドは io::Result という値 (Ok/Err) も返す。
        // expect() メソッドは呼び出された値が成功を表すものでなければ与えたメッセージとともに panic! する。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        // String 型の guess から前後の余分なスペースや改行文字を除いて符号なし 32 bit 整数の 
        // u32 型にキャストする。→ シャドーイング (Shadowing)
        // parse() は Result を返す。
        // Ok(num) は Ok をアンラップして得られた値(整数値)を num という名前に設定し、
        // 続く右側でその値をそのまま返している。
        let guess: u32 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

        println!("You guessed: {}", guess);

        // guess と secret_number (の参照) を比較する。
        // u32 型の guess と比較されることにより、secret_number も u32 型と推測される。
        // cmp() は比較可能な全てのものに対して呼べるメソッドで、引数として、比較したい相手の参照を取る。 
        // そして、use した Ordering 型の値を返す。 
        // match 文を使って、正確に Ordering のどれであるかを判断しています。         
        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {                
                println!("You win!");
                // break で loop から抜ける。
                break;
                }

        }
    }
}