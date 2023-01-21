fn main() {
    let input = include_str!("../../assets/day3.txt");
    let score1 = part_1(input);
    println!("Part 1 result: {score1}");
    // let score2 = part_2(input);
    // println!("Part 2 result: {score2}");
}

fn part_1(input: &str) -> usize {
    // Not the best data model, but gets parsed in one line
    // [[2, 4], [6, 8]]
    // [[2, 3], [4, 5]]
    // [[5, 7], [7, 9]]
    // [[2, 8], [3, 7]]
    // [[6, 6], [4, 6]]
    // [[2, 6], [4, 8]]
    input.lines().map(|line| {
        let ranges = line.split(",").map(|range| {
            range.split("-").flat_map(str::parse::<usize>).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        // Wow this needs to be simplified.
        if ((ranges[0][0] >= ranges[1][0] && ranges[0][0] <= ranges[1][1]) && 
            (ranges[0][1] >= ranges[1][0] && ranges[0][1] <= ranges[1][1])) ||
            ((ranges[1][0] >= ranges[0][0] && ranges[1][0] <= ranges[0][1]) && 
            (ranges[1][1] >= ranges[0][0] && ranges[1][1] <= ranges[0][1]))  {
                1
            } else {
                0
            }
    }).sum()
}

// fn part_2(input: &str) -> usize {

// }

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

        assert_eq!(2, output);
    }

//     #[test]
//     fn part_2_should_work() {
//         let input = "2-4,6-8
// 2-3,4-5
// 5-7,7-9
// 2-8,3-7
// 6-6,4-6
// 2-6,4-8";

//         let output = part_2(input);

//         assert_eq!(70, output);
//     }
}