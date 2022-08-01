use std::fs;

fn _part_one() {
    let input_content = fs::read_to_string("input.txt").expect("Cannot read the file");
    let mut depths = input_content
        .split('\n')
        .filter_map(|item| item.parse::<u32>().ok());
    // let depths = get_depths_iter();
    let mut increases = 0u32;
    let mut prev_depth = depths.next().expect("Have no depths at all");
    for depth in depths {
        if depth > prev_depth {
            increases += 1;
        }
        prev_depth = depth;

    }
    assert_eq!(increases, 1288);
    println!("Ok");
}

fn _part_two() {
    let input_content = fs::read_to_string("input.txt").expect("Cannot read the file");
    let depths = input_content
        .split('\n')
        .filter_map(|item| item.parse::<u32>().ok());

    let depths: Vec<u32> = depths.collect();
    let mut sums = depths[..].windows(3)
        .map::<u32, _>(|item| item.iter().sum());
    let mut increases = 0u32;
    let mut prev_sum = sums.next().expect("not eniugh sums");
    for sum in sums {
        if sum > prev_sum {
            increases += 1;
        }
        prev_sum = sum;
    }

    assert_eq!(increases, 1311);
    println!("Ok");

}

pub fn _run() {
    _part_one();
    _part_two();
}