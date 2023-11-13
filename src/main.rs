use std::{io, ops::ControlFlow};

use colored::Colorize;

#[derive(Debug, Clone, Copy, Default)]
struct Board(u32);

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle(&self, square: u8) -> Result<Self, ()> {
        if square > 31 {
            return Err(());
        }

        Ok((self.0 ^ (1 << square)).into())
    }

    pub fn to_uint(self) -> u32 {
        self.0
    }
}

impl From<u32> for Board {
    fn from(value: u32) -> Self {
        Board(value)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..8 {
            let indent_line = row % 2 == 0;
            if indent_line {
                write!(f, "  ")?;
            }

            // Print the bitboard
            for col in 0..4 {
                let idx = (row * 4) + col;

                let symbol = if self.0 & (1 << idx) > 0 {
                    "xx".green()
                } else {
                    "oo".red()
                };
                write!(f, "{}  ", symbol)?;
            }

            // Align the end of the bitboard print
            if !indent_line {
                write!(f, "  ")?;
            }

            write!(f, "\t\t")?;

            if indent_line {
                write!(f, "  ")?;
            }

            // Print key
            for col in 0..4 {
                let idx = (row * 4) + col;
                let fmt_idx = format!("{:0>2}", idx);
                let colord_idx = if self.0 & (1 << idx) > 0 {
                    fmt_idx.green()
                } else {
                    fmt_idx.red()
                };

                write!(f, "{}  ", colord_idx)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn read_input(board: Board) -> ControlFlow<Board, Board> {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if buffer.trim() == "done" {
            return ControlFlow::Break(board);
        }

        match buffer.trim().parse::<u8>() {
            Ok(square) if square < 32 => {
                let toggle_result = board.toggle(square);
                match toggle_result {
                    Ok(board) => return ControlFlow::Continue(board),
                    Err(_) => println!("Problem toggling board"),
                }
            }
            Ok(_) => println!("Please enter a valid square index"),
            Err(e) => println!("Problem reading square index: {e}"),
        }
    }
}

fn prompt_loop(board: Board) -> Board {
    println!("{}", board);

    match read_input(board) {
        ControlFlow::Continue(b) => prompt_loop(b),
        ControlFlow::Break(b) => return b,
    }
}

fn main() {
    let finished_board = prompt_loop(Board::new());

    println!("Bitboard: {}", finished_board.to_uint());
}
