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
