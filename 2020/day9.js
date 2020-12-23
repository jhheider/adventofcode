const data = require('./inputs/day9.json')

const test = [
  35,
  20,
  15,
  25,
  47,
  40,
  62,
  55,
  65,
  95,
  102,
  117,
  150,
  182,
  127,
  219,
  299,
  277,
  309,
  576
]

const is_valid = (stream, preamble_size, row) => {
  if (row < preamble_size) { return true }
  for(let x = row - preamble_size; x < row - 1; x++) {
    for(let y = x + 1; y < row; y++) {
      if (stream[row] === stream[x] + stream[y]) { return true }
    }
  }
  return false
}

console.log('Test data:', test.filter((r, i) => !is_valid(test, 5, i)))
const part1 = data.filter((r, i) => !is_valid(data, 25, i))[0]
console.log('Part 1:', part1)

const find_range = (data, number) => {
  for (let x = 0; x < data.length - 1; x++) {
    for (let y = x + 1; y < data.length; y++) {
      let sum = data.slice(x, y + 1).reduce((a, b) => a + b)
      if (sum > number) { break }
      if (sum === number) { return data.slice(x, y + 1) }
    }
  }
}

const part2 = find_range(data, part1)
const min = Math.min(...part2)
const max = Math.max(...part2)
console.log('Part 2:', part2, min + max)