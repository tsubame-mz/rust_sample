use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101); // [1, 101)

    // println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // Murtable変数
        io::stdin()
            .read_line(&mut guess) // 1行入力
            .expect("Failed to read line"); // 例外
        let guess: u32 = match guess.trim().parse() {
            // 数値に変換
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            // 比較
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // ループを抜ける
            }
        }
    }
}
