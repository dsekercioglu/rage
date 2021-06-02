use crate::{TankData, Projectile};
use crate::r_core::math::vec2::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct BotData {
    pub pos: Vec2,
    pub r: f32,
    pub can_fire: bool,
}

impl BotData {
    pub fn from_tank_data(data: &TankData) -> Self {
        BotData {
            pos: Vec2::new(data.x.parse::<f32>().unwrap(), data.y.parse::<f32>().unwrap()),
            r: data.r.parse::<f32>().unwrap() * std::f32::consts::PI / 180f32,
            can_fire: data.can_fire,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bullet {
    pos: Vec2,
    vel: Vec2,
}

impl Bullet {
    pub fn from_projectile(projectile: &Projectile) -> Self {
        Self::new(
            projectile.x.parse::<f32>().unwrap(),
            projectile.y.parse::<f32>().unwrap(),
            projectile.vx as f32,
            projectile.vy as f32,
        )
    }

    pub fn new(x: f32, y: f32, vx: f32, vy: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(vx, vy),
        }
    }

    pub fn new_v(pos: Vec2, vel: Vec2) -> Self {
        Self {
            pos,
            vel,
        }
    }

    pub fn estimated_pos(&self, time: f32) -> Vec2 {
        self.pos + self.vel * time
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn vel(&self) -> Vec2 {
        self.vel
    }
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    pub bot: BotData,
    pub opp: BotData,
    pub dist: f32,
    pub abs_bearing: f32,
}

impl State {
    pub fn new(bot: &TankData, opp: &TankData) -> Self {
        let bot = BotData::from_tank_data(bot);
        let opp = BotData::from_tank_data(opp);
        let dist = (bot.pos - opp.pos).sq_magnitude().sqrt();
        let abs_bearing = (opp.pos - bot.pos).angle();
        Self {
            bot,
            opp,
            dist,
            abs_bearing,
        }
    }
}