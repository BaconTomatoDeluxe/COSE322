use std::io;
use std::io::prelude::*;

fn main() {
    let mut s = 0;
    loop {
        let mut input = String::new();
        print!("?# ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let x: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if x == 0 {
            break;
        }
        s += x;
        println!(" sum={}", s);
    }
}
