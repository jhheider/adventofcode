const { test, input } = require('./inputs/day24.json')

const reduce = (directions) => {
  // https://www.redblobgames.com/grids/hexagons/#coordinates
  // This was very helpful in visualizing hexagonal representation
  const pos = [0, 0, 0]

  let charPos = 0
  while (charPos < directions.length) {
    switch (directions[charPos++]) {
      case 'w':
        pos[0]--
        pos[1]++
        break
      case 'e':
        pos[0]++
        pos[1]--
        break
      case 'n':
        pos[2]--
        switch (directions[charPos++]) {
          case 'w':
            pos[1]++
            break
          case 'e':
            pos[0]++
            break
        }
        break
      case 's':
        pos[2]++
        switch (directions[charPos++]) {
          case 'w':
            pos[0]--
            break
          case 'e':
            pos[1]--
            break
        }
        break
    }
  }
  return pos.join(':')
}

const walk = (input) => {
  const flipped = new Set([reduce(input[0])])
  for (let x = 1; x < input.length; x++) {
    const pos = reduce(input[x])
    if (flipped.has(pos)) flipped.delete(pos)
    else flipped.add(pos)
  }
  return flipped
}

console.log('Day 24: Test 1:', walk(test).size)
console.log('Day 24: Part 1:', walk(input).size)

const permute = (flipped) => {
  let begin = new Set(flipped)
  let end = new Set()
  for (let steps = 0; steps < 100; steps++) {
    begin.forEach((coords) => {
      const [cX, cY, cZ] = coords.split(':').map((i) => +i)
      for (let x = -1; x <= 1; x++) {
        for (let y = -1; y <= 1; y++) {
          for (let z = -1; z <= 1; z++) {
            if (x + y + z !== 0) continue
            const me = `${cX + x}:${cY + y}:${cZ + z}`
            const neighbors =
              begin.has(`${cX + x - 1}:${cY + y + 1}:${cZ + z + 0}`) +
              begin.has(`${cX + x - 1}:${cY + y + 0}:${cZ + z + 1}`) +
              begin.has(`${cX + x + 0}:${cY + y - 1}:${cZ + z + 1}`) +
              begin.has(`${cX + x + 0}:${cY + y + 1}:${cZ + z - 1}`) +
              begin.has(`${cX + x + 1}:${cY + y - 1}:${cZ + z + 0}`) +
              begin.has(`${cX + x + 1}:${cY + y + 0}:${cZ + z - 1}`)
            if (neighbors === 2 || (neighbors === 1 && begin.has(me))) { end.add(me) }
          }
        }
      }
    })
    begin = end
    end = new Set()
  }
  return begin
}

console.log('Day 24: Test 2:', permute(walk(test)).size)
console.log('Day 24: Part 2:', permute(walk(input)).size)
