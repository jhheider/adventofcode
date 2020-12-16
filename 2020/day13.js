const data = require('./day13.json')

const compute_bus = ([time, schedules]) => {
  const busses = schedules.split(',').filter(b => b !== 'x').map(b => +b)
  let departure = time
  while (true) {
    for (b of busses) {
      if (departure % b === 0) return [departure, b, departure - time, (departure - time) * b]
    }
    departure += 1
  }
}

console.log('Test 1:', compute_bus(data.test))
console.log('Part 1:', compute_bus(data.input))

const compute_sequence = (schedules) => {
  const busses = schedules.split(',').map((b, i) => [i, b]).filter(b => b[1] !== 'x').map(b => [b[0], +b[1]])
  let iterator = busses[0][1]
  let time = 0
  console.log(`Bus ${0}: ${busses[0]}, iterator: ${iterator}, time: ${time}`)
  foo: for (let b = 1; b < busses.length; b++) {
    while (true) {
      time += iterator
      if ((time + busses[b][0]) % busses[b][1] === 0) {
        iterator *= busses[b][1]
        console.log(`Bus ${b}: ${busses[b]}, iterator: ${iterator}, time: ${time}`)
        continue foo
      }
    }
  }
  return time
}

console.log('Test 2:', compute_sequence(data.test[1]))
data.more_tests.forEach((t, i) => {
  console.log(`Test ${i + 3}:`, compute_sequence(t))
})
console.log('Part 2:', compute_sequence(data.input[1]))