use std::fs;

#[derive(Debug)]
struct Cell {
    value: u8,
    checked: bool
}

impl Cell {
    fn new(value: u8) -> Cell {
        Cell {value, checked: false}
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
         match self.checked {
            true => format!("@{:2}", self.value),
            false => format!(".{:2}", self.value)
        }
    }
}

struct Board {
    cells: [Cell;25]
}

impl Board {
    fn row(&self, number: usize) -> [&Cell;5] {
        let first = number * 5;
        [&(self.cells[first]), &(self.cells[first+1]), &(self.cells[first+2]), &(self.cells[first+3]), &(self.cells[first+4])]
    }

    fn coll(&self, number: usize) -> [&Cell;5] {
        [&(self.cells[number]), &(self.cells[number+5]), &(self.cells[number+10]), &(self.cells[number+15]), &(self.cells[number+20])]

    }

    fn check_number(&mut self, value: &u8) {
        for cell in self.cells[..].iter_mut() {
            if cell.value == *value {
                cell.checked = true;
            }
        }
    }
    
    fn is_winner(&self) -> bool {
        for i in 0..5 {
            let row = self.row(i);
            if row.iter().all(|cell| cell.checked) {
                return true;
            };
            let coll = self.coll(i);
            if coll.iter().all(|cell| cell.checked) {
                return true;
            };
        };
        false
    }
    fn unchecked_sum(&self) -> u32 {
        self.cells[..].iter()
            .filter(|cell| !cell.checked)
            .map(|cell| cell.value as u32)
            .sum()
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        (0..5).map(|number| self.row(number))
            .map(|row| {
                row.iter().map(|cell| (*cell).to_string())
                .reduce(|accum, item| accum + " " + &item).expect("Yolo1")
            }).reduce(|accum, item| accum + "\n" + &item).expect("Yolo2")
    }
}

pub fn _part1() {
    let file_content: String = fs::read_to_string("input4.txt").unwrap();
    let file_lines = file_content.split("\n").collect::<Vec<&str>>();

    let numbers_input = file_lines[0].split(",")
        .filter_map(|raw| raw.parse::<u8>().ok());

    let mut boards: Vec<Board> = Vec::with_capacity(100);

    let mut numbers_buf: Vec<u8> = Vec::with_capacity(25);
    
    for &line in &file_lines[2..] {
        if line == "" {continue}
        let mut digits: Vec<u8> = line.split(" ")
            .filter_map(|raw| raw.parse::<u8>().ok())
            .collect()
        ;
        assert_eq!(digits.len(), 5);
        numbers_buf.append(&mut digits);
        if numbers_buf.len() == 25 {
            let cells: Vec<Cell> = numbers_buf[0..25].iter().map(|u| Cell::new(*u)).collect();
            boards.push(Board{cells: cells.try_into().unwrap()});
            numbers_buf = Vec::with_capacity(25);
        }
    };
    for number in numbers_input {
        for ref mut board in &mut boards {
            board.check_number(&number);
            if board.is_winner() {
                println!("{}", board.to_string());
                dbg!(number);
                println!("{}", board.unchecked_sum());
                println!("{}", board.unchecked_sum() * number as u32);
                return;
            }
        }
    }
}
pub fn part2() {
    let file_content: String = fs::read_to_string("input4.txt").unwrap();
    let file_lines = file_content.split("\n").collect::<Vec<&str>>();

    let numbers_input = file_lines[0].split(",")
        .filter_map(|raw| raw.parse::<u8>().ok());

    let mut boards: Vec<Board> = Vec::with_capacity(100);

    let mut numbers_buf: Vec<u8> = Vec::with_capacity(25);
    
    for &line in &file_lines[2..] {
        if line == "" {continue}
        let mut digits: Vec<u8> = line.split(" ")
            .filter_map(|raw| raw.parse::<u8>().ok())
            .collect();
        assert_eq!(digits.len(), 5);
        numbers_buf.append(&mut digits);
        if numbers_buf.len() == 25 {
            let cells: Vec<Cell> = numbers_buf[0..25].iter().map(|u| Cell::new(*u)).collect();
            boards.push(Board{cells: cells.try_into().unwrap()});
            numbers_buf = Vec::with_capacity(25);
        };
    };
    for number in numbers_input {
        dbg!(boards.len());
        if boards.len() == 1 {
            let ref mut board = boards[0];
            board.check_number(&number);
            if board.is_winner() {
                
                println!("{}", board.unchecked_sum() * number as u32);
                return;
            }
        }
        boards.iter_mut().for_each(|board| board.check_number(&number));
        boards = boards.into_iter()
            .filter(|board| !board.is_winner())
            .collect();
    }
}