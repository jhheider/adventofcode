const { values } = require('lodash')
const _ = require('lodash')
const data = require('./inputs/day16.json')

const valid_values = _.memoize(rules => {
  const output = new Set()
  rules.forEach(rule => [..._.range(rule[1], rule[2] + 1), ..._.range(rule[3], rule[4] + 1)].forEach(r => output.add(r)))
  return output
})

const compute_error = (valid, tickets) => {
  let error = 0
  tickets.forEach(ticket => ticket.forEach(n => { if (!valid.has(n)) { error += n } }))
  return error
}

const check_tickets = input => compute_error(valid_values(input.rules), input.other_tickets)

console.log('Test 1:', check_tickets(data.test1))
console.log('Part 1:', check_tickets(data.input))

const ticket_is_valid = (valid, ticket) => ticket.reduce((r, t) => r && valid.has(t), true)

const valid_tickets = input => input.other_tickets.filter(t => ticket_is_valid(valid_values(input.rules), t))

const match_fields = input => {
  const tickets = [...valid_tickets(input), input.ticket]
  const fields = new Set(input.rules)
  let possibles = []
  _.range(0, fields.size).forEach(i => { const p = new Set(); fields.forEach(f => p.add(f)); possibles[i] = p })
  tickets.forEach(t => {
    possibles = possibles.map((p, i) => {
      if (p.size === 1) return p
      p.forEach(r => {
        if ((t[i] < r[1] || t[i] > r[2]) && (t[i] < r[3] || t[i] > r[4])) {
          p.delete(r)
        }
      })
      return p
    })
  })
  while (possibles.reduce((r, v) => Math.max(r, v.size), 1) > 1) {
    const found = []
    possibles.filter(p => p.size === 1).forEach(s => s.forEach(v => found.push(v)))
    possibles = possibles.map(p => {
      if (p.size === 1) return p
      found.forEach(f => p.delete(f))
      return p
    })
  }
  return possibles.map((p, i) => [Array.from(p)[0][0], input.ticket[i]])
}

const answer = ticket => ticket.filter(t => t[0].match(/^departure/)).reduce((r, t) => r * t[1], 1)

console.log('Test 2:', match_fields(data.test2))
console.log('Part 2:', answer(match_fields(data.input)))