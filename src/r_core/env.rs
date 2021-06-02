use std::collections::{VecDeque, HashMap};
use crate::r_core::state::{State, BotData, Bullet};
use crate::{TankData, Projectile};
use std::time::Instant;

#[derive(Debug, Copy, Clone)]
pub enum Team {
    A,
    B,
}

pub struct Env {
    history: VecDeque<State>,
    p0_projectile: Option<Bullet>,
    p1_projectile: Option<Bullet>,
    max_size: usize,
    update_time: Instant,
}

impl Env {
    pub fn new(max_size: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_size),
            p0_projectile: None,
            p1_projectile: None,
            max_size,
            update_time: Instant::now(),
        }
    }

    pub fn update(&mut self,
                  bot: &TankData,
                  opp: &TankData,
                  projectile_data: HashMap<String, Projectile>) {
        self.update_time = Instant::now();
        if self.history.len() > self.max_size {
            self.history.pop_front();
        }
        if let Some(projectile) = projectile_data.get("p0") {
            self.p0_projectile = Some(Bullet::from_projectile(projectile));
        } else {
            self.p0_projectile = None;
        }
        if let Some(projectile) = projectile_data.get("p1") {
            self.p1_projectile = Some(Bullet::from_projectile(projectile));
        } else {
            self.p1_projectile = None;
        }
        self.history.push_back(State::new(bot, opp));
    }

    pub fn get_bot_bullet(&self, team: Team) -> Option<Bullet> {
        match team {
            Team::A => {
                self.p0_projectile
            }
            Team::B => {
                self.p1_projectile
            }
        }
    }

    pub fn get_enemy_bullet(&self, team: Team) -> Option<Bullet> {
        match team {
            Team::A => {
                self.p1_projectile
            }
            Team::B => {
                self.p0_projectile
            }
        }
    }

    pub fn current_state(&self) -> Option<State> {
        self.history.get(self.history.len() - 1).copied()
    }

    pub fn last_update(&self) -> Instant {
        self.update_time
    }
}