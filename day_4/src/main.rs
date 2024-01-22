#![feature(iterator_try_collect)]
#![feature(byte_slice_trim_ascii)]

use std::collections::HashSet;

struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

fn parse_input(input: &str) -> Result<Vec<Card>, &'static str> {
    input.lines().map(parse_card).try_collect()
}

fn parse_card(line: &str) -> Result<Card, &'static str> {
    let [head, body] = line.split(':').collect::<Vec<_>>()[..] else {
        return Err("Invalid line");
    };

    let id = head
        .split_ascii_whitespace()
        .last()
        .ok_or("Invalid card head")?
        .parse::<u32>()
        .or(Err("Invalid card id"))?;

    let [winning_numbers, numbers] = body.split('|').collect::<Vec<_>>()[..] else {
        return Err("Invalid body");
    };

    let winning_numbers = winning_numbers
        .split_ascii_whitespace()
        .map(|number| number.trim_ascii().parse())
        .try_collect()
        .or(Err("Invalid number"))?;

    let numbers = numbers
        .split_ascii_whitespace()
        .map(|number| number.trim_ascii().parse())
        .try_collect()
        .or(Err("Invalid number"))?;

    Ok(Card {
        id,
        numbers,
        winning_numbers,
    })
}

fn solve_part_1(input: &str) -> Result<u32, &'static str> {
    let cards = parse_input(input)?;

    Ok(cards
        .iter()
        .map(|card| {
            let matching_count = card
                .numbers
                .iter()
                .filter(|number| card.winning_numbers.contains(number))
                .count();

            match matching_count {
                0 => 0,
                1 => 1,
                _ => 2u32.pow(matching_count as u32 - 1),
            }
        })
        .sum())
}

fn solve_part_2(input: &str) -> Result<u32, &'static str> {
    let cards = parse_input(input)?;

    Ok(cards
        .iter()
        .map(|card| {
            let matching_count = card
                .numbers
                .iter()
                .filter(|number| card.winning_numbers.contains(number))
                .count();

            matching_count as u32 + 1
        })
        .sum())
}

fn main() -> Result<(), &'static str> {
    let input = include_str!("./input.txt");
    println!("Part 1: {}", solve_part_1(input)?);
    println!("Part 2: {}", solve_part_2(input)?);

    Ok(())
}

#[cfg(test)]
mod day_3_tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(solve_part_1(include_str!("./example1.txt")), Ok(13));
    }

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(include_str!("./input.txt")), Ok(25231));
    }

    #[test]
    fn example_2() {
        assert_eq!(solve_part_2(include_str!("./example1.txt")), Ok(30));
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(include_str!("./input.txt")), Ok(75847567));
    }
}
