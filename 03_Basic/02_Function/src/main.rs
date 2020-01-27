fn my_function(x: i32, y: i32) -> i32 {
    println!("Hello, function!");
    println!("x = {}", x);
    println!("y = {}", y);
    return x + y;
}

fn main() {
    println!("Hello, world!");
    let z = my_function(5, 7);
    println!("z = {}", z);

    // スコープ
    let y = {
        let x = 5;
        x + 4 // セミコロンはいらない
    };
    println!("y = {}", y);
}
