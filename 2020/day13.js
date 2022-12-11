const data = require('./inputs/day13.json')

const computeBus = ([time, schedules]) => {
  const busses = schedules
    .split(',')
    .filter((b) => b !== 'x')
    .map((b) => +b)
  let departure = time
  while (true) {
    for (const b of busses) {
      if (departure % b === 0) { return [departure, b, departure - time, (departure - time) * b] }
    }
    departure += 1
  }
}

console.log('Day 13: Test 1:', computeBus(data.test))
console.log('Day 13: Part 1:', computeBus(data.input))

const computeSequence = (schedules) => {
  const busses = schedules
    .split(',')
    .map((b, i) => [i, b])
    .filter((b) => b[1] !== 'x')
    .map((b) => [b[0], +b[1]])
  let iterator = busses[0][1]
  let time = 0
  for (let b = 1; b < busses.length; b++) {
    while (true) {
      time += iterator
      if ((time + busses[b][0]) % busses[b][1] === 0) {
        iterator *= busses[b][1]
        break
      }
    }
  }
  return time
}

console.log('Day 13: Test 2:', computeSequence(data.test[1]))
data.more_tests.forEach((t, i) => {
  console.log(`Day 13: Test ${i + 3}:`, computeSequence(t))
})
console.log('Day 13: Part 2:', computeSequence(data.input[1]))
