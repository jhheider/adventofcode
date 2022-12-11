const { tests, input } = require('./inputs/day19.json')

const makeDictionary = (rules) => {
  const rulesMap = new Map()
  rules.forEach((r) => {
    const [i, v] = r.split(': ')
    const pairs = v.split(' | ')
    const rule = { conditions: [] }
    pairs.forEach((p, n) => {
      if (p.match(/"/)) {
        rule.char = p.charAt(1)
      } else {
        rule.conditions[n] = p.split(' ').map((v) => +v)
      }
      rulesMap.set(+i, rule)
    })
  })
  return rulesMap
}

const ruleMatches = (strings, ruleNumber, dictionary) => {
  const rule = dictionary.get(ruleNumber)
  if (rule.char) {
    return strings
      .filter((s) => s.charAt(0) === rule.char)
      .map((s) => s.slice(1))
  }
  const matches = []
  rule.conditions.forEach((c) => {
    strings.forEach((s) => {
      if (s === '') return
      let remains = [s]
      for (const r of c) {
        remains = ruleMatches(remains, r, dictionary)
      }
      matches.push(...remains)
    })
  })
  return matches
}

const testValues = ({ rules, data }) => {
  const dictionary = makeDictionary(rules)
  return ruleMatches(data, 0, dictionary).filter((m) => m === '').length
}

console.log('Day 19: Test 1:', testValues(tests[0]))
console.log('Day 19: Part 1:', testValues(input))

const testValues2 = ({ rules, data }) => {
  const dictionary = makeDictionary(rules)
  dictionary.set(8, { conditions: [[42], [42, 8]] })
  dictionary.set(11, {
    conditions: [
      [42, 31],
      [42, 11, 31]
    ]
  })
  return ruleMatches(data, 0, dictionary).filter((m) => m === '').length
}

console.log('Day 19: Test 2:', testValues(tests[1]))
console.log('Day 19: Test 3:', testValues2(tests[1]))
console.log('Day 19: Part 2:', testValues2(input))
