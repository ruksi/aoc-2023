use std::iter::repeat;

pub fn solve_day15_part1(input: String) -> Result<String, String> {
    Ok(hash_initialization_sequence(&input).to_string())
}

pub fn solve_day15_part2(input: String) -> Result<String, String> {
    Ok(facility_focusing_power(input)?.to_string())
}

fn facility_focusing_power(input: String) -> Result<usize, String> {
    let mut boxes = repeat::<Vec<Lens>>(vec![])
        .take(256)
        .collect::<Vec<_>>();
    for step in input.split(',') {
        if step.contains('=') {
            let mut parts = step.split('=');
            let lens_label = parts.next().ok_or("no lens label!?")?;
            let box_index = hash_single(lens_label) as usize;
            let focal_length = parts.next()
                .ok_or("no lens focal length!?")?
                .parse::<u8>()
                .map_err(|e| e.to_string())?;
            let lens = Lens::new(lens_label, focal_length);
            let r#box = boxes.get_mut(box_index).ok_or(format!("no box at {box_index:}"))?;
            if let Some(old_lens) = r#box.iter_mut().find(|l| l.label == lens.label) {
                old_lens.focal_length = lens.focal_length;
            } else {
                r#box.push(lens);
            }
        }
        if step.contains('-') {
            let mut parts = step.split('-');
            let lens_label = parts.next().ok_or("no lens label!?")?;
            let box_index = hash_single(lens_label) as usize;
            let r#box = boxes.get_mut(box_index).ok_or(format!("no box at {box_index:?}!?"))?;
            r#box.retain(|existing| existing.label != lens_label);
        }
    }
    let facility_focusing_power = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let box_number = i + 1;
            let box_focusing_power: usize = b
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| {
                    let slot = lens_index + 1;
                    let focal_length = lens.focal_length;
                    box_number * slot * (focal_length as usize)
                })
                .sum();
            box_focusing_power
        })
        .sum();
    Ok(facility_focusing_power)
}

fn hash_initialization_sequence(text: &str) -> usize {
    text
        .split(',')
        .map(|piece| hash_single(piece) as usize)
        .sum()
}

fn hash_single(text: &str) -> u8 {
    text
        .chars()
        .map(|letter| letter as u8 as isize)
        .fold(0, |value, code| (value + code) * 17 % 256) as u8
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: u8,
}

impl Lens {
    fn new(label: &str, focal_length: u8) -> Self {
        Self { label: label.to_string(), focal_length }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn hashing_the_hash() {
        assert_eq!(hash_single("HASH"), 52);
    }

    #[test]
    fn solve_day15_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day15_part1(input)?, "1320");
        Ok(())
    }

    #[test]
    fn solve_day15_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day15_part1(input)?, "510801");
        Ok(())
    }

    #[test]
    fn solve_day15_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day15_part2(input)?, "145");
        Ok(())
    }

    #[test]
    fn solve_day15_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day15_part2(input)?, "212763");
        Ok(())
    }
}
