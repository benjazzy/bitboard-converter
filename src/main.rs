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
            let col = gen_col(|col| {
                let idx = (row * 4) + col;
                if self.0 & (1 << idx) > 0 {
                    "xx".green().to_string()
                } else {
                    "oo".red().to_string()
                }
            });

            write!(f, "{}", col)?;

            // Align the end of the bitboard print
            if !indent_line {
                write!(f, "  ")?;
            }

            write!(f, "\t\t")?;

            if indent_line {
                write!(f, "  ")?;
            }

            // Print the board indexes
            let col = gen_col(|col| {
                let idx = (row * 4) + col;
                let fmt_idx = format!("{:0>2}", idx);
                if self.0 & (1 << idx) > 0 {
                    fmt_idx.green().to_string()
                } else {
                    fmt_idx.red().to_string()
                }
            });
            write!(f, "{}", col)?;

            writeln!(f)?;
        }

        Ok(())
    }
}

fn gen_col_loop<F>(i: u32, prev: String, formatter: F) -> String
where
    F: Fn(u32) -> String,
{
    if i >= 4 {
        return prev;
    }

    gen_col_loop(i + 1, format!("{prev}  {}", formatter(i)), formatter)
}

fn gen_col<F>(formatter: F) -> String
where
    F: Fn(u32) -> String,
{
    gen_col_loop(0, String::new(), formatter)
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
        ControlFlow::Break(b) => b,
    }
}

fn main() {
    let finished_board = prompt_loop(Board::new());

    println!("Bitboard: {}", finished_board.to_uint());
}
