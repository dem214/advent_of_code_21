use std::fs;
use std::collections::HashSet;

#[derive(Debug, Default)]
struct Entry<'a>{
    patterns: [&'a str; 10],
    output: [&'a str; 4],
    a_seg: char,
    c_seg: char,
    e_seg: char,
    f_seg: char,
    g_seg: char,
}

impl<'a> Entry<'a> {

    fn from_str(s: &'a str) -> Self {
        let mut pipesplitted = s.split('|');
        let left = pipesplitted.next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&'a str>>();
        let right = pipesplitted.next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&'a str>>();
        Entry {
            patterns: left.try_into().unwrap(),
            output: right.try_into().unwrap(),
            a_seg: ' ',
            c_seg: ' ',
            e_seg: ' ',
            f_seg: ' ',
            g_seg: ' '
        }
    }

    fn lens_of_output(&self) -> [usize; 4] {
        self.output.iter()
            .map(|s| s.len())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap()
    }
    fn element_of_length(&self, n: usize) -> Option<&str> {
        for pat in self.patterns {
            if pat.len() == n {
                return Some(pat)
            }
        }
        None
    }

    fn elements_of_length(&self, n: usize) -> Vec<&str> {
        self.patterns.iter()
            .filter(|pat| pat.len() == n)
            .map(|&s| s)
            .collect()
    }

    fn pat_of_1(&self) -> &str {
        self.element_of_length(2).unwrap()
    }

    fn pat_of_4(&self) -> &str {
        self.element_of_length(4).unwrap()
    }

    fn pat_of_7(&self) -> &str {
        self.element_of_length(3).unwrap()
    }
    
    fn pat_of_8(&self) -> &str {
        self.element_of_length(7).unwrap()
    }

    fn find_a_seg(&mut self) {
        let set1: HashSet<char> = HashSet::from_iter(self.pat_of_1().chars());
        let set7: HashSet<char> = HashSet::from_iter(self.pat_of_7().chars());
        let mut diff = set7.difference(&set1);
        assert_eq!(diff.clone().collect::<HashSet<_>>().len(), 1);
        self.a_seg = *diff.next().unwrap();
    }

    fn find_c_seg(&mut self) {
        let set1: HashSet<char> = HashSet::from_iter(self.pat_of_1().chars());
        let founded = self.elements_of_length(6).iter()
            .filter_map(|&pat| {
                let pat_hash = HashSet::from_iter(pat.chars());
                if !pat_hash.is_superset(&set1) {
                    Some(set1.difference(&pat_hash).next().unwrap().to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<char>>();
        assert_eq!(founded.len(), 1);
        self.c_seg = founded[0];
    }

    fn find_e_seg(&mut self) {
        let mut set4x: HashSet<char> = HashSet::from_iter(self.pat_of_4().chars());
        let fullset: HashSet<char> = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        set4x.insert(self.a_seg);
        let founded = self.elements_of_length(6).iter()
            .filter_map(|&pat| {
                let pat_hash = HashSet::from_iter(pat.chars());
                if pat_hash.is_superset(&set4x) {
                    Some(fullset.difference(&pat_hash).next().unwrap().to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<char>>();
        assert_eq!(founded.len(), 1);
        self.e_seg = founded[0];

    }


    fn find_f_seg(&mut self) {
        let mut set1: HashSet<char> = HashSet::from_iter(self.pat_of_1().chars());
        assert!(set1.remove(&self.c_seg));
        assert_eq!(set1.len(), 1);
        self.f_seg = set1.into_iter().next().unwrap();
    }

    
    // fn find_e_seg(&mut self) {
    //     let set9: HashSet<char> = HashSet::from_iter(self.pat_of_9().chars());
    //     let fullset: HashSet<char> = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);

    //     let mut diff = fullset.difference(&set9);
    //     self.g_seg = *diff.next().unwrap();
    // }

    fn find_all(&mut self) {
        self.find_a_seg();
        self.find_c_seg();
        self.find_f_seg();
        self.find_e_seg();
    }

    fn is_9(&self, value: &str) -> bool {
        if !value.len() == 6 {
            return false;
        }
        let value_set = HashSet::from_iter(value.chars());
        let fullset: HashSet<char> = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        fullset.difference(&value_set).map(|&i| i).collect::<Vec<char>>().eq(&Vec::from([self.e_seg, ]))
    }

    fn is_3(&self, value: &str ) -> bool {
        if !value.len() == 5 {  // 2, 3, or 5
            return false
        }
        let set7: HashSet<char> = HashSet::from_iter(self.pat_of_1().chars());
        let value_set = HashSet::from_iter(value.chars());
        value_set.is_superset(&set7)
    }

    fn is_0(&self, value: &str ) -> bool {
        if !value.len() == 6 {
            return false
        }
        let value_set = HashSet::from_iter(value.chars());
        value_set.is_superset(&HashSet::from([self.a_seg, self.c_seg, self.f_seg, self.e_seg]))
    }

    fn is_6(value: &str) -> bool {
        value.len() == 6
    }

    fn is_2(&self, value: &str) -> bool {
        let value_set: HashSet<char> = HashSet::from_iter(value.chars());
        value_set.contains(&self.c_seg)
    }


    fn parse_output(&self, value: &str) -> u8 {
        match value {
            one if value.len() == 2 => 1,
            four if value.len() == 4 => 4,
            seven if value.len() == 3 => 7,
            eight if value.len() == 7 => 8,
            nine if self.is_9(value) => 9,
            three if self.is_3(value) => 3,
            zero if self.is_0(value) => 0,
            six if Entry::is_6(value) => 6,
            two if self.is_2(value) => 2,
            five => 5,

            // _ => panic!("Cannot parse: {value} for {:?}", self)

        }
    }
    fn parsed_output(&self) -> [u8; 4] {
        self.output.iter()
            .map(|s| self.parse_output(s))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }

}

fn merge_digits(digits: [u8; 4]) -> u16 {
    digits[0] as u16 * 1000
        + digits[1] as u16 * 100
        + digits[2] as u16 * 10
        + digits[3] as u16
}

pub fn part1() {
    let file_content = fs::read_to_string("inputs/input8.txt").unwrap();
    let entries: Vec<Entry> = file_content.split('\n')
        .map(|raw| Entry::from_str(raw))
        .collect();
    let lens_of_output = entries.iter()
        .map(|entry| entry.lens_of_output())
        .flatten()
        .collect::<Vec<usize>>();
    let those_digits_count = lens_of_output.into_iter()
        .filter(|&digit| digit == 2 || digit == 4 || digit == 3 || digit == 7 )
        .count();
    println!("{those_digits_count}");
}

pub fn part2() {
    let file_content = fs::read_to_string("inputs/input8.txt").unwrap();
    let mut entries: Vec<Entry> = file_content.split('\n')
        .map(|raw| Entry::from_str(raw))
        .collect();

    for entry in entries.iter_mut() {
        entry.find_all();
    }
    let founded = entries.iter()
        .map(|entry| entry.parsed_output())
        .map(|raw| merge_digits(raw))
        .fold(0u64, |acc, value| acc + value as u64);
    println!("{:#?}", founded)

    
}