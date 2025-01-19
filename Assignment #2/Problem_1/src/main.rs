use Problem_1::Game;

fn main() {
    let mut game = Game::new();

    // Game loop
    while !game.is_loss() {
        game.print_board();
        
        loop {
            game.print_block();
            println!("Put your block (r c) or Rotate (0): ");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read input");
            
            match input.trim() {
                "0" => game.rotate_block(),
                coords => {
                    let parts: Vec<&str> = coords.split_whitespace().collect();
                    if parts.len() == 2 {
                        if let (Ok(row), Ok(col)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                            match game.place_block(row, col) {
                                Some(player) => println!("{player} is not able to put the block into ({row}, {col})."),
                                None => break
                            }
                        } else {
                            println!("Invalid coordinates. Please try again.");
                        }
                    } else {
                        println!("Invalid input. Please try again.");
                    }
                }
            }
            println!();
        }

        // Switch to the next player
        game.swap_turn();
    }

    // Game over
    game.print_board();
    game.print_block();
    game.print_ending();
}
