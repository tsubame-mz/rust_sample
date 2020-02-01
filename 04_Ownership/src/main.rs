fn sample01() {
    println!("-- sample01 --");
    let a = String::from("Hello");
    let b = a;  // aはもう使用不可
    // println!("{}", a);   // エラー
    println!("{}", b);

    let a = 6;
    let b = a;  // 整数型はコピーされるのでaはまだ使用可
    println!("{}", a);  // OK
    println!("{}", b);

    let a = String::from("World");
    let b = a.clone();  // cloneすればaも使用可
    println!("{}", a);   // OK
    println!("{}", b);
}

fn test_func1(x: String) {
    println!("{}", x);   // OK
}

fn test_func2(x: i32) {
    println!("{}", x);   // OK
}

fn test_func3(x: String) -> String {
    println!("{}", x);   // OK
    return x;
}

fn sample02() {
    println!("-- sample02 --");
    let a = String::from("Hello");
    test_func1(a);  // aはmoveされるのでもう使用不可
    // println!("{}", a);   // エラー

    let a = 8;
    test_func2(a);  // 整数型はコピーされるのでaはまだ使用可
    println!("{}", a);   // OK

    let a = String::from("World");
    let b = test_func3(a);  // moveされて戻ってくる
    println!("{}", b);   // OK
}

fn calculate_length(x: String) -> usize {
    return x.len();
}

fn calculate_length2(x: &String) -> usize {  // 参照だけ渡す
    // x.push_str("World");    // 参照なので変更はできない
    return x.len();
}

fn change_str(x: &mut String) {  // 参照を渡すが変更可
    x.push_str("World");    // OK
}

fn sample03() {
    println!("-- sample03 --");
    let a = String::from("Hello");
    let n = calculate_length(a);    // aはmoveされるので、もう使えない
    // println!("length of {} is {}", a, n);   // エラー
    println!("length of *** is {}", n);

    let a = String::from("Hello");
    let n = calculate_length2(&a);    // aはmoveされない
    println!("length of {} is {}", a, n);   // OK

    let mut a = String::from("Hello");  // 変更可能な変数として宣言
    change_str(&mut a);
    println!("{}", a);
}

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

fn sample04() {
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
    sample02();
    sample03();
    sample04();
}

