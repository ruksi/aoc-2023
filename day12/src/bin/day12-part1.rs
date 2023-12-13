fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day12::solve_day12_part1(input)?;
    println!("{}", result);
    Ok(())
}
