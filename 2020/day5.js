const data = require('./inputs/day5.json')

const decodeSeat = (seatCode) => {
  let row = 0
  let seat = 0
  for (let x = 0; x <= 6; x++) {
    row += seatCode.charAt(x) === 'B' ? 2 ** (6 - x) : 0
  }
  for (let x = 7; x <= 9; x++) {
    seat += seatCode.charAt(x) === 'R' ? 2 ** (9 - x) : 0
  }
  return [row, seat, row * 8 + seat]
}

const pt1 = data.map(decodeSeat).reduce((a, b) => Math.max(a, b[2]), 0)

console.log('Day 5: Part 1:', pt1)

const ids = data
  .map(decodeSeat)
  .map((a) => a[2])
  .sort((a, b) => a - b)

for (let x = 1; x < 1023; x++) {
  if (ids.includes(x - 1) && !ids.includes(x) && ids.includes(x + 1)) {
    console.log('Day 5: Part 2:', x)
  }
}
