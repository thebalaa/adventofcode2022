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
    assert_eq!(parsed[0], vec![1000, 2000, 3000]);
    assert_eq!(parsed.len(), 5);
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut output = vec![vec![]];
    for line in input.lines() {
        if line.is_empty() {
            output.push(vec![])
        } else {
            output
                .last_mut()
                .unwrap()
                .push(line.parse::<u32>().unwrap());
        }
    }
    output
}

#[test]
fn can_solve() {
    let data = include_str!("../example.txt");
    let parsed = parse(data);
    assert_eq!(solve(&parsed), 24000);
}

fn solve(input: &Vec<Vec<u32>>) -> u32 {
    let mut total = 0;
    for elv in input {
        let etotal = elv.iter().sum();
        if total < etotal {
            total = etotal;
        }
    }
    total
}

fn solve2(input: &Vec<Vec<u32>>) -> u32 {
    let mut total: Vec<_> = input.iter().map(|list| list.iter().sum()).collect();
    total.sort();
    total.reverse();

    total.iter().take(3).sum()
}
