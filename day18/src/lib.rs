use std::collections::{HashMap, HashSet};
use std::ops::Deref;

pub fn solve_day18_part1(input: String) -> Result<String, String> {
    let plan = DigPlan::from_text(&input)?;
    let _map = DigMap::from_plan(plan)?;
    Ok("#yolo".to_string())
}

pub fn solve_day18_part2(input: String) -> Result<String, String> {
    Ok(input)
}

#[derive(Debug, Clone)]
struct DigMap {
    edges: Vec<Vec<Option<String>>>,
}

impl Deref for DigMap {
    type Target = Vec<Vec<Option<String>>>;
    fn deref(&self) -> &Self::Target { &self.edges }
}

fn some_neighbors(point: Point, edges: &Vec<Vec<Option<String>>>) -> Vec<Point> {
    neighbors(point)
        .into_iter()
        .filter(|(x, y)| {
            if *x < 0 || *y < 0 { return false; }
            let Some(row) = edges.get(*y as usize) else { return false; };
            let Some(cell) = row.get(*x as usize) else { return false; };
            return cell.is_some();
        })
        .collect()
}

fn neighbors(point: Point) -> Vec<Point> {
    let (x, y) = point;
    vec![
        (x + 1, y),
        (x, y + 1),
        (x - 1, y),
        (x, y - 1),
    ]
}

impl DigMap {
    fn from_plan(plan: DigPlan) -> Result<Self, String> {
        let mut map = HashMap::new();
        let mut position: Point = (0, 0);
        let mut start_direction: Option<Direction> = None;
        for step in plan.iter() {
            if start_direction.is_none() {
                start_direction = Some(step.direction.clone());
            }
            for _ in 0..step.length {
                match step.direction {
                    Direction::Up => position.1 -= 1,
                    Direction::Right => position.0 += 1,
                    Direction::Down => position.1 += 1,
                    Direction::Left => position.0 -= 1,
                }
                map.insert(position, step.color.clone());
            }
        }

        // normalize minimum to (0, 0)
        let min_x = map.keys().map(|(x, _)| x).min().ok_or("No min x")?.to_owned();
        let min_y = map.keys().map(|(_, y)| y).min().ok_or("No min y")?.to_owned();
        map = map
            .into_iter()
            .map(|((x, y), color)| ((x - min_x, y - min_y), color))
            .collect();

        // convert to vector
        let mut edges = Vec::new();
        let max_x = map.keys().map(|(x, _)| x).max().ok_or("No max x")?.to_owned();
        let max_y = map.keys().map(|(_, y)| y).max().ok_or("No max y")?.to_owned();
        for y in 0..=max_y {
            let mut row = Vec::new();
            for x in 0..=max_x {
                if let Some(color) = map.get(&(x, y)) {
                    row.push(Some(color.to_owned()));
                } else {
                    row.push(None);
                }
            }
            edges.push(row);
        }

        let start_point = (min_x.wrapping_neg(), min_y.wrapping_neg());
        let start_direction = start_direction.ok_or("No start direction")?;
        let next = match start_direction {
            Direction::Up => (start_point.0, start_point.1 - 1),
            Direction::Right => (start_point.0 + 1, start_point.1),
            Direction::Down => (start_point.0, start_point.1 + 1),
            Direction::Left => (start_point.0 - 1, start_point.1),
        };
        let mut route = vec![start_point, next];
        loop {
            let last = route.last().ok_or("No last route point")?.to_owned();
            let neighbors = some_neighbors(last, &edges);
            if neighbors.len() != 2 { panic!("{neighbors:?}"); }
            let next = if !route.contains(&neighbors[0]) {
                Some(neighbors[0])
            } else if !route.contains(&neighbors[1]) {
                Some(neighbors[1])
            } else {
                None
            };
            let Some(next) = next else {
                break;
            };
            route.push(next);
        }


        let _start = (start_point, start_direction);
        Ok(Self { edges })
    }

    fn edge_volume(&self) -> usize {
        self.iter()
            .map(|row| row.iter().filter(|cell| cell.is_some()).count())
            .sum()
    }

