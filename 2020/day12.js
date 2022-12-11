const data = require('./inputs/day12.json')

const ship = [0, 0, 90]
const waypoint = [10, 1]

const heading = (angle) => {
  switch (angle) {
    case 0:
      return 'N'
    case 90:
      return 'E'
    case 180:
      return 'S'
    case 270:
      return 'W'
  }
}

const normalize = (angle) => {
  if (angle < 0) angle = 360 + angle
  return angle % 360
}

const translate = (ship, instruction) => {
  const command = instruction.charAt(0)
  const value = +instruction.slice(1)
  switch (command) {
    case 'N':
      return [ship[0], ship[1] + value, ship[2]]
    case 'E':
      return [ship[0] + value, ship[1], ship[2]]
    case 'S':
      return [ship[0], ship[1] - value, ship[2]]
    case 'W':
      return [ship[0] - value, ship[1], ship[2]]
    case 'F':
      return translate(ship, `${heading(ship[2])}${value}`)
    case 'L':
      return [ship[0], ship[1], normalize(ship[2] - value)]
    case 'R':
      return [ship[0], ship[1], normalize(ship[2] + value)]
  }
}

const sail = (ship, instructions) =>
  instructions.reduce(
    (ship, instruction) => translate(ship, instruction),
    ship
  )

const manhattanDistance = ({ ship }) => Math.abs(ship[0]) + Math.abs(ship[1])

const test1 = sail(ship, data.test)
console.log('Day 12: Test 1:', test1, manhattanDistance({ ship: test1 }))
const part1 = sail(ship, data.input)
console.log('Day 12: Part 1:', part1, manhattanDistance({ ship: part1 }))

const heading2 = (waypoint, change) => {
  const [r, originalTheta] = [
    Math.sqrt(waypoint[0] ** 2 + waypoint[1] ** 2),
    Math.atan2(waypoint[0], waypoint[1])
  ]
  const newTheta = originalTheta + (Math.PI * change) / 180
  return [
    Math.round(r * Math.sin(newTheta)),
    Math.round(r * Math.cos(newTheta))
  ]
}

const translate2 = ({ ship, waypoint, instruction }) => {
  const command = instruction.charAt(0)
  const value = +instruction.slice(1)
  switch (command) {
    case 'N':
      return { ship, waypoint: [waypoint[0], waypoint[1] + value] }
    case 'E':
      return { ship, waypoint: [waypoint[0] + value, waypoint[1]] }
    case 'S':
      return { ship, waypoint: [waypoint[0], waypoint[1] - value] }
    case 'W':
      return { ship, waypoint: [waypoint[0] - value, waypoint[1]] }
    case 'F':
      return {
        ship: [ship[0] + value * waypoint[0], ship[1] + value * waypoint[1]],
        waypoint
      }
    case 'L':
      return { ship, waypoint: heading2(waypoint, -value) }
    case 'R':
      return { ship, waypoint: heading2(waypoint, value) }
  }
}

const sail2 = (configuration, instructions) =>
  instructions.reduce(
    (configuration, instruction) =>
      translate2({ ...configuration, instruction }),
    configuration
  )
const test2 = sail2({ ship, waypoint }, data.test)
console.log('Day 12: Test 2:', test2, manhattanDistance(test2))
const part2 = sail2({ ship, waypoint }, data.input)
console.log('Day 12: Part 2:', part2, manhattanDistance(part2))
