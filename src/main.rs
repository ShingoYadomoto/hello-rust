use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
   show_lines();
   hello_fellow();
   show_lines();
   primitive_types();
   show_lines();
   cast();
   show_lines();
   constant();
   show_lines();
   array();
   show_lines();
   func();
   show_lines();
   tuple();
   show_lines();
}

fn show_lines() {
    let line = "========================================================";
    println!("{}", line);
}

fn hello_fellow() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn primitive_types() {
    let x = 12; // デフォルトでは i32
    let a = 12u8;
    let b = 4.3; // デフォルトでは f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}

fn cast() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

const PI: f32 = 3.14159;

fn constant() {
    println!(
        "ゼロからアップル {} を作るには、まず宇宙を創造する必要があります。",
        PI
    );
}

fn array() {
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn func() {
   let total = add(1, 2);
   println!("{}", total);
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn tuple() {
    // 戻り値をタプルで返す
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // タプルを2つの変数に分解
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
