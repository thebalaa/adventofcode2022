use std::collections::BTreeSet;

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file).unwrap();
    let parsed = parse(&data);
    let solved = solve(&parsed);
    println!("{solved}");
    let solved = solve2(&parsed);
    println!("{solved}");
}

#[test]
fn can_parse() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(
        parsed[0],
        "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect::<Vec<_>>()
    );
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(solve(&parsed), 157);
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect()).collect()
}

fn solve(data: &Vec<Vec<char>>) -> u32 {
    data.iter()
        .map(|line| {
            let middle = line.len() / 2;
            let left: BTreeSet<char> = line[0..middle].iter().copied().collect();
            let right: BTreeSet<char> = line[middle..].iter().copied().collect();
            right
                .intersection(&left)
                .copied()
                .map(priority)
                .sum::<u32>()
        })
        .sum()
}

#[test]
fn can_solve2() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(solve2(&parsed), 70);
}
fn solve2(data: &Vec<Vec<char>>) -> u32 {
    data.chunks(3)
        .map(|lines| {
            lines
                .iter()
                .map(|line| line.iter().copied().collect::<BTreeSet<char>>())
                .fold(None as Option<BTreeSet<char>>, |acc, set| match acc {
                    None => Some(set),
                    Some(acc) => Some(acc.intersection(&set).copied().collect()),
                })
                .unwrap()
                .into_iter()
                .map(priority)
                .sum::<u32>()
        })
        .sum()
}

fn priority(input: char) -> u32 {
    match input {
        'a'..='z' => input as u32 - 'a' as u32 + 1,
        'A'..='Z' => input as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
}
