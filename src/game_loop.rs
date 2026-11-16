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
    let outcome = board.outcome.unwrap();
    print_board(&board);
    match outcome {
        Outcome::Tie => println!("The result is a tie."),
        Outcome::Winner(symbol) => println!("The winner is {}. Congratulations!", symbol),
    }
}

fn prompt_player(board: &GameBoard, current_player: &Symbol) -> usize {
    print_board_with_options(board);
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

fn print_board(board: &GameBoard) {
    let cells = &board.cells;
    // row 1
    println!(
        "{}|{}|{}",
        cell_or_blank(&cells[0][0]),
        cell_or_blank(&cells[0][1]),
        cell_or_blank(&cells[0][2])
    );
    println!("-----");
    //row 2
    println!(
        "{}|{}|{}",
        cell_or_blank(&cells[1][0]),
        cell_or_blank(&cells[1][1]),
        cell_or_blank(&cells[1][2])
    );
    println!("-----");
    //row 3
    println!(
        "{}|{}|{}",
        cell_or_blank(&cells[2][0]),
        cell_or_blank(&cells[2][1]),
        cell_or_blank(&cells[2][2])
    );
}

fn print_board_with_options(board: &GameBoard) {
    let cells = &board.cells;
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

fn cell_or(cell: &Cell, loc: usize) -> String {
    match cell {
        Cell::Empty => format!("{}", loc),
        Cell::Taken(Symbol::O) => String::from("O"),
        Cell::Taken(Symbol::X) => String::from("X"),
    }
}

fn cell_or_blank(cell: &Cell) -> String {
    match cell {
        Cell::Empty => String::from(" "),
        Cell::Taken(Symbol::O) => String::from("O"),
        Cell::Taken(Symbol::X) => String::from("X"),
    }
}
