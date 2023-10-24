fn main() {
    let input = include_str!("../../assets/day4.txt");
    let output1 = part_1(input);
    println!("Part 1 results: {output1}");

    let output2 = part_2(input);
    println!("Part 2 results: {:?}", output2);
}

fn part_1(input: &str) -> usize {
    parse_input(input).into_iter().flat_map(
        |(range1, range2)|
            if contains(range1, range2) || contains(range2, range1) {
                Some(())
            } else {
                None
            }
    ).count()
}

fn contains(range1: (usize, usize), range2: (usize, usize)) -> bool {
    range1.0 <= range2.0 && range1.1 >= range2.1
}

fn part_2(input: &str) -> usize {
    parse_input(input).into_iter().flat_map(
        |(range1, range2)|
            if inside(range1.0, range2) ||
                inside(range1.1, range2) ||
                inside(range2.0, range1) ||
                inside(range2.1, range1) {
                    Some(())
                } else {
                    None
                }
    ).count()
}

fn inside(value: usize, range: (usize, usize)) -> bool {
    value >= range.0 && value <= range.1
}

trait CollecTupleExt: Iterator {
    fn collect_tuple_2(self) -> (Self::Item, Self::Item)
    where
        Self: Sized,
        Self::Item: Copy
    {
        let items = self.collect::<Vec<_>>();

        (items[0], items[1])
    }
}

impl<I: Iterator> CollecTupleExt for I {}

fn parse_input(input: &str) -> Vec<((usize, usize),(usize, usize))> {
    input.lines().into_iter().map(|line| {
        line.split(',').map(|range| {
            range.split('-').flat_map(str::parse::<usize>).collect_tuple_2()
        }).collect_tuple_2()
    }).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let output = part_1(input);

        assert!(output == 2);
    }

    #[test]
    fn part_2_should_work() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let output = part_2(input);

        assert!(output == 4);
    }
}