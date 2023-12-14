set positional-arguments

@new_day day:
  echo "pub mod day$1;" >> src/lib.rs
  echo "pub fn parse_input(input: &str)  -> Vec<u8> { vec![0] }\n\npub fn part1(input: Vec<u8>) -> usize {0}" > "src/day$1.rs" 
