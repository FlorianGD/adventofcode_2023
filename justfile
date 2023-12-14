set positional-arguments

@new_day day:
  echo "pub mod day$1;" >> src/lib.rs
  cp day_template.txt "src/day$1.rs"
