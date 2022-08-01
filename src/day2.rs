use std::fs;

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn command(&mut self, direction: &str, value: i32) {
        match direction {
            "down" => self.aim += value,
            "up" => self.aim -= value,
            "forward" => {
                self.horizontal += value;
                self.depth += self.aim * value;
            }
            unparsed => println!("Some unparsed params: {}: {}", unparsed, value),
        }
    }

    fn finalize(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn parse_line(line: &str) -> Option<(&str, i32)> {
    let duplet: Vec<&str> = line.split(' ').collect();
    if duplet.len() != 2 {
        println!("not two words in line: {:?}", duplet);
        return None
    }
    let part2 = duplet[1].parse::<i32>().ok()?;
    Some((duplet[0], part2))
}

pub fn _part1() {
    let input_content = fs::read_to_string("input2.txt").expect("Cannot read the file");
    let mut position = Position::default();
    input_content.split('\n')
        .filter_map(|line| parse_line(line))
        .for_each(|duplet| position.command(duplet.0, duplet.1));
    println!("{:?}", &position.finalize());
    println!("ok");
}