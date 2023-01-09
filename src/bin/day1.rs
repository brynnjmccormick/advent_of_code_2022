fn main() {
    let input = include_str!("../../assets/day1.txt");
    let output1 = part_1(input);
    println!("Part 1 results: {output1}");

    let output2 = part_2(input);
    println!("Part 2 results: {:?}", output2);
}

fn part_1(input: &str) -> usize {
    input.split("\n\n").map(|elf_cal| {
        elf_cal
            .lines()
            // We use flat_map here (equivalent to .map().flatten()) because str::parse returns a Result, which
            // implements the Iter trait.
            .flat_map(str::parse::<usize>)
            .sum()
    })
    .max()
    .unwrap()
}

fn part_2(input: &str) -> usize {
    let mut cals = input.split("\n\n").map(|elf_cal| {
        elf_cal
            .lines()
            .flat_map(str::parse::<usize>)
            .sum()
    })
    .collect::<Vec<usize>>();

    cals.sort_by(|a, b| b.partial_cmp(a).unwrap());

    cals[..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    
        let result = part_1(input);
        assert_eq!(24000, result);
    }

    #[test]
    fn part_2_should_work() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    
        let result = part_2(input);
        assert_eq!(45000, result);
    }
}