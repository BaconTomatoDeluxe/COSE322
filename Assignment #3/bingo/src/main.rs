use std::io;

fn main () {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        if s.contains("sum=5"){
            println!("Bingo!");
        } else if s.contains("sum=12"){
            break;
        }
    }
}