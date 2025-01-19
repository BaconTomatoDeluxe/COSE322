use std::io::{stdin, Read};

fn letter_to_grade(letter: &str) -> f64 {
    match letter {
        "A+" => 4.50,
        "A0" => 4.00,
        "B+" => 3.50,
        "B0" => 3.00,
        "C+" => 2.50,
        "C0" => 2.00,
        "D+" => 1.50,
        "D0" => 1.00,
        "F" => 0.00,
        _ => panic!("Unexpected Letter"),
    }
}

fn min_letter(g: f64) -> String {
    if g > 4.50 { String::from("impossible") }
    else if  g > 4.00 { String::from("A+") }
    else if  g > 3.50 { String::from("A0") }
    else if  g > 3.00 { String::from("B+") }
    else if  g > 2.50 { String::from("B0") }
    else if  g > 2.00 { String::from("C+") }
    else if  g > 1.50 { String::from("C0") }
    else if  g > 1.00 { String::from("D+") }
    else if g > 0.00 { String::from("D0") }
    else { String::from("F")}
}

fn main() {
	let mut input: String = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let mut sum:f64  = 0.0;
    let mut credits: u16 = 0;

    let n: u8 = input.next().unwrap().parse::<u8>().unwrap();
    let x: f64 = input.next().unwrap().parse::<f64>().unwrap() + 0.01;

    for _ in 1..n {
        let c: u16 = input.next().unwrap().parse::<u16>().unwrap();
        let letter: &str = input.next().unwrap();
        let g: f64 = letter_to_grade(letter);

        credits += c;
        sum += (c as f64) * g;
    }

    let c = input.next().unwrap().parse::<u16>().unwrap();
    credits += c;

    let g = (x * (credits as f64) - sum) / (c as f64);

    println!("{}", min_letter(g));

}
