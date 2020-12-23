const _ = require('lodash')
const { test, input } = require('./day20.json')

const t2i = line => parseInt(line.replace(/#/g, '1').replace(/\./g, '0'), 2)

const column = (tile, number) => tile.map(r => r.split('')[number]).join('')

const flip = input => {
  const tile = [...input.tile].reverse()
  const edges = compute_edges(tile)
  return { tile, edges: input.edges, neighbors: input.neighbors }
}

const rotate = input => {
  const tile = input.tile.map((_, i) => column(input.tile, input.tile.length - i - 1))
  const edges = compute_edges(tile)
  return { tile, edges: edges, neighbors: tile.neighbors }
}

function* make_iterator(tile) {
  yield tile
  for (let x = 0; x < 7; x++) {
    if (x === 4) { tile.reverse() }
    tile = rotate(tile)
    yield tile
  }
}

const compute_edges = tile => {
  const length = tile.length
  const top = t2i(tile[0])
  const bottom = t2i(tile[length - 1])
  const left = t2i(column(tile, 0))
  const right = t2i(column(tile, length - 1))
  return [top, right, bottom, left]
}

const make_map = input => {
  const size = Math.sqrt(input.length)
  const tiles = new Map()
  const output = new Map()
  input.forEach(t => tiles.set(t.index, { tile: t.tile, edges: compute_edges(t.tile), neighbors: [null, null, null, null] }))
  const corner = tiles.entries().next()
  output.set(corner.value[0], corner.value[1])
  tiles.delete(corner.value[0])
  console.log(tiles, output)
  while (tiles.size > 0) {
  }
  return tiles
}

const compute_corners = input => {
  const tile_map = make_map(input)
  let output = 1
  tile_map.forEach((t, i) => { if (t.neighbors.filter(n => n).length === 2) output *= i })
  return output
}

const fill_map = input => {
  const map = make_map(input)
  let filled = []
  for (let [index, tile] of map) {
    if (tile.neighbors[0] === null && tile.neighbors[3] === null) {
      filled[0] = [index]
      break
    }
  }
  const size = Math.sqrt(input.length)
  for (let y = 0; y < size; y++) {
    for (let x = 0; x < size; x++) {
      if (x === 0 && y === 0) continue
      if (x > 0) filled[y][x] = map.get(filled[y][x - 1]).neighbors[1]
      else filled[y] = [map.get(filled[y - 1][x]).neighbors[2]]
    }
  }
  filled = filled.map(r => r.map(c => strip(map.get(c).tile)))
  const output = []
  for (let y = 0; y < size; y++) {
    for (let c = 0; c < filled[0][0].length; c++) {
      const row = []
      for (let x = 0; x < size; x++) {
        row.push(...filled[y][x][c].split(''))
      }
      output.push(row)
    }
  }
  return output
}

const strip = map => map.slice(1, -1).map(r => r.slice(1, -1))

const has_serpent = (map, y, x) =>
  map[y + 1][x + 0] === '#' &&
  map[y + 2][x + 1] === '#' &&
  map[y + 2][x + 4] === '#' &&
  map[y + 1][x + 5] === '#' &&
  map[y + 1][x + 6] === '#' &&
  map[y + 2][x + 7] === '#' &&
  map[y + 2][x + 10] === '#' &&
  map[y + 1][x + 11] === '#' &&
  map[y + 1][x + 12] === '#' &&
  map[y + 2][x + 13] === '#' &&
  map[y + 2][x + 16] === '#' &&
  map[y + 1][x + 17] === '#' &&
  map[y + 1][x + 18] === '#' &&
  map[y + 1][x + 19] === '#' &&
  map[y + 0][x + 18] === '#'

const count_serpents = map => {
  let serpents = 0
  for (let f = 0; f < 2; f++) {
    for (let r = 0; r < 4; r++) {
      for (let y = 0; y < map.length - 2; y++) {
        for (let x = 0; x < map.length - 19; x++) {
          console.log(f, r, y, x)
          if (has_serpent(map, y, x)) serpents += 1
        }
      }
      if (serpents > 0) {
        console.log(map.map(r => r.join('')).join('\n'))
        return serpents
      }
    }
    map = map.map((r, y) => r.map((c, x) => map[map.length - x - 1][y]))
    console.log(map)
  }
  map.reverse()
}



const start = process.hrtime()
console.log('Test 1:', compute_corners(test))
console.log(`${process.hrtime(start)}s...`)
console.log('Part 1:', compute_corners(input))
console.log(`${process.hrtime(start)}s...`)

console.log('Test 2:', count_serpents(fill_map(test)))
console.log(`${process.hrtime(start)}s...`)
