// enum定義
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

// 別々の型を持たせられる
#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]
struct Ipv4Addr(u8,u8,u8,u8);

#[derive(Debug)]
struct Ipv6Addr(String);

// 構造体も持たせられる
#[derive(Debug)]
enum IpAdder2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}


fn sample01(){
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("{:?}, {:?}", v4, v6);

    let v4 = IpAddr::V4(8, 8, 8, 8);
    let v6 = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", v4, v6);

    let v4 = IpAdder2::V4(Ipv4Addr(1,2,3,4));
    let v6 = IpAdder2::V6(Ipv6Addr(String::from("::1")));
    println!("{:?}, {:?}", v4, v6);

}


fn sample02() {
    let a = Some(5);  // Option型
    let b = Some("test");
    let c: Option<i32> = None;  // Noneの場合は内部の型指定が必要
    println!("{:?}, {:?}, {:?}", a, b, c);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32 {
    return match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,   // Noneの場合の処理をいれないとエラー
        Some(i) => Some(i + 1)
    }
}

fn sample03() {
    let x1 = value_in_cents(Coin::Penny);
    let x2 = value_in_cents(Coin::Nickel);
    let x3 = value_in_cents(Coin::Dime);
    let x4 = value_in_cents(Coin::Quarter);
    println!("{:?}, {:?}, {:?}, {:?}", x1, x2, x3, x4);

    let x = Some(5);
    println!("{:?}", plus_one(x));

    let x = 8;
    match x {
        5 => println!("x is five"),
        7 => println!("x is seven"),
        _ => () // 上記以外のパターンはなにもしない
    }

    // matchさせたいパターンが1つだけの場合
    let x = 8;
    if let 8 = x {
        println!("x is eight");
    }
    else {
        println!("x is not eight");
    }
}


fn main() {
    sample01();
    sample02();
    sample03();
}
