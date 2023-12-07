mod part1;
mod part2;

fn main() -> anyhow::Result<()> {
    let lines = std::io::stdin()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;
    println!("{}\n{}", part1::run(&lines)?, part2::run(&lines)?);
    Ok(())
}
