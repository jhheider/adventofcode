use num::Integer;
use std::cmp::max;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

struct Character {
    hp: i32,
    damage: i32,
    armor: i32,
}

struct Stats {
    cost: i32,
    damage: i32,
    armor: i32,
}

trait Statted {
    fn stats(&self) -> Stats;
}

#[derive(EnumIter)]
enum Weapon {
    Dagger,
    Shortsword,
    Warhammer,
    Longsword,
    Greataxe,
}

impl Statted for Weapon {
    fn stats(&self) -> Stats {
        match *self {
            Weapon::Dagger => Stats {
                cost: 8,
                damage: 4,
                armor: 0,
            },
            Weapon::Shortsword => Stats {
                cost: 10,
                damage: 5,
                armor: 0,
            },
            Weapon::Warhammer => Stats {
                cost: 25,
                damage: 6,
                armor: 0,
            },
            Weapon::Longsword => Stats {
                cost: 40,
                damage: 7,
                armor: 0,
            },
            Weapon::Greataxe => Stats {
                cost: 74,
                damage: 8,
                armor: 0,
            },
        }
    }
}

#[derive(EnumIter)]
enum Armor {
    None,
    Leather,
    Chainmail,
    Splintmail,
    Bandedmail,
    Platemail,
}

impl Statted for Armor {
    fn stats(&self) -> Stats {
        match *self {
            Armor::None => Stats {
                cost: 0,
                damage: 0,
                armor: 0,
            },
            Armor::Leather => Stats {
                cost: 13,
                damage: 0,
                armor: 1,
            },
            Armor::Chainmail => Stats {
                cost: 31,
                damage: 0,
                armor: 2,
            },
            Armor::Splintmail => Stats {
                cost: 53,
                damage: 0,
                armor: 3,
            },
            Armor::Bandedmail => Stats {
                cost: 75,
                damage: 0,
                armor: 4,
            },
            Armor::Platemail => Stats {
                cost: 102,
                damage: 0,
                armor: 5,
            },
        }
    }
}

#[derive(EnumIter, PartialEq)]
enum Ring {
    None,
    Damage1,
    Damage2,
    Damage3,
    Defence1,
    Defence2,
    Defence3,
}

impl Statted for Ring {
    fn stats(&self) -> Stats {
        match *self {
            Ring::None => Stats {
                cost: 0,
                damage: 0,
                armor: 0,
            },
            Ring::Damage1 => Stats {
                cost: 25,
                damage: 1,
                armor: 0,
            },
            Ring::Damage2 => Stats {
                cost: 50,
                damage: 2,
                armor: 0,
            },
            Ring::Damage3 => Stats {
                cost: 100,
                damage: 3,
                armor: 0,
            },
            Ring::Defence1 => Stats {
                cost: 20,
                damage: 0,
                armor: 1,
            },
            Ring::Defence2 => Stats {
                cost: 40,
                damage: 0,
                armor: 2,
            },
            Ring::Defence3 => Stats {
                cost: 80,
                damage: 0,
                armor: 3,
            },
        }
    }
}

pub fn main() {
    let test = Character {
        hp: 12,
        damage: 7,
        armor: 2,
    };
    let input = Character {
        hp: 104,
        damage: 8,
        armor: 1,
    };

    let test1 = rounds_to_win(
        &Character {
            hp: 8,
            damage: 5,
            armor: 5,
        },
        &test,
    );
    assert_eq!(test1, Some(4));
    println!("Day 21: Test 1: PC wins after {} rounds", test1.unwrap());

    let part1 = optimize(&input);
    assert_eq!(part1, 78);
    println!("Day 21: Part 1: PC wins by spending only {}gp", part1);

    let part2 = deoptimize(&input);
    assert_eq!(part2, 148);
    println!("Day 21: Part 2: PC loses after spending {}gp", part2);
}

fn dpr(pc: &Character, boss: &Character) -> (i32, i32) {
    (
        max(1, pc.damage - boss.armor),
        max(1, boss.damage - pc.armor),
    )
}

#[allow(unstable_name_collisions)]
fn rounds_to_win(pc: &Character, boss: &Character) -> Option<i32> {
    let damage_per_round = dpr(pc, boss);
    let pc_rounds_to_win = boss.hp.div_ceil(&damage_per_round.0);
    let boss_rounds_to_win = pc.hp.div_ceil(&damage_per_round.1);
    if boss_rounds_to_win < pc_rounds_to_win {
        return None;
    }
    Some(pc_rounds_to_win)
}

fn optimize(boss: &Character) -> i32 {
    let mut best_cost = i32::MAX;

    for weapon in Weapon::iter() {
        for armor in Armor::iter() {
            for left_ring in Ring::iter() {
                for right_ring in Ring::iter() {
                    if (left_ring == right_ring) && (left_ring != Ring::None) {
                        continue;
                    }

                    let cost = weapon.stats().cost
                        + armor.stats().cost
                        + left_ring.stats().cost
                        + right_ring.stats().cost;
                    if cost > best_cost {
                        continue;
                    }

                    let pc = Character {
                        hp: 100,
                        damage: weapon.stats().damage
                            + left_ring.stats().damage
                            + right_ring.stats().damage,
                        armor: armor.stats().armor
                            + left_ring.stats().armor
                            + right_ring.stats().armor,
                    };
                    match rounds_to_win(&pc, boss) {
                        None => continue,
                        Some(_) => best_cost = cost,
                    };
                }
            }
        }
    }
    best_cost
}

fn deoptimize(boss: &Character) -> i32 {
    let mut worst_cost = i32::MIN;

    for weapon in Weapon::iter() {
        for armor in Armor::iter() {
            for left_ring in Ring::iter() {
                for right_ring in Ring::iter() {
                    if (left_ring == right_ring) && (left_ring != Ring::None) {
                        continue;
                    }

                    let cost = weapon.stats().cost
                        + armor.stats().cost
                        + left_ring.stats().cost
                        + right_ring.stats().cost;
                    if cost < worst_cost {
                        continue;
                    }

                    let pc = Character {
                        hp: 100,
                        damage: weapon.stats().damage
                            + left_ring.stats().damage
                            + right_ring.stats().damage,
                        armor: armor.stats().armor
                            + left_ring.stats().armor
                            + right_ring.stats().armor,
                    };
                    match rounds_to_win(&pc, boss) {
                        None => worst_cost = cost,
                        Some(_) => continue,
                    };
                }
            }
        }
    }
    worst_cost
}
