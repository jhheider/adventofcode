default:
  @just -l

# template for a new year in rust
new-year YEAR:
  @test ! -e {{YEAR}}
  mkdir -p {{YEAR}}/{src,data}
  cp .template/main.rs {{YEAR}}/src/main.rs
  cp .template/data.rs {{YEAR}}/src/data.rs
  touch {{YEAR}}/data/day1.txt
  for i in `seq 1 25`; do cp .template/day.rs {{YEAR}}/src/day$i.rs; done
  cargo init {{YEAR}} --name aoc{{YEAR}} --edition 2021
  cargo add regex --manifest-path {{YEAR}}/Cargo.toml