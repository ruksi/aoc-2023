use std::collections::HashMap;
use std::ops::Deref;

pub fn solve_day08_part1(input: String) -> Result<String, String> {
    let (instructions, network) = parse_input(input)?;
    let start = "AAA".to_string();
    let goal = "ZZZ".to_string();
    let mut label = start;
    for (route_length, step) in instructions.iter().enumerate() {
        if label == goal { return Ok(route_length.to_string()); }
        let node = network.get(&label).ok_or_else(|| format!("Unknown label: {}", label))?;
        match step {
            'L' => label = node.left.clone(),
            'R' => label = node.right.clone(),
            _ => return Err(format!("Invalid instruction step: {}", step)),
        }
    };
    Err(format!("No path to {}", goal))
}

pub fn solve_day08_part2(input: String) -> Result<String, String> {
    let (instructions, network) = parse_input(input)?;

    let lengths_when_route_begins_to_loop = network
        .keys()
        .filter(|label| label.ends_with('A'))
        .map(|start_label| {
            let mut label = start_label;
            for (route_length, step) in instructions.iter().enumerate() {
                if label.ends_with('Z') { return route_length; }
                let node = network.get(label).unwrap_or_else(|| panic!("Unknown label: {}", label));
                match step {
                    'L' => label = &node.left,
                    'R' => label = &node.right,
                    _ => panic!("Invalid instruction character: {}", step),
                }
            };
            unreachable!()
        })
        .collect::<Vec<_>>();

    let least_common_multiple = lengths_when_route_begins_to_loop.into_iter().fold(1, lcm);
    Ok(least_common_multiple.to_string())
}

fn parse_input(input: String) -> Result<(Instructions, Network), String> {
    let mut parts = input.split("\n\n");
    let instructions_str = parts.next().ok_or("No instructions in input")?;
    let instructions = Instructions::from(instructions_str);
    let network_str = parts.next().ok_or("No network in input")?;
    let network = Network::from(network_str)?;
    Ok((instructions, network))
}

struct Instructions(Vec<char>);

impl Deref for Instructions {
    type Target = Vec<char>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Instructions {
    fn from(text: &str) -> Self {
        let as_chars = text.chars().collect::<Vec<char>>();
        Self(as_chars)
    }
    pub fn iter(&self) -> InstructionsIterator {
        // there is probably some shorthand for this in std::iter but 🤷
        InstructionsIterator { instructions: self, index: 0, length: self.len() }
    }
}

struct InstructionsIterator<'a> {
    instructions: &'a Instructions,
    index: usize,
    length: usize,
}

impl Iterator for InstructionsIterator<'_> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        // an infinite iterators repeating the instruction steps ♾️
        let current = self.index;
        self.index += 1;
        self.instructions.get((current) % self.length).copied()
    }
}

struct Network(HashMap<String, Node>);

impl Deref for Network {
    type Target = HashMap<String, Node>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Network {
    fn from(text: &str) -> Result<Self, String> {
        let mut nodes = HashMap::new();

        // create the edge references
        for line in text.lines() {
            let mut parts = line.split(" = ");
            let node_label = parts.next().ok_or("Invalid node line, no node label")?;

            let edges_str = parts.next().ok_or("Invalid node line, no edges")?;
            let mut edges = edges_str
                .split(",")
                .map(|e_str| e_str.trim_matches(|c| c == '(' || c == ')' || c == ' '));
            let left_label = edges.next().ok_or("Invalid node line, no left edge label")?;
            let right_label = edges.next().ok_or("Invalid node line, no right edge label")?;

            let node = Node::new(left_label, right_label);
            nodes.insert(node_label.to_string(), node);
        }

        Ok(Self(nodes))
    }
}

struct Node {
    // the node labels are the hash map keys
    left: String,
    right: String,
}

impl Node {
    fn new(left: &str, right: &str) -> Self {
        Self { left: left.to_string(), right: right.to_string() }
    }
}

// https://rosettacode.org/wiki/Greatest_common_divisor#Stein's_Algorithm
fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (std::cmp::min(x, y), std::cmp::max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

// https://rosettacode.org/wiki/Least_common_multiple#Rust
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn solve_day08_part1_on_example_rl() -> Result<(), String> {
        let input = fs::read_to_string("examples/example-rl.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day08_part1(input)?, "2");
        Ok(())
    }

    #[test]
    fn solve_day08_part1_on_example_llr() -> Result<(), String> {
        let input = fs::read_to_string("examples/example-llr.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day08_part1(input)?, "6");
        Ok(())
    }

    #[test]
    fn solve_day08_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day08_part1(input)?, "19241");
        Ok(())
    }

    #[test]
    fn solve_day08_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example-two-paths.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day08_part2(input)?, "6");
        Ok(())
    }

    #[test]
    fn solve_day08_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day08_part2(input)?, "9606140307013");
        Ok(())
    }
}
