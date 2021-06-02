mod r_core;

use reqwest;
use std::io::Read;
use std::collections::HashMap;
use reqwest::Url;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde::{Serialize, Deserialize};
use crate::r_core::predictor::knn::LinearSearcher;
use crate::r_core::controller::controller::Controller;
use crate::r_core::env::{Env, Team};
use std::sync::{Arc, Mutex};
use r_core::map::map::Map;
use crate::r_core::map::map::Shape::{Rect, RotRect, Circle};
use crate::r_core::math::vec2::Vec2;
use crate::r_core::state::Bullet;
use std::time::Instant;
use text_io::read;

const GET_URL: &str = "http://localhost:3000";
const POST_URL_A: &str = "http://localhost:4000";
const POST_URL_B: &str = "http://localhost:5000";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Projectile {
    x: String,
    y: String,
    vx: i32,
    vy: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TankData {
    pub x: String,
    pub y: String,
    pub r: String,
    pub can_fire: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    tankA: TankData,
    tankB: TankData,
    projectiles: HashMap<String, Projectile>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut team = None;
    while team.is_none() {
        println!("Team(A/B): ");
        let input: String = read!("{}\n");
        if input == "A" {
            team = Some(Team::A);
        } else if input == "B" {
            team = Some(Team::B);
        }
    }
    let team = team.unwrap();
    let post_url = match team {
        Team::A => {
            POST_URL_A
        }
        Team::B => {
            POST_URL_B
        }
    };
    println!("Map: ");
    let mut map = None;
    while map.is_none() {
        println!("Map(A/B/C): ");
        let input: String = read!("{}\n");
        if input == "A" {
            map = Some("./maps/MapA.json");
        } else if input == "B" {
            map = Some("./maps/MapB.json");
        } else if input == "C" {
            map = Some("./maps/MapC.json");
        }
    };

    let client = reqwest::blocking::Client::new();
    let environment = Arc::new(Mutex::new(Env::new(10)));
    //
    let map = Arc::new(Map::map_from_file(map.unwrap()));
    let mut controller = Controller::new(environment.clone(), map.clone(), team);

    let time = Instant::now();
    println!("map alloc time: {:?}", time.elapsed());


    loop {
        let mut response = client.get(GET_URL).send()?;
        let game_data = response.json::<GameData>()?;

        let (bot, opp) = match team {
            Team::A => {
                (&game_data.tankA, &game_data.tankB)
            }
            Team::B => {
                (&game_data.tankB, &game_data.tankA)
            }
        };
        environment.lock().unwrap().update(
            &bot,
            &opp,
            game_data.projectiles,
        );
        let out_data = controller.out_data();
        controller.update();
        client.post(post_url).json(
            &out_data
        ).send()?;
    }
}