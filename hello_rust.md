# Hello Rust  

## MEMO  

- 整数リテラルにはアンダースコア (`_`) を含めることができ、カンマの代わりに桁を見やすく表記するのに使うことができる。  

## 変数と可変性  

変数: variable  
定数: constant  
束縛: binding  
型: type  
可変性: mutability  
シャドーイング: shadowing  

### 不変（immutable）な変数の値への束縛  

`let`　キーワードを使って変数を宣言する。ただし、このあと変数 `x` への再代入は許されない。  
変数名は

```Rust  
let x = 5;
x = 6;  // コンパイルエラー！
```  

### 可変（mutable）な変数の値への束縛  

`mut` キーワードを付けると再代入が可能な変数となる。ただし、代入される値は同じ型でなければならない。  

```Rust  
let mut x = 5;
x = 6;  // OK
```  

### 変数と定数の違い  

定数は `const` キーワードで宣言する。このとき型の注釈は必須であり、**定数は定数式にしか束縛できず、関数呼び出し結果や、実行時に評価される値には束縛できない**。  
定数はグローバルスコープを含めてどんなスコープでも定義できる。  
定数の名前は、すべて大文字で単語区切りはアンダースコア（`_`）となる。  

```Rust  
const MAX_POINTS: u32 = 100_000;
```  

### シャドーイング  

`let` キーワードで、前に定義した変数と同じ名前の変数を新しく宣言できる。この変数はmutableな変数と違い、同じ名前を再利用しつつ違う型の値を束縛することができる。  

```Rust  
let spaces = "   ";          // &str 型
let spaces = spaces.len();   // usize 型  OK
```  

```Rust  
let mut m_spaces = "   ";
spaces = m_spaces.len();     // 型が異なるためコンパイルエラー！
```  

## 固定長データ型（Data Types）  

固定されたメモリ領域を必要とするデータ型は"last in, first out"のスタックメモリに保存される。  

### スカラー型（Scalar Types）  

#### 整数型（Integer Types）  

`i` がつくものは符号あり、`u`が符号なし（正の数）であり、続く数字はビットサイズを表す。整数型の基準は `i32` 型となる。  
`isize` と`usize` 型は、プログラムが動作しているコンピュータの種類に依存する（64ビットアーキテクチャなら64ビット、32ビットアーキテクチャなら32ビットになる）。  

| Length | Signed  | Unsigned |
|--------|:-------:|:--------:|
| 8-bit  | `i8`    | `u8`     |
| 16-bit | `i16`   | `u16`    |
| 32-bit | `i32`   | `u32`    |
| 64-bit | `i64`   | `u64`    |
| arch   | `isize` | `usize`  |

#### 浮動小数点型（Floating-Point Types）  

32ビットの `f32`（単精度浮動小数点数）と64ビットの `f64`（倍精度 ―）があり、基準型は `f64`である。

#### 論理値型（The Boolean Type）  

論理値型は `bool` であり、`true` または `false` のふたつしか取りうる値はない。  

#### 文字型（The Character Type）  

単一の文字（Unicode）は `char` 型である（文字列ではない）。リテラルは `'a'` のようにシングルクォーテーションで囲われる。  

### 複合型（Compound Types）  

#### タプル型（The Tuple Type）  

複数の型のいくつかの値をひとつにまとめる複合型。`()` でカンマ区切りの値を囲う。  

```Rust  
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);  // 6.4
}
```  

#### 配列型（The Array Type）  

タプルと異なり、すべての要素の型は同じでなければならない。`[]` でカンマ区切りの値を囲う。  

```Rust  
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
```  

## 可変長データ型  

コンパイル時にサイズがわからなかったり、サイズが可変のデータについては、スタックではなくヒープメモリに保存される。  

### 文字列型（The String Type）  

可変かつ伸長可能なテキスト文字列を保持するために `String` 型が用意されている。  

## 所有権（Ownership）  

