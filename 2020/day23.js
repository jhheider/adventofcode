const { test, input } = require('./inputs/day23.json')

const wrappedDecrement = (numeral, size) =>
  `${((+numeral + size - 2) % size) + 1}`

const buildRing = (order) =>
  new Map(order.split('').map((v, i, arr) => [v, arr[(i + 1) % arr.length]]))
const buildRing2 = (order) => {
  let [start, finish] = [0, 0]
  const cups = new Map(
    order.split('').map((v, i, arr) => {
      if (i === 0) {
        start = v
      }
      if (i === 8) {
        finish = v
      }
      return [v, arr[(i + 1) % arr.length]]
    })
  )
  cups.set(finish, '10')
  for (let i = 10; i < 1_000_000; i++) {
    cups.set(`${i}`, `${i + 1}`)
  }
  cups.set('1000000', start)
  return cups
}

const result = (cups) => {
  let value = cups.get('1')
  while (value.slice(-1) !== '1') {
    value += cups.get(value.slice(-1))
  }
  return value.slice(0, -1)
}

const result2 = (cups) => {
  const r1 = cups.get('1')
  const r2 = cups.get(r1)
  return r1 * r2
}

const run = (order, runs, type2 = false) => {
  const cups = (type2 ? buildRing2 : buildRing)(order)
  let destination = cups.keys().next().value
  for (let turn = 0; turn < runs; turn++) {
    const pickUp = []
    for (let x = 0; x < 3; x++) {
      const cup = cups.get(x > 0 ? pickUp[x - 1] : destination)
      pickUp.push(cup)
    }
    cups.set(destination, cups.get(pickUp[2]))
    for (
      let x = wrappedDecrement(destination, cups.size);
      x !== destination;
      x = wrappedDecrement(x, cups.size)
    ) {
      if (pickUp.includes(x)) continue
      cups.set(pickUp[2], cups.get(x.toString()))
      cups.set(x.toString(), pickUp[0])
      destination = cups.get(destination)
      break
    }
  }
  return (type2 ? result2 : result)(cups)
}

console.log('Day 23: Test 1 (10 runs):', run(test, 10))
console.log('Day 23: Test 1 (100 runs):', run(test, 100))
console.log('Day 23: Part 1:', run(input, 100))

console.log('Day 23: Test 2:', run(test, 10_000_000, true))
console.log('Day 23: Part 2:', run(input, 10_000_000, true))
