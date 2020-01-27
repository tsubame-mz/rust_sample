fn main() {
    // 普通のif文
    let x = 4;
    if x % 4 == 0 {
        println!("4で割切れる数");
    } else if x % 3 == 0 {
        println!("3で割切れる数");
    } else {
        println!("その他");
    }

    // let内でif文
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num = {}", num);

    // while
    let mut num = 3;
    while num > 0 {
        println!("num = {}", num);
        num -= 1;
    }

    // 配列のiteration
    let ary = [1, 2, 3, 4, 5];
    for data in ary.iter() {
        println!("data = {}", data);
    }

    // Rangeループ
    for n in 1..4 {
        println!("n = {}", n);
    }
    for n in (1..4).rev() {
        println!("n = {}", n);
    }
}
