use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct GameState {
    hp: isize,
    mana: isize,
    boss_hp: isize,
    boss_damage: usize,
    shield: usize,
    poison: usize,
    recharge: usize,
    mana_spent: usize,
    is_hard: bool,
}

#[derive(Clone, Debug)]
struct Spell {
    cost: usize,
    effect: Effect,
}

#[derive(Clone, Debug)]
enum Effect {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl GameState {
    fn new(hp: isize, mana: isize, boss_hp: isize, boss_damage: usize, is_hard: bool) -> Self {
        GameState {
            hp,
            mana,
            boss_hp,
            boss_damage,
            shield: 0,
            poison: 0,
            recharge: 0,
            mana_spent: 0,
            is_hard,
        }
    }

    fn apply_effects(&mut self) {
        if self.shield > 0 {
            self.shield -= 1;
        }
        if self.poison > 0 {
            self.boss_hp -= 3;
            self.poison -= 1;
        }
        if self.recharge > 0 {
            self.mana += 101;
            self.recharge -= 1;
        }
    }

    fn apply_spell(&mut self, spell: &Spell) {
        self.mana -= spell.cost as isize;
        self.mana_spent += spell.cost;
        match spell.effect {
            Effect::MagicMissile => self.boss_hp -= 4,
            Effect::Drain => {
                self.boss_hp -= 2;
                self.hp += 2;
            }
            Effect::Shield => self.shield = 6,
            Effect::Poison => self.poison = 6,
            Effect::Recharge => self.recharge = 5,
        }
    }

    fn is_won(&self) -> bool {
        self.boss_hp <= 0
    }

    fn is_lost(&self) -> bool {
        self.hp <= 0 || self.mana < 0
    }

    fn is_over(&self) -> bool {
        self.is_won() || self.is_lost()
    }

    fn is_valid(&self) -> bool {
        self.mana >= (53 - 101) && self.boss_hp > 0 && self.hp > 0
    }

    fn boss_turn(&mut self) {
        self.apply_effects();
        if self.shield > 0 {
            self.hp -= self.boss_damage as isize - 7
        } else {
            self.hp -= self.boss_damage as isize
        };
    }

    fn player_turn(&mut self, spell: &Spell) {
        if self.is_hard {
            self.hp -= 1;
            if self.hp <= 0 {
                return;
            }
        }
        self.apply_effects();
        self.apply_spell(spell);
        if !self.is_over() {
            self.boss_turn();
        }
    }

    fn play(&mut self, spell: &Spell) -> bool {
        self.player_turn(spell);
        self.is_won()
    }
}

impl Spell {
    fn new(cost: usize, effect: Effect) -> Self {
        Spell { cost, effect }
    }
}

pub fn main() {
    let test = GameState::new(10, 250, 13, 8, false);
    let test1 = min_mana(test);
    assert_eq!(test1, 226);
    println!("Day 22 Test 1: {}", test1);

    let input = GameState::new(50, 500, 58, 9, false);
    let part1 = min_mana(input);
    assert_eq!(part1, 1269);
    println!("Day 22 Part 1: {}", part1);

    let input2 = GameState::new(50, 500, 58, 9, true);
    let part2 = min_mana(input2);
    assert_eq!(part2, 1309);
    println!("Day 22 Part 2: {}", part2);
}

fn min_mana(start: GameState) -> usize {
    let mut min_mana = usize::max_value();
    let spells = vec![
        Spell::new(53, Effect::MagicMissile),
        Spell::new(73, Effect::Drain),
        Spell::new(113, Effect::Shield),
        Spell::new(173, Effect::Poison),
        Spell::new(229, Effect::Recharge),
    ];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(game) = queue.pop_front() {
        for spell in &spells {
            let max_mana = if game.recharge > 0 {
                game.mana + 101
            } else {
                game.mana
            };
            if spell.cost > max_mana as usize {
                continue;
            }
            match spell.effect {
                Effect::Shield => {
                    if game.shield > 1 {
                        continue;
                    }
                }
                Effect::Poison => {
                    if game.poison > 1 {
                        continue;
                    }
                }
                Effect::Recharge => {
                    if game.recharge > 1 {
                        continue;
                    }
                }
                _ => (),
            }
            let mut game = game.clone();
            if game.play(spell) {
                min_mana = min_mana.min(game.mana_spent);
            } else if game.is_valid() && game.mana_spent < min_mana {
                queue.push_back(game);
            }
        }
    }
    min_mana
}
