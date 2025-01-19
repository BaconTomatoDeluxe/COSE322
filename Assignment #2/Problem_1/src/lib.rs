use std::fmt;

use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
    P1, P2, Empty
}

#[derive(Clone, Copy)]
pub struct Point {
    r: i32,
    c: i32,
}

pub struct Block {
    b_id: usize,      // 0 -13
    rotate: usize,    // 0-3
}

pub struct Game {
    board: [[Player; 9]; 9],
    current_player: Player,
    current_block: Block,
    losser: Option<Player>,
}

// //block_list[b_id][rotate]
const BLOCK_LIST: [[[Option<Point>; 5]; 4]; 14] = [
    [
        // b_id: 0
        //  _ _ _ 
        // |O|O|O|
        // |_|_|_|
        // |_|_|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), None, None],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 0}), None, None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), None, None],
        [Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 2, c: 2}), None, None],
    ],
    [
        // b_id: 1
        //  _ _ _ 
        // |O|O|O|
        // |O|_|_|
        // |_|_|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 0}), None],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 2}), None],
        [Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 2, c: 2}), Some(Point {r: 0, c: 1}), None],
    ],
    [
        // b_id: 2
        //  _ _ _ 
        // |O|O|O|
        // |_|_|O|
        // |_|_|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), None],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 0}), Some(Point {r: 0, c: 1}), None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 0}), None],
        [Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 2, c: 2}), Some(Point {r: 2, c: 1}), None],
    ],
    [
        // b_id: 3
        //  _ _ _ 
        // |O|O|O|
        // |_|O|_|
        // |_|_|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 0}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 1}), None],
    ],
    [
        // b_id: 4
        //  _ _ _ 
        // |_|O|O|
        // |O|O|_|
        // |_|_|_|
        [Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 1}), None],
    ],
    [
        // b_id: 5
        //  _ _ _ 
        // |O|O|_|
        // |_|O|O|
        // |_|_|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 1}), None],
    ],
    [
        // b_id: 6
        //  _ _ _ 
        // |O|O|_|
        // |O|O|_|
        // |_|_|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 1, c: 2}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 1}), None],
        [Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), None],
    ],
    [
        // b_id: 7
        //  _ _ _ 
        // |O|_|_|
        // |O|O|_|
        // |O|O|_|
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 0})],
        [Some(Point {r: 1, c: 2}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 2, c: 0})],
        [Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 2, c: 2})],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 2})],
    ],
    [
        // b_id: 8
        //  _ _ _ 
        // |_|O|O|
        // |O|O|_|
        // |O|_|_|
        [Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), Some(Point {r: 2, c: 0})],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 1}), Some(Point {r: 2, c: 2})],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 2})],
        [Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 0})],
    ],
    [
        // b_id: 9
        //  _ _ _ 
        // |O|O|_|
        // |_|O|O|
        // |_|O|_|
        [Some(Point {r: 0, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 2, c: 1})],
        [Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 0}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 2})],
        [Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 1})],
        [Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0})],
    ],
    [
        // b_id: 10
        //  _ _ _ 
        // |_|O|O|
        // |O|O|_|
        // |_|O|_|
        [Some(Point {r: 0, c: 1}), Some(Point {r: 0, c: 2}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 1}), Some(Point {r: 2, c: 1})],
        [Some(Point {r: 0, c: 0}), Some(Point {r: 1, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 2})],
        [Some(Point {r: 2, c: 0}), Some(Point {r: 2, c: 1}), Some(Point {r: 1, c: 2}), Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 1})],
        [Some(Point {r: 2, c: 2}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0})],
    ],
    [
        // b_id: 11
        //  _ _ _ 
        // |_|_|O|
        // |O|O|O|
        // |O|_|_|
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 2}), Some(Point {r: 2, c: 0})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1}), Some(Point {r: 0, c: 0}), Some(Point {r: 2, c: 2})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 2}), Some(Point {r: 2, c: 0})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1}), Some(Point {r: 0, c: 0}), Some(Point {r: 2, c: 2})],
    ],
    [
        // b_id: 12
        //  _ _ _ 
        // |O|_|_|
        // |O|O|O|
        // |_|_|O|
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 0}), Some(Point {r: 2, c: 2})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 0}), Some(Point {r: 0, c: 2})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 0}), Some(Point {r: 2, c: 2})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1}), Some(Point {r: 2, c: 0}), Some(Point {r: 0, c: 2})],
    ],
    [
        // b_id: 13
        //  _ _ _ 
        // |_|O|_|
        // |O|O|O|
        // |_|O|_|
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1})],
        [Some(Point {r: 1, c: 1}), Some(Point {r: 1, c: 0}), Some(Point {r: 1, c: 2}), Some(Point {r: 0, c: 1}), Some(Point {r: 2, c: 1})],
    ],
];

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::P1 => write!(f, "{}", 'O'),
            Player::P2 => write!(f, "{}", '@'),
            Player::Empty => write!(f, "{}", '_'),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        let block = Self::generate_block();
        Self {
            board: [[Player::Empty; 9]; 9],
            current_player: Player::P1,
            current_block: block,
            losser: None,
        }
    }

    pub fn print_board(&self) {
        println!();
        println!("   1 2 3 4 5 6 7 8 9");
        println!("   _ _ _ _ _ _ _ _ _");
        let mut  i = 1;
        for r in &self.board {
            print!("{} |", i);
            i += 1;
            for c in r {
                print!("{}|", c);
            }
            println!();
        }
        println!();
    }

    pub fn print_block(&self) {
        let mut block_board = vec![[Player::Empty; 3]; 3];

        let block = Self::get_block(self.current_block.b_id, self.current_block.rotate);
        for point in block {
            block_board[point.r as usize][point.c as usize] = self.current_player;
        }

        match self.current_player {
            Player::P1 => println!("P1's block:"),
            Player::P2 => println!("P2's block:"),
            _ => println!("ERROR"),
        }

        println!(" _ _ _");
        for r in block_board {
            print!("|");
            for c in r {
                print!("{}|", c);
            }
            println!();
        }
        println!();
    }

    pub fn print_ending(&self) {
        if self.losser == Some(Player::P1) {
                println!("P1 fails to put the block. P2 wins!");
        }
        if self.losser == Some(Player::P2) {
            println!("P2 fails to put the block. P1 wins!");
        }
    }

    pub fn rotate_block(&mut self) {
        self.current_block = Block {b_id:self.current_block.b_id, rotate: (self.current_block.rotate + 1) % 4};
    }

    pub fn place_block(&mut self, r: i32, c: i32) -> Option<Player> {
        let current_block: Vec<Point> = Self::get_block(self.current_block.b_id, self.current_block.rotate);
        let block_in_board: Vec<Point> = current_block.into_iter().map(|point| Point {r: point.r + r - 1, c: point.c + c - 1}).collect();

        let is_valid = |row: i32, col: i32| row >= 0 && row < 9 && col >= 0 && col < 9 && self.board[row as usize][col as usize] == Player::Empty;
        if block_in_board.iter().all(|point| is_valid(point.r, point.c)) {
            for point in &block_in_board {
                self.board[point.r as usize][point.c as usize] = self.current_player;
            }
        } else {
            return Some(self.current_player);
        }
        // Successfully locate block
        None
    }

    pub fn swap_turn(&mut self) {
        self.current_player = match self.current_player {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
            Player::Empty => Player::Empty,
        };

        self.current_block = Self::generate_block();
    }

    fn generate_block() -> Block {
        let b_id = rand::thread_rng().gen_range(0..14);
        let rotate = rand::thread_rng().gen_range(0..4);
        Block { b_id, rotate }
    }

    pub fn is_loss(&mut self) -> bool {
        let is_possible: bool = Self::check_possible(&self, self.current_block.b_id);

        if !is_possible {
            self.losser = Some(self.current_player);
        }
        !is_possible
    }

    fn can_place_block(&self, block_id: usize, rotation: usize, start_row: i32, start_col: i32) -> bool {
        let block = Self::get_block(block_id, rotation);
        let is_valid = |row: i32, col: i32| row >= 0 && row < 9 && col >= 0 && col < 9 && self.board[row as usize][col as usize] == Player::Empty;
        if block.iter().all(|point| is_valid(point.r + start_row - 1, point.c + start_col - 1)) {
            return true
        }
        false // All points are within bounds and unoccupied
    }

    fn check_possible(&self, block_id: usize) -> bool {
        // Iterate over each cell on the board
        for row in -1..10 {
            for col in -1..10 {
                // For each rotation (0 to 3)
                for rotation in 0..4 {
                    // Check if the block can be placed at (row, col) with the current rotation
                    if self.can_place_block(block_id, rotation, row, col) {
                        return true; // Found a valid placement, player can continue
                    }
                }
            }
        }
        false // No valid placement found; player loses
    }
    
    fn get_block(block_id: usize, rotation: usize) -> Vec<Point> {
        let mut block: Vec<Point> = Vec::new();

        for point in BLOCK_LIST[block_id][rotation] {
            if let Some(point) = point {
                block.push(point);
            }
        }

        block
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initialization() {
        let game = Game::new();
        for row in &game.board {
            for cell in row {
                assert_eq!(*cell, Player::Empty);
            }
        }
    }

    #[test]
    fn test_place_block_valid() {
        let mut game = Game::new();
        assert_eq!(game.place_block(1, 1), None);
    }

    #[test]
    fn test_place_block_invalid() {
        let mut game = Game::new();
        game.place_block(1, 1);
        assert!(!game.can_place_block(game.current_block.b_id, game.current_block.rotate, 1, 1));
    }

    #[test]
    fn test_switch_turn() {
        let mut game = Game::new();
        assert_eq!(game.current_player, Player::P1, "inital player: P1");
        game.swap_turn();
        assert_eq!(game.current_player, Player::P2, "swaped palyer: P2");
    }

    #[test]
    fn test_is_loss_false() {
        let mut game = Game::new();
        game.current_block.b_id = 3;
        game.board = [[Player::P1; 9]; 9];
        game.board[7][6] = Player::Empty;
        game.board[7][7] = Player::Empty;
        game.board[7][8] = Player::Empty;
        game.board[8][7] = Player::Empty;
        // current board
        //  _ _ _ _ _ _ _ _ _ 
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|_|_|_|
        // |O|O|O|O|O|O|O|_|O|

        // b_id: 3
        //  _ _ _ 
        // |O|O|O|
        // |_|O|_|
        // |_|_|_|

        assert!(!game.is_loss());
    }

    #[test]
    fn test_is_loss_true() {
        let mut game = Game::new();
        game.current_block.b_id = 3;
        game.board = [[Player::P1; 9]; 9];
        game.board[7][6] = Player::Empty;
        game.board[7][7] = Player::Empty;
        game.board[7][8] = Player::Empty;
        // current board
        //  _ _ _ _ _ _ _ _ _ 
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|O|O|O|
        // |O|O|O|O|O|O|_|_|_|
        // |O|O|O|O|O|O|O|O|O|

        // b_id: 3
        //  _ _ _ 
        // |O|O|O|
        // |_|O|_|
        // |_|_|_|

        assert!(game.is_loss());
    }
}