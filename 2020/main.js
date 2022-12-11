const first = process.argv.slice(2)[0]
const days = first ? [first] : [...Array(25).keys()]

for (const day of days) {
  const dayPath = `./day${day + 1}.js`
  require(dayPath)
}
