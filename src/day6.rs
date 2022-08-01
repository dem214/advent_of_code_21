#[derive(Debug, Clone, Copy)]
struct Fish {
    timer: u8,
}

impl Fish {
    fn tick(&mut self) -> Option<Fish> {
        if self.timer == 0 {
            self.timer = 6;
            Some(Fish { timer: 8 })
        } else {
            self.timer -= 1;
            None
        }
    }
}

#[derive(Default)]
struct Swarm {
    fishes: Vec<Fish>,
}

impl Swarm {
    fn tick(&mut self) {
        let mut new_fihes = vec!();
        for fish in &mut self.fishes[..] {
            let res = fish.tick();
            if res.is_some() {
                new_fihes.push(res.unwrap());
            }
        }
        self.fishes.append(&mut new_fihes);
    }

    fn simulate(&mut self, num_of_days: u32) {
        for _ in 0..num_of_days {
            self.tick()
        }
    }

    fn count_fishes(&self) -> usize {
        self.fishes.len()
    }
}

use std::fs;

pub fn part1() {
    let file_content = fs::read_to_string("inputs/input6.txt").expect("cannot read the file");
    let fishes = file_content
        .split(",")
        .filter_map(|raw| raw.parse::<u8>().ok())
        .map(|digit| Fish { timer: digit })
        .collect::<Vec<Fish>>();
    let mut swarm = Swarm { fishes: fishes };
    swarm.simulate(80);
    println!("{}", swarm.count_fishes());
}

use memoize::memoize;

#[memoize]
fn count_sons(days_to_grow: u64) -> u64{
    let mut i = 9;
    let mut sons = 1u64;
    while i < days_to_grow {
        sons = sons.checked_add(count_sons(days_to_grow-i)).expect("sons overflow");
        i += 7;
    }
    sons
}

pub fn part2() {
    let file_content = fs::read_to_string("inputs/input6.txt").expect("cannot read the file");
    let init_ages = file_content
        .split(",")
        .filter_map(|raw| raw.parse::<u8>().ok())
        .map(|i| i as i64 - 9)
        .collect::<Vec<i64>>();
    let mut total = 0u64;
    for age in init_ages{
        total = total.checked_add(count_sons((256 - age) as u64)).expect("Overflow");
    }
    println!("{}", total)
}