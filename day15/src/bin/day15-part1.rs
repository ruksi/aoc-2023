fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day15::solve_day15_part1(input)?;
    println!("{}", result);
    Ok(())
}
