const { test, input } = require('./inputs/day25.json')

function * makeLooper () {
  let value = 1
  const values = new Map()
  for (let i = 1; ; i++) {
    value = (value * 7) % 20201227
    yield values.set(value, i)
  }
}
const computeLoop = (input) => {
  const loop = makeLooper()
  let value = loop.next().value
  if (value.get(input)) return loop.value.get(input)
  for (
    value = loop.next().value.get(input);
    !value;
    value = loop.next().value.get(input)
  ) { // noop
  }
  return value
}

const computeKey = (input) => {
  const loops = input.map(computeLoop)
  let value = 1
  for (let i = 0; i < loops[1]; i++) {
    value = (value * input[0]) % 20201227
  }
  return value
}

console.log('Day 25: Test 1:', computeKey(test))
console.log('Day 25: Part 1:', computeKey(input))
