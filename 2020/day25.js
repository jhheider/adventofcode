const { test, input } = require('./inputs/day25.json')

function* make_looper() {
  let value = 1
  let values = new Map()
  for (let i = 1; ; i++) {
    value = (value * 7) % 20201227
    yield values.set(value, i)
  }
  return values
}
const compute_loop = input => {
  const loop = make_looper()
  let value = loop.next().value
  if (value.get(input)) return loop.value.get(input)
  for (value = loop.next().value.get(input); !value; value = loop.next().value.get(input)) {}
  return value
}

const compute_key = input => {
  const loops = input.map(compute_loop)
  let value = 1
  for (let i = 0; i < loops[1]; i++) {
    value = (value * input[0]) % 20201227
  }
  return value
}

console.log('Test 1:', compute_key(test))
console.log('Part 1:', compute_key(input))