// 構造体
#[derive(Debug)] // Debugトレイト
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn sample01() {
    let user1 = User {
        username: String::from("test"),
        email: String::from("test@test.test"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    // mutはインスタンス全体が対象
    let mut user2 = User {
        username: String::from("test2"),
        email: String::from("test2@test.test"),
        active: true,
        sign_in_count: 2,
    };
    user2.email = String::from("test22.test.test");
    println!("{:?}", user2);

    let user3 = User {
        username: String::from("test3"),
        email: String::from("test3@test.test"),
        ..user1 // 残りの項目はuser1と同じにする
    };
    println!("{:?}", user3);
}

#[derive(Debug)]
struct Point(i32, i32, i32); // タプル構造体

#[derive(Debug)]
struct Color(i32, i32, i32);

fn sample02() {
    let p = Point(0, 1, 2);
    let c = Color(3, 4, 5);
    println!("{:?}", p);
    println!("{:#?}", c);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // C++のクラスのように構造体上にメソッドを定義
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    // 関連関数
    fn square(size: u32) -> Rectangle {
        // Rectangleを生成
        return Rectangle {
            width: size,
            height: size,
        };
    }
}

fn sample03() {
    let rect1 = Rectangle {
        width: 5,
        height: 7,
    };
    println!("{}", rect1.area());

    let square1 = Rectangle::square(4);
    println!("{}", square1.area());
}

fn main() {
    sample01();
    sample02();
    sample03();
}
