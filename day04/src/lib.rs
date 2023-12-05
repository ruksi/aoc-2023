use std::collections::HashSet;

pub fn solve_day04_part1(input: String) -> Result<String, String> {
    let cards = input.lines().map(Card::from).collect::<Vec<_>>();
    let score = cards.iter().map(|c| c.point_score()).sum::<usize>();
    Ok(score.to_string())
}

#[derive(Debug)]
struct Card {
    win_numbers: HashSet<usize>,
    your_numbers: HashSet<usize>,
}

impl Card {
    fn from(line: &str) -> Card {
        let mut parts = line.split(":");

        let card_str = parts.next().unwrap_or_else(|| panic!("Bad card line {}", line));
        let mut card_parts = card_str.split_whitespace();
        card_parts.next(); // skip the "Card" part
        let id_str = card_parts.next().unwrap_or_else(|| panic!("Bad card id {}", line));
        let _id = id_str.parse::<usize>().unwrap_or_else(|_e| panic!("Bad card id {}", line));

        let numbers_str = parts.next().unwrap_or_else(|| panic!("Bad card line {}", line));
        let mut numbers_parts = numbers_str.split("|");
        let win_numbers_str = numbers_parts.next().unwrap_or_else(|| panic!("Bad numbers {}", line));
        let your_numbers_str = numbers_parts.next().unwrap_or_else(|| panic!("Bad numbers {}", line));

        let win_numbers = win_numbers_str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap_or_else(|_e| panic!("Bad win number {}", line)))
            .collect::<HashSet<_>>();
        let your_numbers = your_numbers_str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap_or_else(|_e| panic!("Bad your number {}", line)))
            .collect::<HashSet<_>>();

        Card { win_numbers, your_numbers }
    }

    fn match_count(&self) -> usize {
        self.win_numbers
            .intersection(&self.your_numbers)
            .collect::<HashSet<_>>()
            .len()
    }

    fn point_score(&self) -> usize {
        let mut score = 0;
        for _ in 0..self.match_count() {
            match score {
                0 => score = 1,
                _ => score *= 2,
            }
        }
        score
    }
}


pub fn solve_day04_part2(input: String) -> Result<String, String> {
    let cards = input.lines().map(Card::from).collect::<Vec<_>>();
    let mut card_count = cards.len();
    let mut scan_indices: Vec<usize> = (0..card_count).collect();
    while !scan_indices.is_empty() {
        let mut next_scan_indices = Vec::new();
        for card_index in scan_indices {
            let card = &cards[card_index];
            let new_card_count = card.match_count();
            let new_card_indices = (card_index + 1)..=(card_index + new_card_count);
            next_scan_indices.extend(new_card_indices);
            card_count += new_card_count;
        }
        scan_indices = next_scan_indices;
    }
    Ok(card_count.to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn solve_day04_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day04_part1(input)?, "13");
        Ok(())
    }

    #[test]
    fn solve_day04_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day04_part1(input)?, "23441");
        Ok(())
    }

    #[test]
    fn solve_day04_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day04_part2(input)?, "30");
        Ok(())
    }

    #[test]
    fn solve_day04_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day04_part2(input)?, "5923918");
        Ok(())
    }
}
