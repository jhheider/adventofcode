const data = require('./inputs/day7.json')

const tree_bags = color => {
  let contains = new Set([color])
  let size
  do {
    size = contains.size
    contains.forEach(color => data.forEach(d => { if (d[1].match(new RegExp(color))) contains.add(d[0].slice(0, -1)) } ))
  } while (size !== contains.size)
  contains.delete(color)
  return contains
}

console.log('Part 1:', tree_bags('shiny gold').size)

const contains = color => {
  const contents = data.find(d => d[0] === `${color} bags`)[1]
  if (contents === 'no other bags.') { return 0 }
  return contents.split(', ').map(c => {
    const num = +c.split(' ')[0]
    const color = c.match(/[0-9]+ (.*) bag.*$/)[1]
    return num + num * contains(color)
  }).reduce((a, b) => a + b)
}

console.log('Part 2:', contains('shiny gold'))