const _ = require('lodash')
const { test, input } = require('./inputs/day21.json')

const parseData = (data) => {
  const map = {}
  const ingredients = []
  data.forEach((line) => {
    const i = line.replace(/ \(.*/, '').split(' ')
    ingredients.push(i)
    const allergens = line.match(/.* \(contains (.*)\)$/)[1].split(', ')
    allergens.forEach((a) => (map[a] = _.intersection(map[a] || i, i)))
  })
  return [map, _.flatten(ingredients)]
}

const findAllergens = (data) => {
  const found = {}
  const map = { ...data }
  let done = false
  while (!done) {
    done = true
    Object.keys(map).forEach((a) => {
      map[a] = map[a].filter((x) => !found[x])
      if (map[a].length > 1) done = false
      else {
        found[map[a][0]] = a
        delete map[a]
      }
    })
  }
  return found
}

const countSafe = (input) => {
  const [map, ingredients] = parseData(input)
  const allergens = findAllergens(map)
  return ingredients.filter((i) => !allergens[i]).length
}

console.log('Day 21: Test 1:', countSafe(test))
console.log('Day 21: Part 1:', countSafe(input))

const dangerous = (input) =>
  Object.entries(findAllergens(parseData(input)[0]))
    .sort((a, b) => (a[1] < b[1] ? -1 : 1))
    .map((a) => a[0])
    .join(',')

console.log('Day 21: Test 2:', dangerous(test))
console.log('Day 21: Part 2:', dangerous(input))
