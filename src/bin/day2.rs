fn main() {
    let input = include_str!("../../assets/day2.txt");
    let score1 = part_1(input);
    println!("Part 1 score: {score1}");
    let score2 = part_2(input);
    println!("Part 2 score: {score2}");
}

fn part_1(strategy_guide: &str) -> isize {
    strategy_guide.lines().map(|round| -> isize {
        let chars = round.as_bytes();
        let opponent = chars[0] - 64;
        let you = chars[2] - 87;
        // draw - 0, win - 1, loss - 2
        let result: isize = (you as isize - opponent as isize).rem_euclid(3);

        you as isize + match result {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => unreachable!("rem_euclid(3) should always return 0-2")
        }
    }).sum()
}

fn part_2(strategy_guide: &str) -> isize {
    strategy_guide.lines().map(|round| -> isize {
        let chars = round.as_bytes();
        let opponent = chars[0] - 64;
        // loss - 1, draw - 2, win - 3
        let result = chars[2] - 87;
        let you = (3 + opponent as isize - (2 * result as isize)).rem_euclid(3) + 1;

        you as isize + match result {
            2 => 3,
            3 => 6,
            1 => 0,
            _ => unreachable!("result should never be another value")
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let input = "A Y
B X
C Z";
        let output = part_1(input);

        assert_eq!(output, 15);
    }

    #[test]
    fn part_2_should_work() {
        let input = "A Y
B X
C Z";
        let output = part_2(input);

        assert_eq!(output, 12);
    }
}