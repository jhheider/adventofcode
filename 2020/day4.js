const data = require('./inputs/day4.json')

const normalize_passports = data => {
  const passports = []
  let passport = { byr: null, iyr: null, eyr: null, hgt: null, hcl: null, ecl: null, pid: null, cid: null }
  data.forEach(row => {
    if (row.length === 0) {
      passports.push(passport)
      passport = { byr: null, iyr: null, eyr: null, hgt: null, hcl: null, ecl: null, pid: null, cid: null }
      return
    }
    const fields = row.split(' ')
    fields.forEach(field => {
      const [key, value] = field.split(':')
      passport[key] = value
    })
  })
  passports.push(passport)
  return passports
}

const passports = normalize_passports(data)

const validatePt1 = passport =>
  passport.byr != null &&
    passport.iyr != null &&
    passport.eyr != null &&
    passport.hgt != null &&
    passport.hcl != null &&
    passport.ecl != null &&
    passport.pid != null

const pt1 = passports.filter(validatePt1).length

console.log('Part 1:', pt1)

const validatePt2 = passport => {
  const byr = +passport.byr
  if (byr < 1920 || byr > 2002) return false

  const iyr = +passport.iyr
  if (iyr < 2010 || iyr > 2020) return false

  const eyr = +passport.eyr
  if (eyr < 2020 || eyr > 2030) return false

  const hgt = +passport.hgt.slice(0, -2)
  if (passport.hgt.match(/^[0-9]+cm$/)) {
    if (hgt < 150 || hgt > 193) return false
  } else if (passport.hgt.match(/^[0-9]+in$/)) {
    if (hgt < 59 || hgt > 76) return false
  } else { return false }

  if (!passport.hcl.match(/^#[0-9a-f]{6}$/)) return false
  if (!['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(passport.ecl)) return false
  if (!passport.pid.match(/^[0-9]{9}$/)) return false
  return true
}

passports.filter(validatePt1).forEach(passport => { console.log(passport.byr, validatePt2(passport))})
let pt2 = passports.filter(validatePt1).filter(validatePt2).length

console.log('Part 2:', pt2)