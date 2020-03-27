// 標準 (std) ライブラリから io ライブラリをインポート
// Cargo.io からダウンロードした rand 
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // ミュータブルな guess という変数を作成し、
    // String 型の関連関数 new() が返す新しい String インスタンスに束縛する
    // (関連関数-associated function: Python などの Static Method のようなもの)
    let mut guess = String::new();

    // io ライブラリの関連関数 stdin() は標準入力へのハンドルを返す
    // さらにハンドルに対して read_line() メソッドを呼んで文字列へのミュータブルな参照 (&mut) を渡す
    // 同メソッドは io::Result という値 (Ok/Err) も返す
    // expect() メソッドは呼び出された値が成功を表すものでなければ与えたメッセージとともに panic! する
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
}