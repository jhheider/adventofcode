const _ = require('lodash')
const data = require('./day6.json')

const normalize_data = data => {
  const answers = []
  let group = ""
  data.forEach(row => {
    if (row.length === 0) {
      answers.push(group)
      group = ""
      return
    }
    group += row
  })
  answers.push(group)
  return answers
}

const answers = normalize_data(data)
const counts = answers.map(a => _.uniq(a.split('')).length)

console.log('Part 1:', _.sum(counts))

const normalize_data2 = data => {
  const answers = []
  let group = []
  data.forEach(row => {
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

const answers2 = normalize_data2(data)
const counts2 = answers2.map(g => _.intersection(...g).length)
console.log('Part 2:', _.sum(counts2))

console.log(answers2[0], _.intersection(...answers2[0]))