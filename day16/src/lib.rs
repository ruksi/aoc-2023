use std::collections::HashSet;

pub fn solve_day16_part1(input: String) -> Result<String, String> {
    let (layout, height, width) = parse_layout(input);
    let count = get_energized_count(&layout, height, width, Beam::new(0, 0, Direction::Right))?;
    Ok(count.to_string())
}

pub fn solve_day16_part2(input: String) -> Result<String, String> {
    let (layout, height, width) = parse_layout(input);
    // the following unnecessarily goes through all cells but 🤷
    let starts = layout
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .iter()
                .enumerate()
                .flat_map(|(x, _)| {
                    let mut starts = vec![];
                    if y == 0 { starts.push(Beam::new(x, y, Direction::Down)); }
                    if x == width - 1 { starts.push(Beam::new(x, y, Direction::Left)); }
                    if y == height - 1 { starts.push(Beam::new(x, y, Direction::Up)); }
                    if x == 0 { starts.push(Beam::new(x, y, Direction::Right)); }
                    starts
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_energy = starts
        .into_iter()
        .map(|start| get_energized_count(&layout, height, width, start))
        .filter_map(Result::ok)
        .max()
        .ok_or_else(|| "No solution found".to_string())?;
    Ok(max_energy.to_string())
}

fn parse_layout(input: String) -> (Vec<Vec<char>>, usize, usize) {
    let layout = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let height = layout.len();
    let width = layout[0].len();
    (layout, height, width)
}

fn get_energized_count(layout: &Vec<Vec<char>>, height: usize, width: usize, start: Beam) -> Result<usize, String> {
    let mut history = HashSet::new();
    let mut beams = vec![start];
    while !beams.is_empty() {
        history.extend(beams.iter().cloned());
        beams = beams
            .drain(..)
            .flat_map(|mut beam| {
                let cell = layout[beam.position.1][beam.position.0];
                if cell == '/' {
                    match beam.heading {
                        Direction::Up => beam.heading = Direction::Right,
                        Direction::Right => beam.heading = Direction::Up,
                        Direction::Down => beam.heading = Direction::Left,
                        Direction::Left => beam.heading = Direction::Down,
                    }
                    return vec![beam];
                }
                if cell == '\\' {
                    match beam.heading {
                        Direction::Up => beam.heading = Direction::Left,
                        Direction::Right => beam.heading = Direction::Down,
                        Direction::Down => beam.heading = Direction::Right,
                        Direction::Left => beam.heading = Direction::Up,
                    }
                    return vec![beam];
                }
                if cell == '-' {
                    if beam.heading == Direction::Up || beam.heading == Direction::Down {
                        beam.heading = Direction::Left;
                        let split = Beam::new(beam.position.0, beam.position.1, Direction::Right);
                        return vec![beam, split];
                    }
                }
                if cell == '|' {
                    if beam.heading == Direction::Right || beam.heading == Direction::Left {
                        beam.heading = Direction::Up;
                        let split = Beam::new(beam.position.0, beam.position.1, Direction::Down);
                        return vec![beam, split];
                    }
                }
                vec![beam]
            })
            .filter_map(|mut beam| {
                if beam.heading == Direction::Left {
                    if beam.position.0 == 0 { return None; }
                    beam.position.0 -= 1;
                }
                if beam.heading == Direction::Right {
                    if beam.position.0 == width - 1 { return None; }
                    beam.position.0 += 1;
                }
                if beam.heading == Direction::Up {
                    if beam.position.1 == 0 { return None; }
                    beam.position.1 -= 1;
                }
                if beam.heading == Direction::Down {
                    if beam.position.1 == height - 1 { return None; }
                    beam.position.1 += 1;
                }
                if history.contains(&beam) {
                    // don't go to paths we've already covered
                    return None;
                }
                Some(beam)
            })
            .collect();
    }

    let energized = history
        .iter()
        .map(|beam| beam.position)
        .collect::<HashSet<Point>>();

    Ok(energized.len())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    position: Point,
    heading: Direction,
}

impl Beam {
    fn new(x: usize, y: usize, heading: Direction) -> Self {
        Self { position: (x, y), heading }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction { Up, Right, Down, Left }

type Point = (usize, usize); // (x, y)

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn solve_day16_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day16_part1(input)?, "46");
        Ok(())
    }

    #[test]
    fn solve_day16_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day16_part1(input)?, "8034");
        Ok(())
    }

    #[test]
    fn solve_day16_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day16_part2(input)?, "51");
        Ok(())
    }

    #[test]
    fn solve_day16_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day16_part2(input)?, "8225");
        Ok(())
    }
}
