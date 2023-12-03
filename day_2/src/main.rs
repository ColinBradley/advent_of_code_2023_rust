fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

fn solve_part_1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let [game, rounds] = line.split(':').collect::<Vec<_>>()[..] else {
                panic!("Invalid line");
            };

            let is_valid = rounds.split(';').all(|round| {
                round.split(',').all(|grab| {
                    let [count, color] = grab.trim().split(' ').collect::<Vec<_>>()[..] else {
                        panic!("Invalid grab");
                    };

                    let count = count.parse::<u32>().unwrap();

                    match color {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        _ => true,
                    }
                })
            });

            if is_valid {
                let game_id = game.split(' ').last().unwrap().parse::<u32>().unwrap();
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let grabs = line
                .split(':')
                .last()
                .unwrap()
                .split([';', ','])
                .map(|grab| {
                    let [count, color] = grab.trim().split(' ').collect::<Vec<_>>()[..] else {
                        panic!("Invalid grab");
                    };

                    let count = count.parse::<u32>().unwrap();

                    (color, count)
                });

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for (color, count) in grabs {
                match color {
                    "red" => red = count.max(red),
                    "green" => green = count.max(green),
                    "blue" => blue = count.max(blue),
                    _ => (),
                }
            }

            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod day_2_tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve_part_1(input), 8);
    }

    #[test]
    fn part_1() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_part_1(input), 2169);
    }

    #[test]
    fn example_2() {
        let input = include_str!("./example1.txt");
        assert_eq!(solve_part_2(input), 2286);
    }

    #[test]
    fn part_2() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_part_2(input), 60948);
    }
}
