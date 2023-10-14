default:
  @just -l

# template for a new year in rust
new-year YEAR:
  @test ! -e {{YEAR}} && mkdir -p {{YEAR}}/{src,data} && cp 2022/src/main.rs {{YEAR}}/src/main.rs && for i in `seq 1 25`; do touch {{YEAR}}/data/day$i.txt; mkdir {{YEAR}}/src/day$i; echo 'pub fn main() {}' >{{YEAR}}/src/day$i/mod.rs; done && echo 'tab_spaces = 2' >{{YEAR}}/.rustfmt.toml && echo '[package]\nname = "aoc{{YEAR}}"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\nregex = "*"' >{{YEAR}}/Cargo.toml