    fn print_edges(&self) {
        for row in self.iter() {
            for cell in row.iter() {
                match cell {
                    Some(_color) => print!("#"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone)]
struct DigPlan(Vec<DigStep>);

impl Deref for DigPlan {
    type Target = Vec<DigStep>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DigPlan {
    fn from_text(text: &str) -> Result<Self, String> {
        let steps = text
            .lines()
            .map(|line| DigStep::from_text(line))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(steps))
    }

    fn get_sideness(&self) -> Side {
        let degrees = self
            .windows(2)
            .map(|w| {
                let degrees = match (&w[0].direction, &w[1].direction) {
                    (Direction::Up, Direction::Right) => 90,
                    (Direction::Up, Direction::Left) => -90,
                    (Direction::Right, Direction::Up) => -90,
                    (Direction::Right, Direction::Down) => 90,
                    (Direction::Down, Direction::Right) => -90,
                    (Direction::Down, Direction::Left) => 90,
                    (Direction::Left, Direction::Up) => 90,
                    (Direction::Left, Direction::Down) => -90,
                    _ => 0,
                };
                degrees
            })
            .sum::<isize>();
        if degrees == 270 || degrees == 450 { // missing 90 or -90
            Side::Right
        } else if degrees == -270 || degrees == -450 {  // missing -90 or 90
            Side::Left
        } else {
            panic!("Invalid route turns degree sum: {}", degrees);
        }
    }
}

#[derive(Debug, Clone)]
struct DigStep {
    direction: Direction,
    length: usize,
    color: String,
}

impl DigStep {
    fn from_text(text: &str) -> Result<Self, String> {
        let mut parts = text.split_whitespace();
        let direction = match parts.next() {
            Some("R") => Direction::Right,
            Some("L") => Direction::Left,
            Some("U") => Direction::Up,
            Some("D") => Direction::Down,
            Some(other) => return Err(format!("Invalid direction: {}", other)),
            None => return Err("No direction found".to_string()),
        };
        let length = match parts.next() {
            Some(length) => length.parse::<usize>().map_err(|e| e.to_string())?,
            None => return Err("No length found".to_string()),
        };
        let color = match parts.next() {
            Some(color) => color.to_string().trim_matches(|c| c == '(' || c == ')').to_string(),
            None => return Err("No color found".to_string()),
        };
        Ok(Self { direction, length, color })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    fn turn_to(&self, sideness: &Side) -> Direction {
        match (self, sideness) {
            (Direction::Up, Side::Left) => Direction::Left,
            (Direction::Up, Side::Right) => Direction::Right,
            (Direction::Right, Side::Left) => Direction::Up,
            (Direction::Right, Side::Right) => Direction::Down,
            (Direction::Down, Side::Left) => Direction::Right,
            (Direction::Down, Side::Right) => Direction::Left,
            (Direction::Left, Side::Left) => Direction::Down,
            (Direction::Left, Side::Right) => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Side { Left, Right }

type Point = (isize, isize); // (x, y)

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_stuff() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        let plan = DigPlan::from_text(&input)?;
        let map = DigMap::from_plan(plan)?;
        assert_eq!(map.edge_volume(), 38);
        map.print_edges();
        Ok(())
    }

    #[test]
    fn test_dig_plan() -> Result<(), String> {
        let plan = DigPlan::from_text("R 6 (#70c710)\nD 5 (#0dc571)");
        assert_eq!(plan?.len(), 2);
        Ok(())
    }

    #[test]
    fn test_dig_step() -> Result<(), String> {
        let step = DigStep::from_text("R 6 (#70c710)")?;
        assert_eq!(step.direction, Direction::Right);
        assert_eq!(step.length, 6);
        assert_eq!(step.color, "#70c710");
        Ok(())
    }

    #[test]
    fn solve_day18_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day18_part1(input)?, "#yolo");
        Ok(())
    }

    #[test]
    fn solve_day18_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day18_part1(input)?, "#yolo");
        Ok(())
    }

    // #[test]
    // fn solve_day18_part2_on_example() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day18_part2(input)?, "#yolo");
    //     Ok(())
    // }

    // #[test]
    // fn solve_day18_part2_on_my_input() -> Result<(), String> {
    //     let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
    //     assert_eq!(solve_day18_part2(input)?, "#yolo");
    //     Ok(())
    // }
}
