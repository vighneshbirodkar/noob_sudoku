use std::env;
use std::fmt;
use std::fs;
use std::io::{Error, ErrorKind};

const SUDOKU_SIZE: usize = 9;
const CELL_SIZE: usize = 3;

struct Sudoku {
    grid: [[u8; SUDOKU_SIZE]; SUDOKU_SIZE],
    num_empty: usize,
}

struct Choices {
    num_choices: usize,
    choices: [bool; SUDOKU_SIZE + 1],
}

impl fmt::Display for Choices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "choices(n={}) - [", self.num_choices)?;
        for i in 1..=SUDOKU_SIZE {
            if self.choices[i] {
                write!(f, "{} ", i)?;
            }
        }
        write!(f, "]")?;
        return Ok(());
    }
}

impl Choices {
    fn new() -> Choices {
        let mut choices = [true; SUDOKU_SIZE + 1];
        choices[0] = false;
        return Choices {
            num_choices: 9,
            choices: choices,
        };
    }

    fn invalidate(&mut self, value: usize) {
        if self.choices[value] {
            self.num_choices -= 1
        }
        self.choices[value] = false;
    }

    fn len(&self) -> usize {
        return self.num_choices;
    }
}

impl Iterator for Choices {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        if self.len() == 0 {
            return None;
        }
        for i in 1..=SUDOKU_SIZE {
            if self.choices[i] {
                return Some(i as u8);
            }
        }
        return None;
    }
}

impl Sudoku {
    fn get_choices(&self, row: usize, col: usize) -> Choices {
        if row >= SUDOKU_SIZE {
            panic!("row - {} out of bounds", row);
        }

        if col >= SUDOKU_SIZE {
            panic!("row - {} out of bounds", row);
        }

        let mut choices = Choices::new();

        for index in 0..SUDOKU_SIZE {
            choices.invalidate(self.grid[index][col] as usize);
            choices.invalidate(self.grid[row][index] as usize);
        }

        let rc = (row / CELL_SIZE) * CELL_SIZE;
        let cc = (col / CELL_SIZE) * CELL_SIZE;

        for ri in rc..(rc + CELL_SIZE) {
            for ci in cc..(cc + CELL_SIZE) {
                choices.invalidate(self.grid[ri][ci] as usize);
            }
        }

        return choices;
    }

    fn from_file(filename: &String) -> std::io::Result<Sudoku> {
        let string = fs::read_to_string(filename)?;
        let split_str: Vec<&str> = string.split_whitespace().collect();
        let mut num_empty: usize = 0;

        if split_str.len() != (SUDOKU_SIZE * SUDOKU_SIZE) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid length of numbers {}", split_str.len()),
            ));
        }

        let mut parsed_grid = [[0u8; SUDOKU_SIZE]; SUDOKU_SIZE];
        for (i, number_str) in split_str.iter().enumerate() {
            match number_str.parse::<u8>() {
                Ok(num) => {
                    let row = i / SUDOKU_SIZE;
                    let col = i % SUDOKU_SIZE;
                    parsed_grid[row][col] = num;

                    if num == 0 {
                        num_empty += 1;
                    }
                }
                Err(_) => {
                    let msg = format!("Invalid number at index {} - {}", i, number_str);
                    return Err(Error::new(ErrorKind::InvalidData, msg));
                }
            }
        }

        let sudoku = Sudoku {
            grid: parsed_grid,
            num_empty: num_empty,
        };

        return Ok(sudoku);
    }

    fn assign_inplace(&mut self, row: usize, col: usize, num: u8) {
        if self.grid[row][col] != 0 {
            panic!("Assign at {}, {} with {}", row, col, self.grid[row][col]);
        }

        self.grid[row][col] = num;
        self.num_empty -= 1;
    }

    fn is_full(&self) -> bool {
        return self.num_empty == 0;
    }

    fn try_first_obvious_inplace(&mut self) -> bool {
        for r in 0..SUDOKU_SIZE {
            for c in 0..SUDOKU_SIZE {
                if self.grid[r][c] != 0 {
                    continue;
                }

                let mut choices = self.get_choices(r, c);
                if choices.len() == 1 {
                    match choices.next() {
                        Some(num) => {
                            self.assign_inplace(r, c, num);
                            return true;
                        }
                        _ => {}
                    }
                }
            }
        }
        return false;
    }

    fn try_all_obvious_inplace(&mut self) -> bool {
        let mut try_filling = true;
        while try_filling {
            try_filling = self.try_first_obvious_inplace();
            if self.is_full() {
                return true;
            }
        }
        return false;
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        for i in 0..SUDOKU_SIZE {
            let row_vec: Vec<String> = self.grid[i].iter().map(|n| n.to_string()).collect();
            writeln!(f, "{}", row_vec.join(" "))?;
        }
        return Ok(());
    }
}

fn main() -> Result<(), String> {
    if env::args().len() <= 1 {
        return Err(String::from("No filename specified."));
    }

    let args_collection: Vec<String> = env::args().collect();
    let filename = &args_collection[1];
    let result = Sudoku::from_file(filename);
    let mut sudoku: Sudoku;

    match result {
        Err(why) => {
            return Err(format!("Read error - {}", why));
        }
        Ok(parsed_sudoku) => sudoku = parsed_sudoku,
    }

    print!("Sudoku read:");
    println!("{}", sudoku);
    sudoku.try_all_obvious_inplace();
    print!("Sudoku after:");
    println!("{}", sudoku);

    return Ok(());
}
