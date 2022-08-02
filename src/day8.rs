use std::collections::{BTreeSet, HashMap};
use std::fs;

#[derive(Debug, Default)]
struct Entry<'a> {
    patterns: [&'a str; 10],
    output: [&'a str; 4],
    a_seg: char,
    c_seg: char,
    f_seg: char,
}

impl<'a> Entry<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut pipesplitted = s.split('|');
        let left = pipesplitted
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&'a str>>();
        let right = pipesplitted
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&'a str>>();
        Entry {
            patterns: left.try_into().unwrap(),
            output: right.try_into().unwrap(),
            a_seg: ' ',
            c_seg: ' ',
            f_seg: ' ',
        }
    }

    fn lens_of_output(&self) -> [usize; 4] {
        self.output
            .iter()
            .map(|s| s.len())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap()
    }
}

struct EntryParser {
    mapping: HashMap<BTreeSet<char>, u8>,
}

impl EntryParser {
    fn new(patterns: [&str; 10]) -> Self {
        let mut set1: BTreeSet<char> = BTreeSet::new();
        let mut set4: BTreeSet<char> = BTreeSet::new();
        let mut set7: BTreeSet<char> = BTreeSet::new();
        let mut set8: BTreeSet<char> = BTreeSet::new();
        for pat in patterns {
            match pat.len() {
                2 => set1 = BTreeSet::from_iter(pat.chars()),
                4 => set4 = BTreeSet::from_iter(pat.chars()),
                3 => set7 = BTreeSet::from_iter(pat.chars()),
                7 => set8 = BTreeSet::from_iter(pat.chars()),
                also => {}
            }
        }

        let mut set0: BTreeSet<char> = BTreeSet::new();
        let mut set6: BTreeSet<char> = BTreeSet::new();
        let mut set9: BTreeSet<char> = BTreeSet::new();
        let mut c_seg: char = ' ';

        for pat in patterns.iter().filter(|p| p.len() == 6) {
            // 0, 6, 9
            let pat_set = BTreeSet::from_iter(pat.chars());
            if pat_set.is_superset(&set4) {
                set9 = pat_set;
                continue;
            } else if pat_set.is_superset(&set1) && !pat_set.is_superset(&set4) {
                set0 = pat_set;
                continue;
            } else if !pat_set.is_superset(&set1) {
                c_seg = set1.difference(&pat_set).next().unwrap().to_owned();
                set6 = pat_set;
                continue;
            } else {
                panic!("Unknown 6 elem pattern")
            }
        }

        let mut set3: BTreeSet<char> = BTreeSet::new();
        let mut set2: BTreeSet<char> = BTreeSet::new();
        let mut set5: BTreeSet<char> = BTreeSet::new();

        for pat in patterns.iter().filter(|p| p.len() == 5) {
            // 2, 3, or 5
            let pat_set = BTreeSet::from_iter(pat.chars());
            if pat_set.is_superset(&set7) {
                set3 = pat_set;
                continue;
            } else if pat_set.contains(&c_seg) {
                set2 = pat_set;
                continue;
            } else if !pat_set.contains(&c_seg) {
                set5 = pat_set;
                continue;
            } else {
                panic!("Unknown 5 seg pattern");
            }
        }

        EntryParser {
            mapping: HashMap::from([
                (set0, 0),
                (set1, 1),
                (set2, 2),
                (set3, 3),
                (set4, 4),
                (set5, 5),
                (set6, 6),
                (set7, 7),
                (set8, 8),
                (set9, 9),
            ]),
        }
    }

    fn parse(&self, value: &str) -> u8 {
        self.mapping[&BTreeSet::from_iter(value.chars())]
    }

    fn parse_entry(&self, entry: &Entry<'_>) -> u16 {
        self.parse(entry.output[0]) as u16 * 1000
            + self.parse(entry.output[1]) as u16 * 100
            + self.parse(entry.output[2]) as u16 * 10
            + self.parse(entry.output[3]) as u16
    }
}

pub fn part1() {
    let file_content = fs::read_to_string("inputs/input8.txt").unwrap();
    let entries: Vec<Entry> = file_content
        .split('\n')
        .map(|raw| Entry::from_str(raw))
        .collect();
    let lens_of_output = entries
        .iter()
        .map(|entry| entry.lens_of_output())
        .flatten()
        .collect::<Vec<usize>>();
    let those_digits_count = lens_of_output
        .into_iter()
        .filter(|&digit| digit == 2 || digit == 4 || digit == 3 || digit == 7)
        .count();
    println!("{those_digits_count}");
}

pub fn part2() {
    let file_content = fs::read_to_string("inputs/input8.txt").unwrap();
    let entries: Vec<Entry> = file_content
        .split('\n')
        .map(|raw| Entry::from_str(raw))
        .collect();
    let founded = entries
        .iter()
        .map(|entry| {
            let entry_parser = EntryParser::new(entry.patterns);
            entry_parser.parse_entry(&entry)
        })
        .fold(0u32, |acc, value| acc + value as u32);
    assert_eq!(founded, 968175);
}
