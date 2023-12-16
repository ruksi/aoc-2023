fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day13::solve_day13_part1(input)?;
    println!("{}", result);
    Ok(())
}
