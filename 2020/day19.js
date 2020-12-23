const { tests, input } = require('./inputs/day19.json')

const make_dictionary = rules => {
  const rules_map = new Map()
  rules.forEach(r => {
    const [i, v] = r.split(': ')
    const pairs = v.split(' | ')
    const rule = { conditions: [] }
    pairs.forEach((p, n) => {
      if (p.match(/"/)) { rule.char = p.charAt(1) }
      else { rule.conditions[n] = p.split(' ').map(v => +v) }
      rules_map.set(+i, rule)
    })
  })
  return rules_map
}

const rule_matches = (strings, rule_number, dictionary) => {
  const rule = dictionary.get(rule_number)
  if (rule.char) {
    return strings.filter(s => s.charAt(0) === rule.char).map(s => s.slice(1))
  }
  const matches = []
  rule.conditions.forEach(c => {
    strings.forEach(s => {
      if (s === '') return
      let remains = [s]
      for (let r of c) {
        remains = rule_matches(remains, r, dictionary)
      }
      matches.push(...remains)
    })
  })
  return matches
}

const test_values = ({ rules, data }) => {
  const dictionary = make_dictionary(rules)
  return rule_matches(data, 0, dictionary).filter(m => m === '').length
}

console.log('Test 1:', test_values(tests[0]))
console.log('Part 1:', test_values(input))

const test_values2 = ({ rules, data }) => {
  const dictionary = make_dictionary(rules)
  dictionary.set(8, { conditions: [[42], [42, 8]] })
  dictionary.set(11, { conditions: [[42, 31], [42, 11, 31]] })
  return rule_matches(data, 0, dictionary).filter(m => m === '').length
}

console.log('Test 2:', test_values(tests[1]))
console.log('Test 3:', test_values2(tests[1]))
console.log('Part 2:', test_values2(input))
