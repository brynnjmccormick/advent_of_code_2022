use std::collections::HashSet;

fn main() {
    let input = include_str!("../../assets/day3.txt");
    let score1 = part_1(input);
    println!("Part 1 result: {score1}");
    let score2 = part_2(input);
    println!("Part 2 result: {score2}");
}

fn part_1(input: &str) -> usize {
    input.lines().map(|line| {
        let half = line.len() / 2;
        let first_half = line.bytes().take(half).collect::<HashSet<_>>();
        let c = line.bytes().skip(half).find(|b| first_half.contains(&b)).unwrap();

        (if c <= 90 { c - 38 } else { c - 96 }) as usize
    }).sum()
}

fn part_2(input: &str) -> usize {
    input.lines().zip((0..).map(|i| i% 3)).fold(Vec::new(), |mut acc, (line, i)| {
        if i == 0 {
            acc.push(vec![line]);
        } else {
            acc.last_mut().unwrap().push(line);
        }

        acc
    }).iter().map(|group| {
        let mut sets = group.iter().map(|line| line.bytes().collect::<HashSet<_>>()).collect::<Vec<_>>();

        // https://stackoverflow.com/a/65175232
        let mut result = sets.pop().unwrap();
        result.retain(|item| {
            sets.iter().all(|set| set.contains(item))
        });

        let c = *result.iter().next().expect("group of three to share an item");

        (if c <= 90 { c - 38 } else { c - 96 }) as usize
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let output = part_1(input);

        assert_eq!(157, output);
    }

    #[test]
    fn part_2_should_work() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let output = part_2(input);

        assert_eq!(70, output);
    }
}