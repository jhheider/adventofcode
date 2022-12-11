const _ = require('lodash')
const data = require('./inputs/day16.json')

const validValues = _.memoize((rules) => {
  const output = new Set()
  rules.forEach((rule) =>
    [
      ..._.range(rule[1], rule[2] + 1),
      ..._.range(rule[3], rule[4] + 1)
    ].forEach((r) => output.add(r))
  )
  return output
})

const computeError = (valid, tickets) => {
  let error = 0
  tickets.forEach((ticket) =>
    ticket.forEach((n) => {
      if (!valid.has(n)) {
        error += n
      }
    })
  )
  return error
}

const checkTickets = (input) =>
  computeError(validValues(input.rules), input.other_tickets)

console.log('Day 16: Test 1:', checkTickets(data.test1))
console.log('Day 16: Part 1:', checkTickets(data.input))

const ticketIsValid = (valid, ticket) =>
  ticket.reduce((r, t) => r && valid.has(t), true)

const validTickets = (input) =>
  input.other_tickets.filter((t) =>
    ticketIsValid(validValues(input.rules), t)
  )

const matchFields = (input) => {
  const tickets = [...validTickets(input), input.ticket]
  const fields = new Set(input.rules)
  let possibles = []
  _.range(0, fields.size).forEach((i) => {
    const p = new Set()
    fields.forEach((f) => p.add(f))
    possibles[i] = p
  })
  tickets.forEach((t) => {
    possibles = possibles.map((p, i) => {
      if (p.size === 1) return p
      p.forEach((r) => {
        if ((t[i] < r[1] || t[i] > r[2]) && (t[i] < r[3] || t[i] > r[4])) {
          p.delete(r)
        }
      })
      return p
    })
  })
  while (possibles.reduce((r, v) => Math.max(r, v.size), 1) > 1) {
    const found = []
    possibles
      .filter((p) => p.size === 1)
      .forEach((s) => s.forEach((v) => found.push(v)))
    possibles = possibles.map((p) => {
      if (p.size === 1) return p
      found.forEach((f) => p.delete(f))
      return p
    })
  }
  return possibles.map((p, i) => [Array.from(p)[0][0], input.ticket[i]])
}

const answer = (ticket) =>
  ticket.filter((t) => t[0].match(/^departure/)).reduce((r, t) => r * t[1], 1)

console.log('Day 16: Test 2:', matchFields(data.test2))
console.log('Day 16: Part 2:', answer(matchFields(data.input)))
