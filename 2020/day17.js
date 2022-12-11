const { test, input } = require('./inputs/day17.json')

const p2a = (point) => point.split(':').map((n) => +n)
const a2p = (array) => array.join(':')
const makePoint = (x, y, z) => a2p([x, y, z])
const makePoint4d = (x, y, z, w) => a2p([x, y, z, w])
const dimensionality = (input) => p2a(input.values().next().value).length

const loadMap = (input, dimensions) => {
  const output = new Set()
  input.forEach((r, y) =>
    r
      .split('')
      .forEach(
        (c, x) =>
          c === '#' &&
          output.add(
            dimensions === 4 ? makePoint4d(x, y, 0, 0) : makePoint(x, y, 0)
          )
      )
  )
  return output
}

const checkAdjacent = (input, point) => {
  const [pX, pY, pZ, pW] = p2a(point)
  let adjacent = 0
  for (let x = -1; x <= 1; x++) {
    for (let y = -1; y <= 1; y++) {
      for (let z = -1; z <= 1; z++) {
        if (pW !== undefined) {
          for (let w = -1; w <= 1; w++) {
            const checkPoint = makePoint4d(pX + x, pY + y, pZ + z, pW + w)
            if (checkPoint !== point && input.has(checkPoint)) adjacent += 1
          }
        } else {
          const checkPoint = makePoint(pX + x, pY + y, pZ + z)
          if (checkPoint !== point && input.has(checkPoint)) adjacent += 1
        }
      }
    }
  }
  return adjacent
}

const bounds = (input) =>
  Array.from(input).reduce(
    (output, point) => {
      const [x, y, z, w] = p2a(point)
      return [
        Math.max(output[0], x),
        Math.min(output[1], x),
        Math.max(output[2], y),
        Math.min(output[3], y),
        Math.max(output[4], z),
        Math.min(output[5], z),
        w !== undefined ? Math.max(output[6], w) : 0,
        w !== undefined ? Math.min(output[7], w) : 0
      ]
    },
    [0, 0, 0, 0, 0, 0, 0, 0]
  )

const iterateMap = (input) => {
  const dimensions = dimensionality(input)
  const [highX, lowX, highY, lowY, highZ, lowZ, highW, lowW] = bounds(input)
  const output = new Set()
  for (let z = lowZ - 1; z <= highZ + 1; z++) {
    for (let y = lowY - 1; y <= highY + 1; y++) {
      for (let x = lowX - 1; x <= highX + 1; x++) {
        if (dimensions === 4) {
          for (let w = lowW - 1; w <= highW + 1; w++) {
            const p = makePoint4d(x, y, z, w)
            const count = checkAdjacent(input, p)
            if ((input.has(p) && count === 2) || count === 3) {
              output.add(p)
            }
          }
        } else {
          const p = makePoint(x, y, z)
          const count = checkAdjacent(input, p)
          if ((input.has(p) && count === 2) || count === 3) {
            output.add(p)
          }
        }
      }
    }
  }
  return output
}

const runCycles = (input, cycles, dimensions) => {
  let state = loadMap(input, dimensions)
  for (let x = 0; x < cycles; x++) {
    state = iterateMap(state)
  }
  return state
}

console.log('Day 17: Test 1:', runCycles(test, 6, 3).size)
console.log('Day 17: Part 1:', runCycles(input, 6, 3).size)

console.log('Day 17: Test 2:', runCycles(test, 6, 4).size)
console.log('Day 17: Part 1:', runCycles(input, 6, 4).size)
