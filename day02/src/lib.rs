use regex::{Captures, Regex};

pub fn solve_day02_part1(input: String) -> Result<String, String> {
    let games = input.split("\n").map(to_game).collect::<Vec<_>>();
    let limit = Set { red: Some(12), green: Some(13), blue: Some(14) };
    let possible_game_id_sum = games
        .iter()
        .filter(|game| {
            !game.sets.iter().any(|set| {
                set.red > limit.red || set.green > limit.green || set.blue > limit.blue
            })
        })
        .map(|game| game.id)
        .sum::<u32>();
    Ok(possible_game_id_sum.to_string())
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

struct Set {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

fn to_game(line: &str) -> Game {
    let mut parts = line.split(":");

    let game_str = parts.next().unwrap_or_else(|| panic!("Bad line: {}", line));
    let mut game_parts = game_str.split(" ");
    game_parts.next(); // "Game"
    let id_str = game_parts.next().unwrap_or_else(|| panic!("Bad line: {}", line));
    let id = id_str.parse().unwrap_or_else(|e| panic!("Bad line: {}, {}", line, e));

    let sets_str = parts.next().unwrap_or_else(|| panic!("Bad line: {}", line));
    let red_re = Regex::new(r"(?<count>\d+) red").expect("Bad red regex");
    let green_re = Regex::new(r"(?<count>\d+) green").expect("Bad green regex");
    let blue_re = Regex::new(r"(?<count>\d+) blue").expect("Bad blue regex");
    fn to_u32(captures: Captures) -> Option<u32> {
        Some(
            captures
                .name("count")
                .expect("but the count group is only thing we match!?")
                .as_str()
                .parse()
                .expect("but we matched a number (\\d)!?")
        )
    }
    let sets = sets_str
        .split(";")
        .map(|set_str| {
            let red = red_re.captures(set_str).map_or(None, to_u32);
            let green = green_re.captures(set_str).map_or(None, to_u32);
            let blue = blue_re.captures(set_str).map_or(None, to_u32);
            Set { red, green, blue }
        })
        .collect();

    Game { id, sets }
}

pub fn solve_day02_part2(input: String) -> Result<String, String> {
    let games = input.split("\n").map(to_game).collect::<Vec<_>>();
    let total_power = games.iter().map(to_power).sum::<Result<u32, String>>()?;
    Ok(total_power.to_string())
}

fn to_power(game: &Game) -> Result<u32, String> {
    let max_red = game.sets.iter().filter_map(|set| set.red).max().ok_or("No reds in game")?;
    let max_green = game.sets.iter().filter_map(|set| set.green).max().ok_or("No greens in game")?;
    let max_blue = game.sets.iter().filter_map(|set| set.blue).max().ok_or("No blues in game")?;
    return Ok(max_red * max_green * max_blue);
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn solve_day02_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day02_part1(input)?, "8");
        Ok(())
    }

    #[test]
    fn solve_day02_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day02_part1(input)?, "2278");
        Ok(())
    }

    #[test]
    fn solve_day02_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day02_part2(input)?, "2286");
        Ok(())
    }

    #[test]
    fn solve_day02_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day02_part2(input)?, "67953");
        Ok(())
    }
}
