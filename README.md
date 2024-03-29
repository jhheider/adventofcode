# Advent of Code

Might as well preserve this for posterity (you never know).

## Tasks

### Run-year

Run `xc [YEAR] [day]` to run a specific day/year.

```sh
YEAR="${1:-2022}"
cd "$YEAR"

case "$YEAR" in
2020)
  npm ci
  if test -f "day$2.js"; then
    node "day$2.js"
  else
    for DAY in $(seq 1 25); do
     test -f "day$DAY.js" && node "day$DAY.js"
    done
  fi
  ;;
*)
  cargo run --release "$2"
  ;;
esac
```

### Run-all

Runs solutions for all years.

```sh
set -x
YEARS="$(ls -d 20??)"

for YEAR in $YEARS; do
  echo -e "\nRunning $YEAR\n"
  xc run-year "$YEAR"
done
```

## Dependencies

|       Project       | Version |
|---------------------|---------|
| rust-lang.org/cargo |   ^0    |
| npmjs.com           |   ^9    |
