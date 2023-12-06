pub fn solve_day05_part1(input: String) -> Result<String, String> {
    let almanac = Almanac::from(input)?;
    let final_forms = almanac.apply_all_tables_to_seeds();
    let smallest = final_forms.iter().min().ok_or("No smallest value found")?;
    Ok(smallest.to_string())
}

pub fn solve_day05_part2(input: String) -> Result<String, String> {
    let almanac = Almanac::from(input)?;
    let ranges = almanac.apply_all_tables_to_seed_ranges();
    let smallest_low_bound = ranges.iter().map(|r| r.0).min().ok_or("No smallest value found")?;
    Ok(smallest_low_bound.to_string())
}

struct Almanac {
    seeds: Vec<isize>,
    tables: Vec<Table>,
}

impl Almanac {
    fn from(input: String) -> Result<Self, String> {
        let mut parts = input.split("\n\n");

        let seeds_str = parts.next().ok_or(format!("Bad seeds line: {}", input))?;
        let seeds_parts = seeds_str.split_whitespace().skip(1); // discard "seeds:" 🚮
        let seeds = seeds_parts
            .map(|s| s.parse::<isize>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, String>>()?;

        let tables = parts.map(Table::from).collect::<Result<Vec<_>, String>>()?;

        Ok(Self { seeds, tables })
    }

    fn apply_all_tables_to_seeds(&self) -> Vec<isize> {
        let mut final_form = self.seeds.clone();
        for table in &self.tables {
            final_form = final_form.iter().map(|&s| table.apply(s)).collect();
        }
        final_form
    }

    fn apply_all_tables_to_seed_ranges(&self) -> Vec<Range> {
        let mut ranges = self.seeds
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let end = start + chunk[1] - 1;
                (start, end)
            })
            .collect::<Vec<_>>();
        for table in &self.tables {
            ranges = ranges
                .iter()
                .flat_map(|&range| table.apply_to_range(range))
                .collect();
        }
        ranges
    }
}

struct Table {
    source_type: String,
    destination_type: String,
    transforms: Vec<Transform>,
}

impl Table {
    fn from(input: &str) -> Result<Self, String> {
        let mut lines = input.lines();

        let mut header = lines.next().ok_or(format!("Bad table: {}", input))?.split_whitespace();
        let name = header.next().ok_or(format!("Bad table header: {}", input))?;
        let mut name_parts = name.split("-to-");
        let source = name_parts.next().ok_or(format!("Bad table name: {}", input))?;
        let destination = name_parts.next().ok_or(format!("Bad table name: {}", input))?;

        let transforms = lines.map(Transform::from).collect::<Result<Vec<_>, String>>()?;
        Ok(Self {
            source_type: source.to_string(),
            destination_type: destination.to_string(),
            transforms,
        })
    }

    fn apply(&self, value: isize) -> isize {
        for transform in &self.transforms {
            if value >= transform.applies_to.0 && value <= transform.applies_to.1 {
                return value + transform.offset;
            }
        }
        value
    }

    fn apply_to_range(&self, range: Range) -> Vec<Range> {
        let mut to_check = vec![range];
        let mut all_transformed = Vec::new();
        for transform in &self.transforms {
            let mut new_to_check = Vec::new();
            for range in to_check {
                let (extracted, rest) = extract_range(range, transform.applies_to);
                if let Some(extracted) = extracted {
                    all_transformed.push((
                        extracted.0 + transform.offset,
                        extracted.1 + transform.offset,
                    ));
                }
                // we just assume that the transform ranges don't overlap~~
                new_to_check.extend(rest);
            }
            to_check = new_to_check;
        }
        all_transformed.extend(to_check); // remember to add the unmodified ones back!
        all_transformed
    }
}

struct Transform {
    applies_to: Range,
    offset: isize,
}

impl Transform {
    fn from(input: &str) -> Result<Self, String> {
        let mut parts = input.split_whitespace();
        let destination_start = parts
            .next().ok_or(format!("Bad transform destination range start: {}", input))?
            .parse::<isize>().map_err(|e| e.to_string())?;
        let source_start = parts
            .next().ok_or(format!("Bad transform source range start: {}", input))?
            .parse::<isize>().map_err(|e| e.to_string())?;
        let range_length = parts
            .next()
            .ok_or(format!("Bad transform range length: {}", input))?
            .parse::<isize>().map_err(|e| e.to_string())?;
        Ok(Self {
            applies_to: (source_start, source_start + range_length - 1),
            offset: destination_start - source_start,
        })
    }
}

