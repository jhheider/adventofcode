const { tests, inputs } = require('./inputs/day18.json')

const runTape = (input) => {
  const regex = /\([0-9 +*]{3,}\)/
  let x = input.match(regex)
  while (x) {
    input = input.replace(regex, runTape(x[0].slice(1, -1)))
    x = input.match(regex)
  }
  const tokens = input.split(' ')
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

tests.forEach((t, i) => console.log(`Day 18: Test ${i + 1}:`, runTape(t)))
console.log(
  'Day 18: Part 1:',
  inputs.map(runTape).reduce((a, b) => a + b)
)

const runTape2 = (input) => {
  const parensRegex = /\([0-9 +*]{3,}\)/
  let parens = input.match(parensRegex)
  while (parens) {
    input = input.replace(parensRegex, runTape2(parens[0].slice(1, -1)))
    parens = input.match(parensRegex)
  }
  const plusRegex = /([0-9]+) \+ ([0-9]+)/
  let plus = input.match(plusRegex)
  while (plus) {
    input = input.replace(plusRegex, +plus[1] + +plus[2])
    plus = input.match(plusRegex)
  }
  return input.split(' * ').reduce((t, i) => (t *= +i), 1)
}

tests.forEach((t, i) => console.log(`Day 18: Test ${i + 7}:`, runTape2(t)))
console.log(
  'Day 18: Part 2:',
  inputs.map(runTape2).reduce((a, b) => a + b)
)
