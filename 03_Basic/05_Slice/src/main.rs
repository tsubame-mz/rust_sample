fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn first_word2(s: &String) -> &str {
    // スライスを返却する
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

fn sample01() {
    let mut a = String::from("hello world");
    let n = first_word(&a);
    println!("n = {}", n);
    a.clear(); // nが意味のない値になる
    println!("a = {}", a);

    let mut b = String::from("hello world");
    let n = first_word2(&b);
    // b.clear(); // nを不変借用しているので、bは変更不可
    println!("n = {}", n);
    b.clear(); // nはもう使っていないので、bは変更可能
}

fn main() {
    sample01();
}
