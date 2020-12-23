const _ = require('lodash')
const input = require('./inputs/day11.json')

const occupied = (array, y, x) => x < 0 || y < 0 || x >= array[0].length || y >= array.length || array[y][x] !== '#' ? false : true

const adjacent = (array, x, y) =>
  occupied(array, y - 1, x - 1) + occupied(array, y - 1, x) + occupied(array, y - 1, x + 1) +
  occupied(array, y, x - 1) + occupied(array, y, x + 1) +
  occupied(array, y + 1, x - 1) + occupied(array, y + 1, x) + occupied(array, y + 1, x + 1)

const splitter = r => r.split('')
const test1 = input.test1.map(splitter)

const permute = (data, adj, threshold) => data.map((r, y) => r.map((c, x) => {
  const count = adj(data, x, y)
  switch (c) {
    case '.': return '.'
    case 'L': return count === 0 ? '#' : 'L'
    case '#': return count >= threshold ? 'L' : '#'
  }
}))

const print = array => console.log(array.map(r => r.join('')).join('\n') + '\n')

const play = (input, adj, threshold, log = false) => {
  let val1 = input
  let val2 = []
  let round = 0
  while(!_.isEqual(val1, val2)) {
    val2 = val1
    if (log) {
      console.log(`Round: ${round++}`)
      print(val1)
    }
    val1 = permute(val1, adj, threshold)
  }
  return val1
}

const count_occupied = array => array.reduce((a, b) => a + b.reduce((c, d) => c += d === '#' ? 1 : 0, 0), 0)

console.log('Test 1:', count_occupied(play(test1, adjacent, 4)))
console.log('Part 1:', count_occupied(play(input.data.map(splitter), adjacent, 4)))

const translate = (x, y, range, direction) => {
  if ([1, 2, 3].includes(direction)) y -= range
  if ([6, 7, 8].includes(direction)) y += range
  if ([1, 4, 6].includes(direction)) x -= range
  if ([3, 5, 8].includes(direction)) x += range
  return [x, y]
}

const los_occupied = (array, x, y, direction) => {
  for(let range = 1; ; range++) {
    const [curX, curY] = translate(x, y, range, direction)
    if (curX < 0 || curY < 0 || curX >= array[0].length || curY >= array.length) return false
    switch (array[curY][curX]) {
      case '.': break
      case 'L': return false
      case '#': return true
    }
  }
}

const adjacent2 = (array, x, y) => [1, 2, 3, 4, 5, 6, 7, 8].map(dir => los_occupied(array, x, y, dir)).reduce((a, b) => a + b)

console.log('Test 2:', count_occupied(play(test1, adjacent2, 5)))
console.log('Part 2:', count_occupied(play(input.data.map(splitter), adjacent2, 5)))