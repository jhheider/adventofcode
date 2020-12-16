const data = require('./day2.json')

var validPt1 = 0
var validPt2 = 0

data.forEach(([range, key, password]) => {
  const [low, high] = range.split('-')
  const re = new RegExp(key, 'g')
  const count = (password.match(re) || []).length
  if (count >= low && count <= high) { validPt1 += 1 }

  let found = (password.charAt(low - 1) === key) + (password.charAt(high - 1) === key)
  if (found === 1) { validPt2 += 1 }
})

console.log('Part 1:', validPt1)
console.log('Part 2:', validPt2)