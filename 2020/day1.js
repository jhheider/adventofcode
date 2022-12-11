const { test, input } = require('./inputs/day1.json')

const findAndMultiply2 = (data) => {
  for (let x = 0; x < data.length - 1; x++) {
    for (let y = x + 1; y < data.length; y++) {
      if (data[x] + data[y] === 2020) {
        return [data[x], data[y], data[x] * data[y]]
      }
    }
  }
}

const findAndMultiply3 = (data) => {
  for (let x = 0; x < data.length - 2; x++) {
    for (let y = x + 1; y < data.length - 1; y++) {
      for (let z = y + 1; z < data.length - 1; z++) {
        if (data[x] + data[y] + data[z] === 2020) {
          return [data[x], data[y], data[z], data[x] * data[y] * data[z]]
        }
      }
    }
  }
}

console.log('Day 1: Test 1:', findAndMultiply2(test))
console.log('Day 1: Part 1:', findAndMultiply2(input))

console.log('Day 1: Test 2:', findAndMultiply3(test))
console.log('Day 1: Part 2:', findAndMultiply3(input))
