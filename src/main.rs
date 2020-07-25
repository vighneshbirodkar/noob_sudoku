use std::env;
use std::fs;


fn read_sudoku(filename: &String) -> std::io::Result<()> {
    let _grid = [[0; 4]; 2];
    let string = fs::read_to_string(filename)?;
    println!("{}", string);
    return Ok(());
}

fn main() -> Result<(), String> {

    if env::args().len() <= 1 {
	return Err(String::from("No filename specified."));
    }

    let args_collection:Vec<String> = env::args().collect();
    let filename = &args_collection[1];
    let result = read_sudoku(filename);

    match result {
        Err(why) => {
            return Err(format!("Read error - {}", why));
        }
        _ => ()
    }

    return Ok(());
        
}
