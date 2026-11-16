use rand::RngExt;
use std::io::{self, Write};

use crate::game_state::{Cell, GameBoard, Outcome, Symbol};

pub fn two_player_loop() {
    let mut board = GameBoard::new();
    let mut current_player = Symbol::O;
    while board.outcome.is_none() {
        let loc = prompt_player(&board, &current_player);
        match board.place(current_player, loc) {
            None => {
                // Move on to next turn
                current_player = current_player.opposite()
            }
            Some(error) => {
                // print the error and try again
                println!("{}", error.message());
            }
        }
    }
    print_outcome(&board, true);
}

pub fn single_player_loop() {
    let mut board = GameBoard::new();
    let mut current_player = Symbol::O;
    while board.outcome.is_none() {
        let loc: usize = match current_player {
            Symbol::O => prompt_player(&board, &current_player),
            Symbol::X => select_move(&board),
        };
        match board.place(current_player, loc) {
            None => {
                // Move on to next turn
                current_player = current_player.opposite()
            }
            Some(error) => {
                // print the error if the player made the error, then try again
                if current_player == Symbol::O {
                    println!("{}", error.message());
                }
            }
        }
    }
    print_outcome(&board, false);
}

fn select_move(_board: &GameBoard) -> usize {
    let mut rng = rand::rng();
    return rng.random_range(1..10);
}

fn prompt_player(board: &GameBoard, current_player: &Symbol) -> usize {
    print_board_with(board, cell_or_number);
    print!("Enter your selection: ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    match input.trim().parse::<usize>() {
        Ok(loc) => return loc,
        Err(_) => {
            println!("Sorry, I didn't understand that. Try again.");
            return prompt_player(board, current_player);
        }
    }
}

fn print_outcome(board: &GameBoard, two_player: bool) {
    let outcome = board.outcome.unwrap();
    print_board_with(&board, cell_or_blank);
    match outcome {
        Outcome::Tie => println!("The result is a tie."),
        Outcome::Winner(symbol) => match symbol {
            Symbol::O => println!("The winner is O. Congratulations!"),
            Symbol::X => {
                if two_player {
                    println!("The winner is X. Congratulations!")
                } else {
                    println!("The winner is X. Try again.")
                }
            }
        },
    }
}

fn print_board_with<F>(board: &GameBoard, cell_or: F)
where
    F: Fn(&Cell, usize) -> String,
{
    let cells: &Vec<Vec<Cell>> = &board.cells;
    // row 1
    println!(
        "{}|{}|{}",
        cell_or(&cells[0][0], 1),
        cell_or(&cells[0][1], 2),
        cell_or(&cells[0][2], 3)
    );
    println!("-----");
    //row 2
    println!(
        "{}|{}|{}",
        cell_or(&cells[1][0], 4),
        cell_or(&cells[1][1], 5),
        cell_or(&cells[1][2], 6)
    );
    println!("-----");
    //row 3
    println!(
        "{}|{}|{}",
        cell_or(&cells[2][0], 7),
        cell_or(&cells[2][1], 8),
        cell_or(&cells[2][2], 9)
    );
}

fn cell_or_number(cell: &Cell, loc: usize) -> String {
    match cell {
        Cell::Empty => format!("{}", loc),
        Cell::Taken(Symbol::O) => String::from("O"),
        Cell::Taken(Symbol::X) => String::from("X"),
    }
}

fn cell_or_blank(cell: &Cell, _loc: usize) -> String {
    match cell {
        Cell::Empty => String::from(" "),
        Cell::Taken(Symbol::O) => String::from("O"),
        Cell::Taken(Symbol::X) => String::from("X"),
    }
}
