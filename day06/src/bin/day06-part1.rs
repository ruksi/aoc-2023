fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day06::solve_day06_part1(input)?;
    println!("{}", result);
    Ok(())
}
