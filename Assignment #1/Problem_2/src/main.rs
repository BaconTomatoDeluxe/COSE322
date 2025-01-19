use std::io::{stdin, Read};

const LIMIT: i64 = i64::pow(10, 9);

fn num (x: i64 , stack: &mut Vec<i64>) {
    stack.push(x);
}

fn pop (stack: &mut Vec<i64>) -> bool {
    stack.pop().is_some()
}

fn inv (stack: &mut Vec<i64>) -> bool {
    match stack.pop() {
        Some(x) => {stack.push(-x); true},
        None => false,
    } 
}

fn dup (stack: &mut Vec<i64>) -> bool {
    match stack.last() {
        Some(x) => {stack.push(*x); true},
        None => false,
    }
}

fn swp (stack: &mut [i64]) -> bool {
    let len = stack.len();
    if len >= 2 {
        stack.swap(len - 1, len - 2);
        true
    }
    else { false }
}

fn add (stack: &mut Vec<i64>) -> bool {
    match stack.pop() {
        Some(a) => match stack.pop() {
            Some(b) => {
                if (a + b).abs() <= LIMIT {stack.push(b + a); true}
                else { false }
            }
            None => false,
        }
        None => false,
    }
}

fn sub (stack: &mut Vec<i64>) -> bool {
    match stack.pop() {
        Some(a) => match stack.pop() {
            Some(b) => {
                if (b - a).abs() <= LIMIT {stack.push(b - a); true}
                else { false }
            }
            None => false,
        }
        None => false,
    }
}

fn mul (stack: &mut Vec<i64>) -> bool {
    match stack.pop() {
        Some(a) => match stack.pop() {
            Some(b) => {
                if (b * a).abs() <= LIMIT {stack.push(b * a); true}
                else { false }
            }
            None => false,
        }
        None => false,
    }
}

fn div (stack: &mut Vec<i64>) -> bool {
    match stack.pop() {
        Some(0) => false,
        Some(a) => match stack.pop() {
            Some(b) => {
                if (b / a).abs() <= LIMIT {stack.push(b / a); true}
                else { false }
            }
            None => false,
        }
        None => false,
    }
}

fn modulo (stack: &mut Vec<i64>) -> bool {
    match stack.pop() {
        Some(0) => false,
        Some(a) => match stack.pop() {
            Some(b) => {
                if (b % a).abs() <= LIMIT {stack.push(b % a); true}
                else { false }
            }
            None => false,
        }
        None => false,
    }
}

fn main() {
	let mut input: String = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    loop {
        let mut program: Vec<&str> = Vec::new();

        loop {
            match input.next().unwrap() {
                "END" => break,
                "QUIT" => return,
                "" => continue,
                op => program.push(op),
            }
        }

        let n: i64 = input.next().unwrap().parse::<i64>().unwrap();

        for _ in 0..n {
            let mut stack: Vec<i64> = Vec::new();
            let v : i64 = input.next().unwrap().parse::<i64>().unwrap();
            stack.push(v);

            for op in &program {
                if op.starts_with("NUM") {num(op[4..].parse::<i64>().unwrap(), &mut stack);}
                else if !(match *op {
                    "POP" => pop(&mut stack),
                    "INV" => inv(&mut stack),
                    "DUP" => dup(&mut stack),
                    "SWP" => swp(&mut stack),
                    "ADD" => add(&mut stack),
                    "SUB" => sub(&mut stack),
                    "MUL" => mul(&mut stack),
                    "DIV" => div(&mut stack),
                    "MOD" => modulo(&mut stack),
                    _ => false,
                }) {stack.clear(); break;}
            }

            if stack.len() == 1 {println!("{}", stack.pop().unwrap())}
            else {println!("ERROR");}
        }
        println!();
    }
}
