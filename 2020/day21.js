const _ = require('lodash')
const { test, input } = require('./day21.json')

const parse_data = data => {
  const map = {}
  const ingredients = []
  data.forEach(line => {
    const i = line.replace(/ \(.*/, '').split(' ')
    ingredients.push(i)
    const allergens = line.match(/.* \(contains (.*)\)$/)[1].split(', ')
    allergens.forEach(a => map[a] = _.intersection(map[a] || i, i))
  })
  return [map, _.flatten(ingredients)]
}

const find_allergens = data => {
  const found = {}
  const map = {...data}
  let done = false
  while (!done) {
    done = true
    Object.keys(map).forEach(a => {
      map[a] = map[a].filter(x => !found[x])
      if (map[a].length > 1) done = false
      else {
        found[map[a][0]] = a
        delete map[a]
      }
    })
  }
  return found
}

const count_safe = input => {
  const [map, ingredients] = parse_data(input)
  const allergens = find_allergens(map)
  return ingredients.filter(i => !allergens[i]).length
}

console.log('Test 1:', count_safe(test))
console.log('Part 1:', count_safe(input))

const dangerous = input => Object.entries(find_allergens(parse_data(input)[0])).sort((a, b) => a[1] < b[1] ? -1 : 1).map(a => a[0]).join(',')

console.log('Test 2:', dangerous(test))
console.log('Part 2:', dangerous(input))
