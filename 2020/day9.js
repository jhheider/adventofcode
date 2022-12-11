const data = require('./inputs/day9.json')

const isValid = (stream, preambleSize, row) => {
  if (row < preambleSize) {
    return true
  }
  for (let x = row - preambleSize; x < row - 1; x++) {
    for (let y = x + 1; y < row; y++) {
      if (stream[row] === stream[x] + stream[y]) {
        return true
      }
    }
  }
  return false
}

const part1 = data.filter((r, i) => !isValid(data, 25, i))[0]
console.log('Day 9: Part 1:', part1)

const findRange = (data, number) => {
  for (let x = 0; x < data.length - 1; x++) {
    for (let y = x + 1; y < data.length; y++) {
      const sum = data.slice(x, y + 1).reduce((a, b) => a + b)
      if (sum > number) {
        break
      }
      if (sum === number) {
        return data.slice(x, y + 1)
      }
    }
  }
}

const part2 = findRange(data, part1)
const min = Math.min(...part2)
const max = Math.max(...part2)
console.log('Day 9: Part 2:', min + max)
