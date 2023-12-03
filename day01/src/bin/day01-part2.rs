fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day01::solve_day01_part2(input)?;
    println!("{}", result);
    Ok(())
}
