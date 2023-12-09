fn main() -> Result<(), String> {
    let input = basics::read_input()?;
    let result = day09::solve_day09_part2(input)?;
    println!("{}", result);
    Ok(())
}
