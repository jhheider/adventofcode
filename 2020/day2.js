const data = require('./inputs/day2.json')

let validPt1 = 0
let validPt2 = 0

data.forEach(([range, key, password]) => {
  const [low, high] = range.split('-')
  const re = new RegExp(key, 'g')
  const count = (password.match(re) || []).length
  if (count >= low && count <= high) {
    validPt1 += 1
  }

  const found =
    (password.charAt(low - 1) === key) + (password.charAt(high - 1) === key)
  if (found === 1) {
    validPt2 += 1
  }
})

console.log('Day 2: Part 1:', validPt1)
console.log('Day 2: Part 2:', validPt2)
