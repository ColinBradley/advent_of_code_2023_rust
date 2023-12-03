fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<Vec<_>>();
            (digits.first().unwrap().to_string() + &digits.last().unwrap().to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let digits = (0..chars.len())
                .filter_map(|index| {
                    let character = chars[index];

                    if character.is_ascii_digit() {
                        Some(character)
                    } else {
                        let rest = &line[index..];
                        if rest.starts_with("one") {
                            Some('1')
                        } else if rest.starts_with("two") {
                            Some('2')
                        } else if rest.starts_with("three") {
                            Some('3')
                        } else if rest.starts_with("four") {
                            Some('4')
                        } else if rest.starts_with("five") {
                            Some('5')
                        } else if rest.starts_with("six") {
                            Some('6')
                        } else if rest.starts_with("seven") {
                            Some('7')
                        } else if rest.starts_with("eight") {
                            Some('8')
                        } else if rest.starts_with("nine") {
                            Some('9')
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<_>>();

            (digits.first().unwrap().to_string() + &digits.last().unwrap().to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod day_1_tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve_part_1(input), 142);
    }

    #[test]
    fn part_1() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_part_1(input), 54081);
    }

    #[test]
    fn example_2() {
        let input = include_str!("./example2.txt");
        assert_eq!(solve_part_2(input), 281);
    }

    #[test]
    fn example_3() {
        let input = "twone";
        assert_eq!(solve_part_2(input), 21);
    }

    #[test]
    fn part_2() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_part_2(input), 54649);
    }
}
