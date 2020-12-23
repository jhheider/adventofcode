const numeric = (a, b) => a - b

const data = require('./inputs/day10.json').sort(numeric)

const test1 = [0, 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 22].sort(numeric)
const test2 = [0, 28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3, 51].sort(numeric)

const mapper = (r, i, a) => i === 0 ? 0 : r - a[i - 1]
const reducer = (acc, val) => { if (val !== 0) { acc[val - 1] += 1; acc[3] += val }; return acc }

const count_diffs = data => data.map(mapper).reduce(reducer, [0, 0, 0, 0])

console.log('Test 1:', count_diffs(test1))
console.log('Test 2:', count_diffs(test2))
const part1 = count_diffs(data)
console.log('Part 1:', part1, part1[0] * part1[2])

data.push(data[data.length - 1])
data.unshift(0)

const multipler = val => {
  const length = Math.min(val.length, 4)
  switch (length) {
    case 4: return 7
    case 3: return 4
    case 2: return 2
    default: return 1
  }
}

const mult = (a, b) => a * b

const permute = data => data.map(mapper).slice(1).join('').split('3').map(multipler).reduce(mult)


console.log('Test 3:', permute(test1))
console.log('Test 4:', permute(test2))
console.log('Part 2:', permute(data))