メモリ領域の確保（allocate）と解放（free）は1対1で対応しなければならない。  

所有権規則  

- Rustの各値は、所有者と呼ばれる変数に対応している。  
- いかなる時も所有者はひとつである。  
- 所有者がスコープから外れたら、値は破棄される。  

```Rust  
{   // s はここでは有効ではない。まだ宣言されていない。
    let s = String::from("hello"); // s はここから有効になる。

        // s で何らかの作業をすることができる。

}  // このスコープは終わり。以降 s は有効ではない。
```  

以下のような**スタック**に保持される型に対して `Copy` 注釈が与えられる。  

- あらゆる整数型。`u32`など。  
- 論理値型である`bool`。  
- あらゆる浮動小数点型。`f64`など。  
- 文字型である`char`。  
- タプル。ただし要素として `Copy` の型だけを含む場合。たとえば `(i32, i32)` は `Copy` だが、`(i32, String)` は違う。  

このような `Copy` の型の値に対しては、  

```Rust  
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

はコンパイルされるが、上記以外の型については同様のことを行おうとするとコンパイルエラーが発生する。  

```Rust  
let s1 = String::from("hello");  // String 型の値は s1 に所有される。
let s2 = s1;                     // 値の所有権が s2 にムーブし、s1 は有効ではなくなる。

println!("{}, world!", s1);      // コンパイルエラー！
```  

関数の引数として変数を渡す際も同様であり、

```Rust  
fn main() {
    let s = String::from("hello");  // s がスコープに入る

    takes_ownership(s);             // s の値が関数にムーブされ ...
                                    // ... ここではもう有効ではない

    let x = 5;                      // x がスコープに入る（i32 型)

    makes_copy(x);                  // x も関数にムーブされるが、
                                    // i32 は Copy なので、この後にxを使っても
                                    // 大丈夫

} // ここで x と s がスコープを抜ける。sの値はムーブされてるので、何も特別なことはない。
  //

fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop` が呼ばれる。後ろ盾してたメモリが解放される。
```  

となる。  

## 借用と参照  

`&` を前置することでその値への参照（Reference）を表す。参照は参照先の所有権を持たず、スコープが終わっても参照先の値は `drop` によって破棄されない。  
関数の引数に参照を取ることを借用（Borrowing）と呼ぶ。参照を関数に渡しても元の変数は `drop` されない。  

```Rust  
fn main() {
    let s1 = String::from("hello");  // s1 は String 型。

    let len = calculate_length(&s1); // s1 の参照を関数にわたすため、所有権はムーブしない。

    println!("The length of '{}' is {}.", s1, len); // s1 はまだ有効であるため OK！
}

fn calculate_length(s: &String) -> usize { // この関数の引数は String 型の参照。
    s.len()
}
```  

参照（しているもの）は変数と同様に、デフォルトでは不変であり、

```Rust  
// NG
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");  // ここでコンパイルエラー！
}
```

は、不変であるべき参照先を変更しようとしているためコンパイルエラーとなる。  
以下のように `mut` で可変な変数および参照として扱うとよい。  

```Rust  
// OK
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```  

ひとつのスコープで、ある特定のデータに対してはひとつしか可変な参照を持てないという制約がある。  
（不変な参照はいくつでもよいが、可変な参照と不変な参照は同時に存在できない）

```Rust  
// NG
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // おなじスコープ内で、ふたつ目の可変参照はエラー！
```  

この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点です。 データ競合とは、競合条件と類似していて、これら 3 つの振る舞いが起きる時に発生します:

- ふたつ以上のポインタが同じデータに同時にアクセスする。  
- 少なくともひとつのポインタがデータに書き込みを行っている。  
- データへのアクセスを同期する機構が使用されていない。  

```Rust  
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangleはStringへの参照を返す

    let s = String::from("hello"); // sは新しいString

    &s // String sへの参照を返す
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 危険だ
```
