const { tests, inputs } = require('./day18.json')

const run_tape = input => {
  const regex = /\([0-9 \+\*]{3,}\)/
  let x = input.match(regex)
  while (x) {
    input = input.replace(regex, run_tape(x[0].slice(1, -1)))
    x = input.match(regex)
  }
  let tokens = input.split(' ')
  let value = +tokens[0]
  for (let pos = 1; pos < tokens.length - 1; pos += 2) {
    switch (tokens[pos]) {
      case '+':
        value += +tokens[pos + 1]
        break
      case '*':
        value *= +tokens[pos + 1]
        break
    }
  }
  return value
}

tests.forEach((t, i) => console.log(`Test ${i + 1}:`, run_tape(t)))
console.log('Part 1:', inputs.map(run_tape).reduce((a, b) => a + b))

const run_tape2 = input => {
  const parens_regex = /\([0-9 \+\*]{3,}\)/
  let parens = input.match(parens_regex)
  while (parens) {
    input = input.replace(parens_regex, run_tape2(parens[0].slice(1, -1)))
    parens = input.match(parens_regex)
  }
  const plus_regex = /([0-9]+) \+ ([0-9]+)/
  let plus = input.match(plus_regex)
  while (plus) {
    input = input.replace(plus_regex, +plus[1] + +plus[2])
    plus = input.match(plus_regex)
  }
  return input.split(' * ').reduce((t, i) => t *= +i, 1)
}

tests.forEach((t, i) => console.log(`Test ${i + 7}:`, run_tape2(t)))
console.log('Part 2:', inputs.map(run_tape2).reduce((a, b) => a + b))
