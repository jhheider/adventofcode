const _ = require('lodash')
const { test, input } = require('./inputs/day22.json')

const combat = (input) => {
  const [a, b] = [[...input[0]], [...input[1]]]
  while (a.length > 0 && b.length > 0) {
    const A = a.shift()
    const B = b.shift()
    A > B ? a.push(A, B) : b.push(B, A)
  }
  const s = score(a.length > 0 ? a : b)
  return [a, b, s]
}

const score = (input) =>
  input.reduce((s, v, i, a) => s + v * (a.length - i), 0)

console.log('Day 22: Test 1:', combat(test)[2])
console.log('Day 22: Part 1:', combat(input)[2])

const recursiveCombat = _.memoize((input) => {
  const [a, b] = [[...input[0]], [...input[1]]]
  const seen = new Set()
  while (a.length > 0 && b.length > 0) {
    const decks = [a.join(':'), b.join(':')].join(',')
    if (seen.has(decks)) return [a, b, 'Player 1', score(a)]
    seen.add(decks)
    const A = a.shift()
    const B = b.shift()
    if (a.length >= A && b.length >= B) {
      recursiveCombat([a.slice(0, A), b.slice(0, B)])[2] === 'Player 1'
        ? a.push(A, B)
        : b.push(B, A)
    } else {
      A > B ? a.push(A, B) : b.push(B, A)
    }
  }
  const s = (a.length > 0 ? a : b).reduce(
    (s, v, i, arr) => s + v * (arr.length - i),
    0
  )
  const winner = a.length > 0 ? 'Player 1' : 'Player 2'
  return [a, b, winner, s]
})

console.log('Day 22: Test 2:', recursiveCombat(test)[3])
console.log('Day 22: Part 2:', recursiveCombat(input)[3])
