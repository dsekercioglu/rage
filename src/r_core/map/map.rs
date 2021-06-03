use crate::r_core::math::vec2::Vec2;
use crate::r_core::state::Bullet;
use crate::r_core::map::map::Shape::{Rect, Circle, RotRect};
use std::cmp::Ordering;
use std::ops::Range;
use std::sync::Arc;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone)]
pub struct Map {
    pub shapes: Arc<Vec<Shape>>,
}

const BULLET_VEL: f32 = 400f32;
const MAX_DIST: f32 = BULLET_VEL * 6f32;

#[derive(Debug, Clone)]
pub struct BulletTrajectory {
    bounces: Vec<Vec2>,
    times: Vec<f32>,
}

impl BulletTrajectory {
    pub fn new(bounces: Vec<Vec2>) -> Self {
        let mut times = vec![0f32];
        let prev = bounces[0];
        let mut dist_so_far = 0f32;
        for bounce in bounces.iter().skip(1) {
            let extra = (prev - *bounce).sq_magnitude().sqrt() / BULLET_VEL;
            times.push(dist_so_far + extra);
            dist_so_far += extra;
        }
        Self {
            bounces,
            times,
        }
    }

    pub fn position_in(&self, time: f32) -> Option<Vec2> {
        let index = self.times
            .binary_search_by(|value| value
                .partial_cmp(&time)
                .unwrap_or(Ordering::Equal))
            .unwrap_or_else(|x| x);
        if index == 0 {
            Some(self.bounces[0])
        } else if index < self.times.len() - 1 {
            let way_through = (time - self.times[index - 1]) / (self.times[index - 1] - self.times[index]);
            Some(self.bounces[index - 1] + (self.bounces[index] - self.bounces[index - 1]) * way_through)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Shape {
    Rect {
        pos: Vec2,
        w_h: Vec2,
    },
    RotRect {
        pos: Vec2,
        w_h: Vec2,
        rot: f32,
    },
    Circle {
        pos: Vec2,
        r: f32,
    },
}

impl Shape {
    pub fn container_side(x: f32, y: f32) -> Shape {
        Rect {
            pos: Vec2::new(x, y),
            w_h: Vec2::new(127.274, 62.9048),
        }
    }

    pub fn rotate_90(&mut self) {
        match self {
            Shape::Rect { w_h, .. } => {
                *w_h = Vec2::new(w_h.y(), w_h.x())
            }
            Shape::RotRect { rot, .. } => {
                *rot += std::f32::consts::PI * 0.5f32;
            }
            Shape::Circle { .. } => {}
        }
    }

    pub fn intersects(&self, mut test: Vec2) -> bool {
        match &self {
            Shape::Rect { pos, w_h } => {
                let corner_pos = *pos - *w_h;
                if test.x() > corner_pos.x() && test.y() > corner_pos.y() {
                    test -= *pos;
                    test.x() < w_h.x() && test.y() < w_h.y()
                } else {
                    false
                }
            }
            Shape::RotRect { pos, w_h, rot } => {
                test -= *pos;
                test.rotate(Vec2::from_angle(-*rot));
                test += *pos;
                Rect {
                    pos: *pos,
                    w_h: *w_h,
                }.intersects(test)
            }
            Shape::Circle { pos, r } => {
                (test - *pos).sq_magnitude() < r * r
            }
        }
    }


    //TODO: fix w_h
    pub fn surface_angle(&self, mut test: Vec2) -> f32 {
        match &self {
            Shape::Rect { pos, .. } => {
                let diff = (test - *pos);
                if diff.y().abs() < diff.x().abs() {
                    if diff.x() > 0f32 {
                        0f32
                    } else {
                        std::f32::consts::PI
                    }
                } else {
                    if diff.y() > 0f32 {
                        -std::f32::consts::PI * 0.5f32
                    } else {
                        std::f32::consts::PI * 0.5f32
                    }
                }
            }
            Shape::RotRect { pos, w_h, rot } => {
                let center = *pos;
                test -= center;
                test.rotate(Vec2::from_angle(-*rot));
                test += center;
                Rect {
                    pos: *pos,
                    w_h: *w_h,
                }.surface_angle(test) + *rot
            }
            Shape::Circle { pos, .. } => {
                (test - *pos).angle()
            }
        }
    }
}

const CONTAINER_SIDE: Vec2 = Vec2::new(63.5487 * 2.05, 31.6329 * 2.05);
const CONTAINER_AREA: Vec2 = Vec2::new(63.5487 * 2.05, 31.6329 * 2.05);
const CAR_A_B: Vec2 = Vec2::new(36.046 * 2.05, 14.837 * 2.05);
const CAR_C: Vec2 = Vec2::new(33.878 * 2.05, 13.789 * 2.05);
const BARREL: f32 = 61.0818 * 0.605;

const SIDE_POS_A: Vec2 = Vec2::new(128.915, 106.215);

impl Map {
    pub fn new_map_a() -> Self {
        let shapes = vec![Rect {
            pos: SIDE_POS_A + Vec2::new(56.332f32, 613.317f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(-70.1838f32, 613.733f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(183.332f32, 613.317f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(309.902f32, 613.317f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(436.902f32, 613.317f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(564.547f32, 613.317f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(691.547f32, 613.317f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(818.117f32, 613.317f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(945.117f32, 613.317f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(1072.41f32, 613.317f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(127.629f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(1.1134f32, -13.6822f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(-123.657f32, -13.6822f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(254.629f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(381.199f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(508.199f32, -14.0987f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(635.844f32, -14.0987f32),
            w_h: CONTAINER_SIDE,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(762.844f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(889.414f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(1016.41f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, Rect {
            pos: SIDE_POS_A + Vec2::new(1143.71f32, -14.0987f32),
            w_h: CONTAINER_SIDE,

        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(1149.52f32, 652.298f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(1149.58f32, 525.707f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(1149.97f32, 398.634f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(1150.03f32, 272.042f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(1150.03f32, 145.474f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(1150.03f32, 17.8873f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(-128.915f32, 652.126f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(-128.915f32, 525.054f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(-128.915f32, 398.365f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(-128.915f32, 271.774f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(-128.915f32, 145.205f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(-128.915f32, 18.6273f32),
            w_h: CONTAINER_SIDE,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(574.701, 16.4741),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(574.701, 142.725),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_2,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(515.085, 356.985),
            w_h: CONTAINER_AREA,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(801.926, 358.89),
            w_h: CONTAINER_AREA,
        }, Rect {
            pos: SIDE_POS_A +Vec2::new(-41.4074, 180.834),
            w_h: CONTAINER_AREA,
        }, Rect {
            pos: SIDE_POS_A+Vec2::new(79.9876, 180.834),
            w_h: CONTAINER_AREA,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(883.58, 397.678),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_4,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(291.896, 216.316),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_4,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(765.132, 80.4514),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_2,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(206.759, 180.834),
            w_h: CONTAINER_AREA,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(451.085, 420.985),
            w_h: CONTAINER_AREA,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(676.88, 551.856),
            w_h: CONTAINER_AREA,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(79.9876, 520.558),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_2,
        }, RotRect {
            pos: SIDE_POS_A + Vec2::new(79.9876, 396.447),
            w_h: CONTAINER_AREA,
            rot: std::f32::consts::FRAC_PI_2,
        }, Rect {
            pos: SIDE_POS_A + Vec2::new(1053.98, 244.834),
            w_h: CONTAINER_AREA,
        }];
        std::fs::write(
            "./maps/MapA.json",
            serde_json::to_string(&shapes).unwrap(),
        );
        Map {
            shapes: Arc::new(serde_json::from_str(&std::fs::read_to_string("./maps/MapA.json").unwrap()).unwrap())
        }
    }

    pub fn new_map_b() -> Self {
        let shapes = vec![
            RotRect {
                pos: SIDE_POS_A + Vec2::new(-123.124, 500.264),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-123.298, 350.581),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-123.139, 574.906),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-122.327, 279.376),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-122.863, 133.498),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-123.037, -16.1845),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-122.878, 207.14),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(221.216, -37.7832),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(72.5438, -36.5491),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(-71.9961, -37.3872),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(1.47256, -37.4734),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(366.017, 606.158),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(221.536, 607.221),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(295.005, 607.135),
                w_h: CAR_A_B,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(365.547, -36.5491),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(294.475, -37.4734),
                w_h: CAR_A_B,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(541.83, 262.791),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(468.759, 263.867),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(660.237, -36.5491),
                w_h: CAR_C,
                rot: 0.0,//???????
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(960.297, -37.4752),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1029.76, -37.5139),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(515.697, -37.3872),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(589.166, -37.4734),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(887.475, -37.7296),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(1101.24, -36.8291),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(-74.3384, 605.724),
                w_h: CAR_C,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-122.328, 425.732),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1153.46, 141.973),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1153.44, 216.614),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1152.82, 67.4401),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1153.16, 508.469),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1152.13, 437.263),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1152.78, -5.37441),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1153.59, 291.386),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(549.795, 112.382),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(587.17, 111.902),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1153.58, 365.027),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1152.13, 583.619),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-122.067, 57.9661),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(788.54, 550.786),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(788.366, 401.103),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(914.133, 379.253),
                w_h: CAR_C,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(839.407, 379.9),
                w_h: CAR_A_B,
                rot: 0.0,//?????
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(841.576, 415.463),
                w_h: CAR_A_B,
                rot: 0.0, //????
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(789.336, 475.253),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(477.797, 111.38),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(512.045, 114.288),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(407.7, 113.812),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(445.075, 113.332),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(335.702, 112.81),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(369.95, 115.718),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(262.967, 115.152),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(300.342, 114.672),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(117.53, 116.191),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(154.905, 115.711),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(81.1483, 115.711),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(190.969, 114.15),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(225.217, 117.058),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(145.766, -36.875),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(439.405, -36.3901),
                w_h: CAR_C,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(515.24, 607.276),
                w_h: CAR_A_B,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(660.551, 605.856),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(589.153, 606.893),
                w_h: CAR_A_B,
                rot: std::f32::consts::PI,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(1027.25, 606.949),
                w_h: CAR_A_B,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1101.16, 606.566),
                w_h: CAR_A_B,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(593.317, 284.532),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(592.287, 359.682),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(100.185, 171.345),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(151.672, 193.085),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(150.642, 268.236),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(58.0687, 473.896),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(57.0387, 549.046),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(437.556, 383.247),
                w_h: CAR_A_B,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(541.318, 382.629),
                w_h: CAR_A_B,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(364.473, 383.24),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(290.708, 384.662),
                w_h: CAR_A_B,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(520.703, 435.701),
                w_h: CAR_A_B,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(489.842, 406.583),
                w_h: CAR_C,
                rot: std::f32::consts::FRAC_PI_2,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(289.7, 456.787),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(736.149, 469.113),
                w_h: CAR_A_B,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(289.821, 420.739),
                w_h: CAR_A_B,
                rot: std::f32::consts::PI,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(955.405, 605.856),
                w_h: CAR_C,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(809.884, 607.143),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(883.68, 607.056),
                w_h: CAR_A_B,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(439.953, 606.347),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(733.756, 606.178),
                w_h: CAR_C,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(733.459, -36.875),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(811.183, -36.7325),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(878.177, 36.8203),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(877.542, 2.17767),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(880.392, 106.592),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(879.753, 72.2837),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(882.292, 177.298),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(881.899, 141.559),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(148.358, 606.136),
                w_h: CAR_C,
                rot: std::f32::consts::PI,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(75.5358, 606.882),
                w_h: CAR_A_B,
            }, Rect {
                pos: SIDE_POS_A + Vec2::new(-0.755966, 605.879),
                w_h: CAR_C,
            }
        ];


        std::fs::write("./maps/MapB.json", serde_json::to_string(&shapes).unwrap());
        Map {
            shapes: Arc::new(serde_json::from_str(&std::fs::read_to_string("./maps/MapB.json").unwrap()).unwrap()),
        }
    }

    pub fn new_map_c() -> Self {
        let shapes = vec![
            Rect {
                pos: SIDE_POS_A + Vec2::new(56.332, 613.317),
                w_h: CONTAINER_SIDE,
            },
            Rect {
                pos: SIDE_POS_A + Vec2::new(-70.1838, 613.733),
                w_h: CONTAINER_SIDE,
            },
            Rect {
                pos: SIDE_POS_A + Vec2::new(183.332, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(309.902, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(436.902, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(564.547, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(691.547, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(818.117, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(945.117, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(1072.41, 613.317),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(127.629, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(1.1134, -13.6822),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(-123.657, -13.6822),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(254.629, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(381.199, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(508.199, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(635.844, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(762.844, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(889.414, -14.0987),
                w_h: CONTAINER_SIDE,

            }, Rect {
                pos: SIDE_POS_A + Vec2::new(1016.41, -14.0987),
                w_h: CONTAINER_SIDE,

            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1143.71, -14.0987),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1149.52, 652.298),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1149.58, 525.707),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1150.03, 272.042),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1150.03, 145.474),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(1150.03, 17.8873),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-128.915, 652.126),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-128.915, 525.054),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-128.915, 398.365),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-128.915, 271.774),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-128.915, 145.205),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, RotRect {
                pos: SIDE_POS_A + Vec2::new(-128.915, 18.6273),
                w_h: CONTAINER_SIDE,
                rot: std::f32::consts::FRAC_PI_2,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(238.262, 38.2629),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(238.262, 194.94),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(417.426, 108.972),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(417.426, 279.644),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(71.308, 108.972),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(417.426, 460.431),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(71.308, 279.644),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(71.308, 460.431),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(238.262, 374.14),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(238.262, 558.237),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(610.379, 38.2629),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(610.379, 194.94),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(792.794, 108.972),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(792.794, 279.644),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(792.794, 460.431),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(610.379, 374.14),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(610.379, 558.237),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(992.247, 194.94),
                r: BARREL,
            }, Circle {
                pos: SIDE_POS_A + Vec2::new(992.247, 374.14),
                r: BARREL,
            }];

        std::fs::write("./maps/MapC.json", serde_json::to_string(&shapes).unwrap());
        Map {
            shapes: Arc::new(serde_json::from_str(&std::fs::read_to_string("./maps/MapC.json").unwrap()).unwrap()),
        }
    }

    pub fn map_from_file(path: &str) -> Self {
        Map {
            shapes: Arc::new(serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()),
        }
    }

    pub fn intersects(&self, point: Vec2) -> bool {
        for shape in self.shapes.iter() {
            if shape.intersects(point) {
                return true;
            }
        }
        false
    }

    pub fn ray_cast(&self, source: Vec2, angles: &[f32], step_size: f32, steps: Range<usize>) -> Vec<(Option<Shape>, Vec2, usize)> {
        let thread_cnt = angles.len().min(4);
        Self::para_cast(source, angles, step_size, steps, self.shapes.clone(), thread_cnt)
    }

    //TODO: single calculation step
    pub fn single_cast(source: Vec2, angle: f32, step_size: f32, steps: Range<usize>, shapes: Arc<Vec<Shape>>) -> (Option<Shape>, Vec2, usize) {
        let step = Vec2::from_angle(angle) * step_size;
        let end = steps.end;

        let mut test = source;
        for s in steps {
            test = source + step * (s as f32);
            for shape in shapes.iter() {
                if shape.intersects(test) {
                    return (Some(*shape), test, s);
                }
            }
        }
        (None, test, end)
    }

    pub fn get_bounce(pos: Vec2, angle: f32, shape: Shape) -> f32 {
        let s_angle = shape.surface_angle(pos);
        let mut angle = angle + std::f32::consts::PI;
        2f32 * s_angle - angle
    }

    fn para_cast(source: Vec2, angles: &[f32], step_size: f32, steps: Range<usize>, shapes: Arc<Vec<Shape>>, threads: usize) -> Vec<(Option<Shape>, Vec2, usize)> {
        let mut out = vec![];
        let chunks = angles.len() / threads + 1;
        let mut handles = vec![];
        for angle in angles.chunks(chunks) {
            let angles = angle.to_vec();
            let steps_clone = steps.clone();
            let shapes_clone = shapes.clone();
            handles.push(std::thread::spawn(move || {
                let mut dists = vec![];
                for angle in angles {
                    dists.push(
                        Self::single_cast(
                            source,
                            angle,
                            step_size,
                            steps_clone.clone(),
                            shapes_clone.clone(),
                        )
                    )
                }
                dists
            }
            ));
        }
        for handle in handles {
            out.extend(handle.join().unwrap());
        }
        out
    }

    pub fn get_bullet_trajectory(&self, bullet: Bullet, step_size: f32, max_bounces: usize) -> BulletTrajectory {
        let mut source = bullet.pos();
        let mut angle = bullet.vel().angle();
        let mut bounces = vec![source];

        let mut step_cnt = (MAX_DIST / step_size).ceil() as usize;

        for _ in 0..max_bounces {
            let (shape, pos, steps) = Self::single_cast(source, angle, step_size, 10..step_cnt, self.shapes.clone());
            if steps >= step_cnt {
                break;
            }
            step_cnt -= steps;
            bounces.push(pos);
            if let Some(shape) = shape {
                source = pos;
                angle = Self::get_bounce(pos, angle, shape);
            } else {
                return BulletTrajectory::new(bounces);
            }
        }
        BulletTrajectory::new(bounces)
    }
}