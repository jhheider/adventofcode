const data = require('./day14.json')

const reduce_mask = mask => mask.split('').map((m, i) => [i, +m]).filter(m => !isNaN(m[1]))
const mask_number = (memory, input, mask, total) => {
  total -= memory[input[1]] || 0
  let bits = input[2].toString(2).padStart(36, '0').split('')
  for (let m of mask) {
    bits[m[0]] = m[1]
  }
  memory[input[1]] = parseInt(bits.join(''), 2)
  total += memory[input[1]]
  return [memory, total]
}

const run_program = (instructions, reducer_cb, mask_cb) => {
  let mem = []
  let mask = []
  let total = 0
  for (let i of instructions) {
    switch (i[0]) {
      case 'mask':
        mask = reducer_cb(i[1])
        break
      case 'mem':
        [mem, total] = mask_cb(mem, i, mask, total)
        break
    }
  }
  return { mask, mem, total }
}

console.log('Test 1:', run_program(data.test1, reduce_mask, mask_number))
console.log('Part 1:', run_program(data.input, reduce_mask, mask_number))

const set_bit = (number, pos, set = true) => {
  let bits = number.toString(2).padStart(36, '0').split('')
  bits[pos] = set ? 1 : 0
  return  parseInt(bits.join(''), 2)
}

const float_bit = (number, pos) => {
  return [set_bit(number, pos), set_bit(number, pos, false)]
}

const mask_memory = (memory, input, mask, total) => {
  let addresses = [input[1]]
  for (let pos = 0; pos <= mask.length; pos++) {
    switch (mask.charAt(pos)) {
      case '0':
        continue
        break
      case '1':
        addresses = addresses.map(a => set_bit(a, pos))
        break
      case 'X':
        addresses = addresses.flatMap(a => float_bit(a, pos))
        break
    }
  }
  addresses.forEach(a => {
    total += input[2] - (memory[a] || 0)
    memory[a] = input[2]
  })
  return [memory, total]
}

const test2 = run_program(data.test2, a => a, mask_memory)
console.log('Test 2:', test2)
const part2 = run_program(data.input, a => a, mask_memory)
console.log('Part 2:', part2.total)