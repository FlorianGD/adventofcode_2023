today := "`date -u +%d`"

new_day day:
  echo "pub mod day{{day}};" >> src/lib.rs
  cp day_template.txt "src/day{{day}}.rs"

today: (new_day today)
