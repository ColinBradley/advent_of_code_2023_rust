#![feature(extract_if)]

struct Rectangle {
    top: i32,
    left: i32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new_part(top: usize, left: usize) -> Self {
        Self {
            top: top as i32 - 1,
            left: left as i32 - 1,
            width: 2,
            height: 3,
        }
    }

    fn new_symbol(top: usize, left: usize) -> Self {
        Self {
            top: top as i32,
            left: left as i32,
            width: 1,
            height: 1,
        }
    }

    fn right(&self) -> i32 {
        self.left + self.width as i32
    }

    fn bottom(&self) -> i32 {
        self.top + self.height as i32
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.left < other.right()
            && self.right() > other.left
            && self.top < other.bottom()
            && self.bottom() > other.top
    }
}

struct PartNumber {
    number: u32,
    rectangle: Rectangle,
}

struct Symbol {
    character: char,
    rectangle: Rectangle,
}

fn parse(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers = Vec::<PartNumber>::new();
    let mut symbols = Vec::<Symbol>::new();

    for (y, line) in input.lines().enumerate() {
        let mut active_part_number: Option<PartNumber> = None;

        for (x, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                let part_number = active_part_number.get_or_insert_with(|| PartNumber {
                    number: 0,
                    rectangle: Rectangle::new_part(y, x),
                });

                part_number.number *= 10;
                part_number.number += character.to_string().parse::<u32>().unwrap();
                part_number.rectangle.width += 1;
            } else {
                if let Some(part_number) = active_part_number.take() {
                    part_numbers.push(part_number);
                }

                if character == '.' {
                    continue;
                }

                symbols.push(Symbol {
                    character,
                    rectangle: Rectangle::new_symbol(y, x),
                });
            }
        }

        if let Some(part_number) = active_part_number.take() {
            part_numbers.push(part_number);
        }
    }

    (part_numbers, symbols)
}

fn solve_part_1(input: &str) -> u32 {
    let (mut part_numbers, symbols) = parse(input);

    symbols
        .iter()
        .flat_map(|symbol| {
            part_numbers
                .extract_if(|part_number| part_number.rectangle.overlaps(&symbol.rectangle))
                .map(|adjacent_part_number| adjacent_part_number.number)
                .collect::<Vec<_>>()
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let (mut part_numbers, symbols) = parse(input);

    symbols
        .iter()
        .filter(|s| s.character == '*')
        .map(|gear| {
            let near_numbers = part_numbers
                .extract_if(|part_number| part_number.rectangle.overlaps(&gear.rectangle))
                .collect::<Vec<_>>();

            match near_numbers.as_slice() {
                [first, second] => first.number * second.number,
                _ => 0,
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input));
    println!("Part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod day_3_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part_1(include_str!("./example1.txt")), 4361);
    }

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(include_str!("./input.txt")), 539433);
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part_2(include_str!("./example1.txt")), 467835);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(include_str!("./input.txt")), 75847567);
    }
}