fn extract_range(original: Range, filter: Range) -> (Option<Range>, Vec<Range>) {
    if filter.1 < original.0 || filter.0 > original.1 {
        return (None, vec![original]); // no overlap, early return
    }

    let mut rest = Vec::new();
    if filter.0 > original.0 {
        rest.push((original.0, filter.0 - 1));
    }
    if filter.1 < original.1 {
        rest.push((filter.1 + 1, original.1));
    }

    let mut extracted = None;
    if filter.0 <= original.0 && filter.1 >= original.1 {
        extracted = Some(original); // to be extracted contains the whole original
    } else if filter.0 >= original.0 && filter.1 <= original.1 {
        extracted = Some(filter); // original contains the range to be extracted
    } else if filter.0 <= original.0 && filter.1 <= original.1 {
        extracted = Some((original.0, filter.1)); // overlaps from the left
    } else if filter.0 >= original.0 && filter.1 >= original.1 {
        extracted = Some((filter.0, original.1)); // overlaps from the right
    }

    (extracted, rest)
}

type Range = (isize, isize); // (start, end)

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn apply_table_on_number() -> Result<(), String> {
        let table = Table::from("alpha-to-beta\n50 98 2\n52 50 48")?;
        assert_eq!(table.source_type, "alpha");
        assert_eq!(table.destination_type, "beta");
        assert_eq!(table.transforms.len(), 2);
        assert_eq!(table.apply(10), 10);
        assert_eq!(table.apply(98), 50);
        assert_eq!(table.apply(99), 51);
        assert_eq!(table.apply(97), 99);
        Ok(())
    }

    #[test]
    fn splitting_ranges() -> Result<(), String> {
        assert_eq!(extract_range((1, 5), (6, 10)), (None, vec![(1, 5)]));
        assert_eq!(extract_range((1, 10), (5, 6)), (Some((5, 6)), vec![(1, 4), (7, 10)]));
        assert_eq!(extract_range((5, 6), (1, 10)), (Some((5, 6)), vec![]));
        assert_eq!(extract_range((5, 10), (4, 5)), (Some((5, 5)), vec![(6, 10)]));
        assert_eq!(extract_range((5, 10), (10, 11)), (Some((10, 10)), vec![(5, 9)]));
        Ok(())
    }

    #[test]
    fn apply_table_on_range() -> Result<(), String> {
        let table = Table::from("alpha-to-beta\n50 98 2\n52 50 48")?;
        assert_eq!(table.source_type, "alpha");
        assert_eq!(table.destination_type, "beta");
        assert_eq!(table.transforms.len(), 2);
        assert_eq!(table.transforms[0].offset, -48);
        assert_eq!(table.transforms[1].offset, 2);
        assert_eq!(table.apply_to_range((10, 10)), vec![(10, 10)]);
        assert_eq!(table.apply_to_range((98, 98)), vec![(50, 50)]);
        assert_eq!(table.apply_to_range((99, 99)), vec![(51, 51)]);
        assert_eq!(table.apply_to_range((97, 97)), vec![(99, 99)]);
        assert_eq!(table.apply_to_range((90, 100)), vec![(50, 51), (92, 99), (100, 100)]);
        Ok(())
    }

    #[test]
    fn solve_day05_part1_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day05_part1(input)?, "35");
        Ok(())
    }

    #[test]
    fn solve_day05_part1_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day05_part1(input)?, "836040384");
        Ok(())
    }

    #[test]
    fn solve_day05_part2_on_example() -> Result<(), String> {
        let input = fs::read_to_string("examples/example.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day05_part2(input)?, "46");
        Ok(())
    }

    #[test]
    fn solve_day05_part2_on_my_input() -> Result<(), String> {
        let input = fs::read_to_string("examples/ruksi.txt").map_err(|e| e.to_string())?;
        assert_eq!(solve_day05_part2(input)?, "10834440");
        Ok(())
    }
}
