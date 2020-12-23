const { test, input } = require('./inputs/day1.json')

const find_and_multiply2 = data => {
  for (let x = 0; x < data.length - 1; x++) {
    for (let y = x + 1; y < data.length; y++) {
      if (data[x] + data[y] === 2020) { return [data[x], data[y], data[x] * data[y]] }
    }
  }
}

const find_and_multiply3 = data => {
  for (let x = 0; x < data.length - 2; x++) {
    for (let y = x + 1; y < data.length - 1; y++) {
      for (let z = y + 1; z < data.length - 1; z++) {
        if (data[x] + data[y] + data[z] === 2020) { return [data[x], data[y], data[z], data[x] * data[y] * data[z]] }
      }
    }
  }
}

console.log('Test 1:', find_and_multiply2(test))
console.log('Part 1:', find_and_multiply2(input))

console.log('Test 2:', find_and_multiply3(test))
console.log('Part 2:', find_and_multiply3(input))
