fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day18::solve_day18_part2(input)?;
    println!("{}", result);
    Ok(())
}
