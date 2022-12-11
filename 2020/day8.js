const data = require('./inputs/day8.json')

const test1 = [
  ['nop', '+0'],
  ['acc', '+1'],
  ['jmp', '+4'],
  ['acc', '+3'],
  ['jmp', '-3'],
  ['acc', '-99'],
  ['acc', '+1'],
  ['jmp', '-4'],
  ['acc', '+6']
].map((r) => [...r, 0])

const run = ({ program, acc = 0, line = 0 }) => {
  if (line === program.length) {
    return { solution: `Program execution complete. acc = ${acc}` }
  }
  if (program[line][2] > 0) {
    return {
      solution: `Line ${line} being called a second time, acc = ${acc}`
    }
  }
  program[line][2] += 1
  switch (program[line][0]) {
    case 'acc':
      acc += +program[line][1]
    // fallthrough
    case 'nop':
      line += 1
      break
    case 'jmp':
      line += +program[line][1]
      break
  }
  return { program, acc, line }
}

let program = { program: test1 }
while (program.solution === undefined) {
  program = run(program)
}
console.log('Day 8: Test 1:', program.solution)

program = { program: data.map((r) => [...r, 0]) }
while (program.solution === undefined) {
  program = run(program)
}
console.log('Day 8: Part 1:', program.solution)

const originalProgram = data

for (let x = 0; x <= data.length; x++) {
  if (originalProgram[x][0] === 'acc') {
    continue
  }
  let program = {
    program: originalProgram.map((r, i) => {
      if (i === x) {
        return [r[0] === 'nop' ? 'jmp' : 'nop', r[1], 0]
      }
      return [r[0], r[1], 0]
    })
  }

  while (program.solution === undefined) {
    program = run(program)
  }
  if (program.solution.match(/^Program/)) {
    console.log(`Day 8: Part 2: ${program.solution}`)
    break
  }
}
