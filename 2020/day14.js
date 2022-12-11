const data = require('./inputs/day14.json')

const reduceMask = (mask) =>
  mask
    .split('')
    .map((m, i) => [i, +m])
    .filter((m) => !isNaN(m[1]))
const maskNumber = (memory, input, mask, total) => {
  total -= memory[input[1]] || 0
  const bits = input[2].toString(2).padStart(36, '0').split('')
  for (const m of mask) {
    bits[m[0]] = m[1]
  }
  memory[input[1]] = parseInt(bits.join(''), 2)
  total += memory[input[1]]
  return [memory, total]
}

const run = (instructions, reducer, maskCb) => {
  let mem = []
  let mask = []
  let total = 0
  for (const i of instructions) {
    switch (i[0]) {
      case 'mask':
        mask = reducer(i[1])
        break
      case 'mem':
        [mem, total] = maskCb(mem, i, mask, total)
        break
    }
  }
  return { mask, mem, total }
}

console.log(
  'Day 14: Test 1:',
  run(data.test1, reduceMask, maskNumber).total
)
console.log(
  'Day 14: Part 1:',
  run(data.input, reduceMask, maskNumber).total
)

const setBit = (number, pos, set = true) => {
  const bits = number.toString(2).padStart(36, '0').split('')
  bits[pos] = set ? 1 : 0
  return parseInt(bits.join(''), 2)
}

const floatBit = (number, pos) => {
  return [setBit(number, pos), setBit(number, pos, false)]
}

const maskMemory = (memory, input, mask, total) => {
  let addresses = [input[1]]
  for (let pos = 0; pos <= mask.length; pos++) {
    switch (mask.charAt(pos)) {
      case '0':
        continue
      case '1':
        addresses = addresses.map((a) => setBit(a, pos))
        break
      case 'X':
        addresses = addresses.flatMap((a) => floatBit(a, pos))
        break
    }
  }
  addresses.forEach((a) => {
    total += input[2] - (memory[a] || 0)
    memory[a] = input[2]
  })
  return [memory, total]
}

const test2 = run(data.test2, (a) => a, maskMemory)
console.log('Day 14: Test 2:', test2.total)
const part2 = run(data.input, (a) => a, maskMemory)
console.log('Day 14: Part 2:', part2.total)
