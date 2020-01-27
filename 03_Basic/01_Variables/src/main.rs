fn main() {
    let x = 5; // Immutable
    println!("x = {}", x);
    // x = 6; // 代入はできない
    // println!("x = {}", x);
    let x = x + 10; // 再定義すれば代入可
    println!("x = {}", x);

    let mut y = 7; // Mutable
    println!("y = {}", y);
    y = 9; // 代入できる
    println!("y = {}", y);

    const MAX_POINTS: u32 = 100_000; // 定数
    println!("MAX_POINTS = {}", MAX_POINTS);

    let x: (i32, f64, u8) = (500, 6.4, 1); // タプル
    println!("x = ({}, {}, {})", x.0, x.1, x.2); // インデックスにアクセス

    let x = [1, 3, 5, 7, 9]; // 配列
    println!("x[2] = {}", x[2]);
}
