const data = require('./inputs/day3.json')

const countTrees = ([dX, dY]) => {
  const position = [0, 0]
  let trees = 0
  while (position[1] < data.length) {
    const row = data[position[1]]
    const col = position[0] % row.length
    const cell = row.charAt(col)
    if (cell === '#') {
      trees += 1
    }
    position[0] += dX
    position[1] += dY
  }
  return trees
}

console.log('Day 3: Part 1:', countTrees([3, 1]))

const deltas = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2]
]

const pt2 = deltas.map((delta) => countTrees(delta)).reduce((a, b) => a * b)

console.log('Day 3: Part 2:', pt2)
