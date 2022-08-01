use std::fs;

fn median(data: &[u16]) -> u16 {
    let size = data.len();
    println!("{size}");

    match size % 2  {
        even if even % 2 == 0 => {
            println!("even");
            let first_median = select(data, (size / 2) - 1);
            let second_median = select(data, size / 2);
            (first_median + second_median) / 2
        },
        odd => select(data, size / 2)
    }
}

fn select(data: &[u16], k: usize) -> u16 {
    let (left, pivot, right) = partition(data);

    let pivot_index = left.len();

    use std::cmp::Ordering::*;
    match pivot_index.cmp(&k) {
        Equal => pivot,
        Greater => select(&left, k),
        Less => select(&right, k - (pivot_index + 1))
    }
}

fn partition(data: &[u16]) -> (Vec<u16>, u16, Vec<u16>) {
    let (pivot_slice, tail) = data.split_at(1);
    let pivot = pivot_slice[0];
    let (mut left, mut right) = (vec![], vec![]);
    for item in tail.into_iter() {
        if item < &pivot {
            left.push(*item)
        } else {
            right.push(*item)
        }
    }
    (left, pivot, right)
}

fn count_fuel(data: &[u16], target: &u16) -> u64 {
    data.into_iter()
        .fold(0u64, |acc, point| {
            acc + (*point as i32 - *target as i32).abs() as u64
        })
}

pub fn part1 () {
    let file_content = fs::read_to_string("inputs/input7.txt").unwrap();
    let dataset = file_content.split(',')
        .filter_map(|raw| raw.parse::<u16>().ok())
        .collect::<Vec<u16>>();
    let median = median(&dataset);
    println!("{median}");
    let fuel = count_fuel(&dataset, &median);
    println!("{fuel}");
}

fn count_fuel_for_move_alg(distance: u16) -> u32 {
    match distance {
        0 => 0,
        1 => 1,
        dist => count_fuel_for_move_alg(dist - 1) + dist as u32
    }
}

fn count_fuel_alg(data: &[u16], target: &u16) -> u64 {
    data.into_iter()
        .fold(0u64, |acc, point| {
            acc + count_fuel_for_move_alg((*point as i32 - *target as i32).abs() as u16) as u64
        })
}

use std::collections::HashMap;

pub fn part2 () {
    let file_content = fs::read_to_string("inputs/input7.txt").unwrap();
    let dataset = file_content.split(',')
        .filter_map(|raw| raw.parse::<u16>().ok())
        .collect::<Vec<u16>>();

    let mut dist_map: HashMap<u16, u64> = HashMap::new();
    for i in 0..*dataset.iter().max().unwrap() + 1 {
        dist_map.insert(i, count_fuel_alg(&dataset, &i));
    }
    let min = dist_map.into_iter()
        .reduce(|acc, item| {
            if item.1 < acc.1 {
                item
            } else {
                acc
            }
        }).unwrap();
    let min_dist = min.1;
    assert_eq!(min_dist, 104149091);
}