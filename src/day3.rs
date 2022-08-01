use std::fs;
use std::collections::HashMap;
use std::hash::Hash;

fn most_common_value<T>(values: Vec<T>, if_equal_default: T) -> T 
    where T: Eq + Hash + Copy
{
    let mut counter: HashMap<T, u16> = HashMap::new();
    values.iter().for_each(|c| *counter.entry(*c).or_insert(0) += 1);
    let values: Vec<&u16> = counter.values().collect();
    if *values[0] == *values[1] {
        if_equal_default
    }
    else {
        *counter.iter()
        .max_by_key(|item| item.1)
        .expect("Empty itertor")
        .0
    }
}

fn less_common_value<T>(values: Vec<T>, if_equal_default: T) -> T 
    where T: Eq + Hash + Copy
{
    let mut counter: HashMap<T, u16> = HashMap::new();
    values.iter().for_each(|c| *counter.entry(*c).or_insert(0) += 1);
    let values: Vec<&u16> = counter.values().collect();
    if *values[0] == *values[1] {
        if_equal_default
    }
    else {
        *counter.iter()
        .min_by_key(|item| item.1)
        .expect("Empty itertor")
        .0
    }
}

fn get_nth_char(words: &[&str], number: usize) -> Vec<char> {
    words.iter()
        .filter_map(|word| (word.chars().nth(number)))
        .collect()
}

fn get_input() -> String {
    fs::read_to_string("input3.txt").unwrap()
}

fn invert_bin_word(word: &str) -> String {
    String::from_iter(
        word.chars()
            .map(|ch| match ch {
                '0' => '1',
                '1' => '0',
                another => panic!("{}", format!("Get some unknown char {} in word {}", ch, word))
            }
        )
    )
}

fn filter_bin_words<'a>(words: &[&'a str], by_nth_char: usize, expected: &char) -> Vec<&'a str> {
    words.iter()
        .filter(|word| word.chars().nth(by_nth_char).expect("cannot get char") == *expected)
        .map(|word| *word)
        .collect()
}

pub fn _part1() {
    // let input = get_input();
    let input = get_input();
    let bin_words: Vec<&str> = input.split('\n')
        .filter(|word| word.len() > 0)
        .collect();
    let len_of_word: usize = bin_words[0].len();
    let mut gamma_chars = Vec::new();
    for i in 0..len_of_word {
        gamma_chars.push(most_common_value(get_nth_char(&bin_words[..], i), '1'))
    }
    let gamma_rate_str = String::from_iter(gamma_chars.iter());
    let epsilon_rate_str = invert_bin_word(&gamma_rate_str);
    let gamma_rate = u32::from_str_radix(&gamma_rate_str, 2).unwrap();
    let epsilon_rate = u32::from_str_radix(&epsilon_rate_str, 2).unwrap();
    println!("The answer: {}", gamma_rate * epsilon_rate);
}

fn get_most_common_word<'a>(start_words: &[&'a str], nth_char: usize) -> &'a str {
    let most_common_char = most_common_value(get_nth_char(&start_words, nth_char), '1');
    let end_words = filter_bin_words(&start_words, nth_char, &most_common_char);
    if end_words.len() == 1 {
        end_words[0]
    } else {
        get_most_common_word(&end_words, nth_char + 1)
    }
}

fn get_less_common_word<'a>(start_words: &[&'a str], nth_char: usize) -> &'a str {
    let less_common_char = less_common_value(get_nth_char(&start_words, nth_char), '0');
    let end_words = filter_bin_words(&start_words, nth_char, &less_common_char);
    if end_words.len() == 1 {
        end_words[0]
    } else {
        get_less_common_word(&end_words, nth_char + 1)
    }
}

pub fn part2() {
    let input = get_input();
    let bin_words: Vec<&str> = input.split('\n')
        .filter(|word| word.len() > 0)
        .collect();
    let oxygen_rate_str = get_most_common_word(&bin_words, 0);
    let oxygen_rate = u32::from_str_radix(oxygen_rate_str, 2).unwrap();
    let co2_rate_str = get_less_common_word(&bin_words, 0);
    let co2_rate = u32::from_str_radix(co2_rate_str, 2).unwrap();
    println!("Answer: {}", oxygen_rate * co2_rate);
}
#[test]
fn test_more_common() {
    assert_eq!(most_common_value(vec!('0', '0', '1', '1', '1', '1'), '0'), '1');
    assert_eq!(most_common_value(vec!('0', '0', '0', '0', '0', '1'), '1'), '0');
    assert_eq!(most_common_value(vec!('1', '1', '0', '0'), '1'), '1');
    assert_eq!(most_common_value(vec!('1', '1', '0', '0'), '0'), '0');
}