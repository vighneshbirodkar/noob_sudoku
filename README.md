# noob_sudoku
A sudoku solver written in rust.

[![asciicast](https://asciinema.org/a/NlkTB2jOvvs0HLbtq62yy1fXe.svg)](https://asciinema.org/a/NlkTB2jOvvs0HLbtq62yy1fXe)

## Command line usage
```shell
cargo run ./path/to/sudoku.txt
```

## File format
Numbers in the sudoku should be written in a text file and separated by whitespace. The unknown digits are designated by `0`.

### Example
```
0 0 0 0 0 3 0 1 7 
0 1 5 0 0 9 0 0 8 
0 6 0 0 0 0 0 0 0 
1 0 0 0 0 7 0 0 0 
0 0 9 0 0 0 2 0 0 
0 0 0 5 0 0 0 0 4 
0 0 0 0 0 0 0 2 0 
5 0 0 6 0 0 3 4 0 
3 4 0 2 0 0 0 0 0 
```
