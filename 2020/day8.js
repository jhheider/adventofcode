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
].map(r => [...r, 0])

const run_program = ({ program, acc = 0, line = 0 }) => {
  if (line === program.length) {
    return { solution: `Program execution complete. acc = ${acc}`}
  }
  if (program[line][2] > 0) {
    return { solution: `Line ${line} being called a second time, acc = ${acc}` }
  }
  program[line][2] += 1
  switch(program[line][0]) {
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
while (program.solution === undefined) { program = run_program(program) }
console.log('Test 1:', program.solution)

program = { program: data.map(r => [...r, 0]) }
while (program.solution === undefined) { program = run_program(program) }
console.log('Part 1:', program.solution)

const original_program = data

foo: for(let x = 0; x <= data.length; x++) {
  if (original_program[x][0] === 'acc') { continue }
  let program = {
    program: original_program.map((r, i) => {
      if (i === x) { return [r[0] === 'nop' ? 'jmp' : 'nop', r[1], 0] }
      return [r[0], r[1], 0]
    })
  }

  console.log(`Toggling line ${x} from`, original_program[x], 'to', program.program[x])
  while(program.solution === undefined) { program = run_program(program) }
  console.log(program.solution)
  if (program.solution.match(/^Program/)) {
    break foo
  }
}