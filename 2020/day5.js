const data = require('./inputs/day5.json')

const decode_seat = seat_code => {
  let row = 0
  let seat = 0
  for(let x = 0; x <= 6; x++) {
    row += seat_code.charAt(x) === 'B' ? 2**(6 - x) : 0
  }
  for(let x = 7; x <= 9; x++) {
    seat += seat_code.charAt(x) === 'R' ? 2**(9 - x) : 0
  }
  return [row, seat, row * 8 + seat]
}

const pt1 = data.map(decode_seat).reduce((a, b) => Math.max(a, b[2]), 0)

console.log('Part 1:', pt1)

const ids = data.map(decode_seat).map(a => a[2]).sort((a, b) => a - b)

for(let x = 1; x < 1023; x++) {
  if (ids.includes(x - 1) && !ids.includes(x) && ids.includes(x + 1)) {
    console.log('Part 2:', x)
  }
}