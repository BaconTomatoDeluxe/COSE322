use std::io::{stdin, Read};

fn valid_pos(x: i16, y: i16) -> bool {
    (0..19).contains(&x) && (0..19).contains(&y)
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut board = [[0; 19]; 19];

    for i in 0..19 {
        for j in 0..19 {
            board[i][j] = input.next().unwrap();
        }
    }
    
    let directions:[(i16, i16); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];

    for i in 0..19 {
        for j in 0..19 {
            if board[i][j] != 0 {
                for &(dx, dy) in &directions {
                    let mut five: bool = true;

                    for k in 1..5 {
                        let (nx, ny) = (i as i16 + k * dx, j as i16 + k * dy);
                        if  !valid_pos(nx, ny) || board[i][j] != board[nx as usize][ny as usize] { five = false; break;}
                    }

                    if five {
                        let (px, py) = (i as i16 - dx, j as i16 - dy);
                        if valid_pos(px, py) && board[i][j] == board[px as usize][py as usize] { continue; }

                        let (nx, ny) = (i as i16 + 5 * dx, j as i16 + 5 * dy);
                        if valid_pos(nx, ny) && board[i][j] == board[nx as usize][ny as usize] { continue; }

                        println!("{}", board[i][j]);
                        println!("{} {}", i + 1, j + 1);
                        return;
                    }
                        
                }
            }
        }
    }
    println!("0");
}
