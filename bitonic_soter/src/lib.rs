// lib.rs は lib クレートのエントリポイントである。
pub mod first;
pub mod second;
pub mod third;

pub mod utils;

// 昇順なのか降順なのか、可読性を向上させるための SortOrder 列挙型を定義する。
pub enum SortOrder {
    // ふたつのバリアントを持つ。
    Ascending,  // 昇順
    Decending,  // 降順
}