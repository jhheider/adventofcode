const { test, input } = require('./day17.json')

const p2a = point => point.split(':').map(n => +n)
const a2p = array => array.join(':')
const make_point = (x, y, z) => a2p([x, y, z])
const make_point_4d = (x, y, z, w) => a2p([x, y, z, w])
const dimensionality = input => p2a(input.values().next().value).length

const load_map = (input, dimensions) => {
  const output = new Set()
  input.forEach((r, y) => r.split('').forEach((c, x) => c === '#' && output.add(dimensions === 4 ? make_point_4d(x, y, 0, 0) : make_point(x, y, 0))))
  return output
}

const check_adjacent = (input, point) => {
  const [pX, pY, pZ, pW] = p2a(point)
  let adjacent = 0
  for (let x = -1; x <= 1; x++) {
    for (let y = -1; y <= 1; y++) {
      for (let z = -1; z <= 1; z++) {
        if (pW !== undefined) {
          for (let w = -1; w <= 1; w++) {
            const check_point = make_point_4d(pX + x, pY + y, pZ + z, pW + w)
            if (check_point !== point && input.has(check_point)) adjacent += 1
          }
        } else {
          const check_point = make_point(pX + x, pY + y, pZ + z)
          if (check_point !== point && input.has(check_point)) adjacent += 1
        }
      }
    }
  }
  return adjacent
}

const bounds = input =>
  Array.from(input).reduce((output, point) => {
    let [x, y, z, w] = p2a(point)
    return [
      Math.max(output[0], x),
      Math.min(output[1], x),
      Math.max(output[2], y),
      Math.min(output[3], y),
      Math.max(output[4], z),
      Math.min(output[5], z),
      w !== undefined ? Math.max(output[6], w) : 0,
      w !== undefined ? Math.min(output[7], w) : 0,
    ]
  }, [0, 0, 0, 0, 0, 0, 0, 0])

const iterate_map = input => {
  const dimensions = dimensionality(input)
  const [highX, lowX, highY, lowY, highZ, lowZ, highW, lowW] = bounds(input)
  const output = new Set()
  for (let z = lowZ - 1; z <= highZ + 1; z++) {
    for (let y = lowY - 1; y <= highY + 1; y++) {
      for (let x = lowX - 1; x <= highX + 1; x++) {
        if (dimensions === 4) {
          for (let w = lowW - 1; w <= highW + 1; w++) {
            const p = make_point_4d(x, y, z, w)
            const count = check_adjacent(input, p)
            if ((input.has(p) && count === 2) || count === 3) { output.add(p) }
          }
        } else {
          const p = make_point(x, y, z)
          const count = check_adjacent(input, p)
          if ((input.has(p) && count === 2) || count === 3) { output.add(p) }
        }
      }
    }
  }
  return output
}

const print_map = input => {
  const dimensions = dimensionality(input)
  const [highX, lowX, highY, lowY, highZ, lowZ, highW, lowW] = bounds(input)
  for (let w = lowW; w <= highW; w++) {
    for (let z = lowZ; z <= highZ; z++) {
      console.log(`z=${z}${dimensions === 4 ? `, w=${w}` : ''}, y=${lowY}, x=${lowX}`)
      for (let y = lowY; y <= highY; y++) {
        let line = ''
        for (let x = lowX; x <= highX; x++) {
          line += input.has(dimensions === 4 ? make_point_4d(x, y, z, w) : make_point(x, y, z)) ? '#' : '.'
        }
        console.log(line)
      }
      console.log()
    }
  }
}

const run_cycles = (input, cycles, dimensions) => {
  let state = load_map(input, dimensions)
  for (let x = 0; x < cycles; x++) {
    state = iterate_map(state)
  }
  return state
}

console.log('Test 1:', run_cycles(test, 6, 3).size)
console.log('Part 1:', run_cycles(input, 6, 3).size)

console.log('Test 2:', run_cycles(test, 6, 4).size)
console.log('Part 1:', run_cycles(input, 6, 4).size)
