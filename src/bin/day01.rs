use {{project_name}}::aoc_main;

fn part1(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(format!("Input has {} lines", input.lines().count()))
}

fn part2(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok("Part 2 not implemented".to_string())
}

aoc_main!({{DAY_NUM_PLACEHOLDER}}, part1, part2);