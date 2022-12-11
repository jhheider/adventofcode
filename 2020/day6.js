const _ = require('lodash')
const data = require('./inputs/day6.json')

const normalizeData = (data) => {
  const answers = []
  let group = ''
  data.forEach((row) => {
    if (row.length === 0) {
      answers.push(group)
      group = ''
      return
    }
    group += row
  })
  answers.push(group)
  return answers
}

const answers = normalizeData(data)
const counts = answers.map((a) => _.uniq(a.split('')).length)

console.log('Day 6: Part 1:', _.sum(counts))

const normalizeData2 = (data) => {
  const answers = []
  let group = []
  data.forEach((row) => {
    if (row.length === 0) {
      answers.push(group)
      group = []
      return
    }
    group.push(row.split(''))
  })
  answers.push(group)
  return answers
}

const answers2 = normalizeData2(data)
const counts2 = answers2.map((g) => _.intersection(...g).length)
console.log('Day 6: Part 2:', _.sum(counts2))
