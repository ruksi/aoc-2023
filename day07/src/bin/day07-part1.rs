fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day07::solve_day07_part1(input)?;
    println!("{}", result);
    Ok(())
}
