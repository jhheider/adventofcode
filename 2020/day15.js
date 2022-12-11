const data = {
  tests: [
    [0, 3, 6],
    [1, 3, 2],
    [2, 1, 3],
    [1, 2, 3],
    [2, 3, 1],
    [3, 2, 1],
    [3, 1, 2]
  ],
  input: [11, 0, 1, 10, 5, 19]
}

const compute = (seed, length) => {
  const last = new Map()
  seed.slice(0, -1).forEach((n, i) => {
    last.set(n, i)
  })
  let value = seed[seed.length - 1]
  for (let x = seed.length - 1; x < length - 1; x++) {
    const newValue = last.has(value) ? x - last.get(value) : 0
    last.set(value, x)
    value = newValue
  }
  return [last, value]
}

data.tests.forEach((t, i) => {
  console.log(`Day 15: Test ${i + 1}:`, compute(data.tests[i], 2020)[1])
})
console.log('Day 15: Part 1:', compute(data.input, 2020)[1])

data.tests.forEach((t, i) => {
  console.log(`Day 15: Test ${i + 8}:`, compute(data.tests[i], 30_000_000)[1])
})
console.log('Day 15: Part 2:', compute(data.input, 30_000_000)[1])
