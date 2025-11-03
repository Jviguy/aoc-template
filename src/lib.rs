use std::fs;

pub fn read_input_file(day: usize, use_example: bool) -> Result<String, std::io::Error> {
    let file_type = if use_example { "examples" } else { "inputs" };
    let path = format!("data/{}/day{:02}.txt", file_type, day);
    fs::read_to_string(&path)
}


// TODO: Maybe consult see if this should be more advanced this works for just this simple case
// but like also arguments of like just keep compile times faster.
#[macro_export]
macro_rules! aoc_main {
    ($day_num:expr, $part1_fn:ident, $part2_fn:ident) => {
        pub fn main() -> Result<(), Box<dyn std::error::Error>> {
            let args: Vec<String> = std::env::args().collect();
            let mut use_example = false;
            let mut part_num = 1;

            if args.contains(&"--example".to_string()) { use_example = true; }

            if args.contains(&"2".to_string()) { part_num = 2; }

            // --- File Reading & Execution ---
            let input = $crate::read_input_file($day_num, use_example)
                .expect("Failed to read input file");

            let result = match part_num {
                1 => $part1_fn(&input),
                2 => $part2_fn(&input),
                _ => return Err("Invalid part number".into()),
            }?;

            println!("Day {} Part {}: {}", $day_num, part_num, result);
            Ok(())
        }
    }
}