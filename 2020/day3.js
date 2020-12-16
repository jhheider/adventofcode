const data = require('./day3.json')

const countTrees = ([dX, dY]) => {
  let position = [0, 0]
  let trees = 0
  while (position[1] < data.length) {
    let row = data[position[1]]
    let col = position[0] % row.length
    let cell = row.charAt(col)
    if (cell === '#') { trees += 1 }
    position[0] += dX
    position[1] += dY
  }
  return trees
}

console.log('Part 1:', countTrees([3, 1]))

let deltas = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
]

const pt2 = deltas.map(delta => countTrees(delta)).reduce((a, b) => a * b)

console.log('Part 2:', pt2)
