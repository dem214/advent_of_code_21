extern crate regex;

use regex::Regex;
use std::fmt;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u16,
    y: u16,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        (self.start.x.max(self.end.x) - self.start.x.min(self.end.x))
            == (self.start.y.max(self.end.y) - self.start.y.min(self.end.y))
    }

    fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical() || self.is_diagonal()
    }

    fn parse(raw: &str) -> Vec<Line> {
        let mut result = Vec::new();
        let re =
            Regex::new(r"(?m)^(?P<start_x>\d+),(?P<start_y>\d+) -> (?P<end_x>\d+),(?P<end_y>\d+)$")
                .unwrap();
        for cap in re.captures_iter(raw) {
            result.push(Line {
                start: Point {
                    x: cap["start_x"].parse().unwrap(),
                    y: cap["start_y"].parse().unwrap(),
                },
                end: Point {
                    x: cap["end_x"].parse().unwrap(),
                    y: cap["end_y"].parse().unwrap(),
                },
            })
        }
        result
    }
    fn marked_points(&self) -> Vec<Point> {
        let mut result = vec![self.start];
        if self.is_horizontal() {
            let y = self.start.y;
            let low_x = self.start.x.min(self.end.x);
            let high_x = self.start.x.max(self.end.x);
            for x in low_x + 1..high_x {
                result.push(Point { x, y });
            }
        } else if self.is_vertical() {
            let x = self.start.x;
            let low_y = self.start.y.min(self.end.y);
            let high_y = self.start.y.max(self.end.y);
            for y in low_y + 1..high_y {
                result.push(Point { x, y });
            }
        } else if self.is_diagonal() {
            let direction = ((self.start.x > self.end.x) && (self.start.y > self.end.y))
                || ((self.start.x < self.end.x) && (self.start.y < self.end.y));
            if direction {
                for i in 1..self.start.x.max(self.end.x) - self.start.x.min(self.end.x) {
                    result.push(Point {
                        x: self.start.x.min(self.end.x) + i,
                        y: self.start.y.min(self.end.y) + i,
                    });
                }
            } else {
                for i in 1..self.start.x.max(self.end.x) - self.start.x.min(self.end.x) {
                    result.push(Point {
                        x: self.start.x.min(self.end.x) + i,
                        y: self.start.y.max(self.end.y) - i,
                    });
                }
            }
        }
        result.push(self.end);
        result
    }
}

struct Board {
    checked_points: Vec<Point>,
}

impl Board {
    fn print(&self) {
        for y in 0..self.y_dim() + 1 {
            for x in 0..self.x_dim() + 1 {
                print!("{}", self.num_of_lines(x, y))
            }
            print!("\n")
        }
    }
}

impl Default for Board {
    fn default() -> Board {
        Board {
            checked_points: Vec::new(),
        }
    }
}

impl Board {
    fn x_dim(&self) -> u16 {
        self.checked_points
            .iter()
            .map(|point| point.x)
            .max()
            .unwrap_or(0)
    }
    fn y_dim(&self) -> u16 {
        self.checked_points
            .iter()
            .map(|point| point.y)
            .max()
            .unwrap_or(0)
    }
    fn num_of_lines(&self, x: u16, y: u16) -> u16 {
        let matched_points: Vec<&Point> = self
            .checked_points
            .iter()
            .filter(|point| point.x == x && point.y == y)
            .collect();
        matched_points.len() as u16
    }
    fn num_of_overlaped_points(&self) -> usize {
        let mut counter = std::collections::HashMap::new();
        for point in &self.checked_points {
            *counter.entry(point).or_insert(0u32) += 1;
        }
        counter
            .values()
            .filter(|count| **count > 1)
            .collect::<Vec<&u32>>()
            .len()
    }
}

pub fn part1() {
    let file_content = fs::read_to_string("inputs/input5.txt").unwrap();
    let lines = Line::parse(&file_content);
    let filterd_lines = lines
        .into_iter()
        .filter(|line| line.is_straight())
        .collect::<Vec<Line>>();
    let mut board = Board::default();
    for line in filterd_lines {
        for point in line.marked_points() {
            board.checked_points.push(point);
        }
    }
    println!("{}", board.num_of_overlaped_points())
}
