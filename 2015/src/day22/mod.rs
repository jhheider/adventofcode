use std::cmp::min;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum Spell {
  MagicMissile, Drain, Shield, Poison, Recharge
}

impl Spell {
  fn cost(&self) -> i32 {
    match *self {
      Spell::MagicMissile => 53,
      Spell::Drain => 73,
      Spell::Shield => 113,
      Spell::Poison => 173,
      Spell::Recharge => 239,
    }
  }
}

struct GameState {
  pc_hp: i32,
  pc_mana: i32,
  mana_spent: i32,
  boss_hp: i32,
  boss_dmg: i32,
  shield: i32,
  poison: i32,
  recharge: i32,
  status: BattleState
}

#[derive(PartialEq)]
enum BattleState {
  won, lost, ongoing
}

impl GameState {
  fn start_turn(&mut self) {
    if self.poison > 0 {
      self.boss_hp -= 3;
      self.poison -= 1;
      if self.boss_hp <= 0 { self.status = BattleState::lost }
    }
    if self.recharge > 0 {
      self.pc_mana += 101;
      self.recharge -= 1;
    }
  }

  fn pc_turn(&mut self, spell: Spell) {
    self.start_turn();

    if spell.cost() > self.pc_mana {
      self.status = BattleState::lost;
      self.end_turn();
      return
    }

    self.mana_spent += spell.cost();
    self.pc_mana -= spell.cost();

    match spell {
      Spell::MagicMissile => self.boss_hp -= 4,
      Spell::Drain => {
        self.pc_hp += 2;
        self.boss_hp -= 2;
      },
      Spell::Shield => {
        if self.shield > 0 { self.status = BattleState::lost }
        self.shield = 7;
      },
      Spell::Poison => {
        if self.poison > 0 { self.status = BattleState::lost }
        self.poison = 6;
      },
      Spell::Recharge => {
        if self.recharge > 0 { self.status = BattleState::lost }
        self.recharge = 5;
      },
    }

    self.end_turn();
  }

  fn boss_turn(&mut self) {
    self.start_turn();
    if self.shield > 0 {
      self.pc_hp -= min(1, self.boss_dmg - 7);
    } else {
      self.pc_hp -= self.boss_dmg;
    }
    self.end_turn();
  }

  fn end_turn(&mut self) {
    if self.shield > 0 { self.shield -= 1 }
    if self.status == BattleState::ongoing {
      if self.pc_hp <= 0 { self.status = BattleState::lost }
      if self.boss_hp <= 0 { self.status = BattleState::won }
    }
  }
}

pub fn main() {
  let test = GameState {
    pc_hp: 10,
    pc_mana: 250,
    mana_spent: 0,
    boss_hp: 13,
    boss_dmg: 8,
    shield: 0,
    poison: 0,
    recharge: 0,
    status: BattleState::ongoing,
  };

  let test1 = play_game(&test);
  assert_eq!(test1, (4, 226));
  println!("Day 22: Test 1: PC wins after {} rounds using {} mana", test1.0, test1.1);
}

fn play_game(initial_state: &GameState) -> (i32, i32) {
  let mut state = initial_state.clone();
  let best_win = i32::MAX;
  for round in 1.. {
    match round % 2 {
      1 => {
        for games 
      }
    }

  }

  (0, 0)
}