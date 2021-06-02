use packed_simd::f32x2;
use std::ops::{Add, AddAssign, MulAssign, Mul, Sub, SubAssign, Div, DivAssign};
use serde::{Serialize, Deserialize, Serializer, de, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{Visitor, SeqAccess};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pos: f32x2,
}

impl Serialize for Vec2 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.x())?;
        seq.serialize_element(&self.y())?;
        seq.end()
    }
}

struct Vec2Deserializer;

impl<'de> Visitor<'de> for Vec2Deserializer {
    type Value = Vec2;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(":pensive:")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
    {
        let x = seq.next_element::<f32>()?.unwrap();
        let y = seq.next_element::<f32>()?.unwrap();
        let vec2 = Vec2::new(x, y);
        Ok(vec2)
    }
}

impl<'de> Deserialize<'de> for Vec2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(Vec2Deserializer)
    }
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            pos: f32x2::new(x, y),
        }
    }

    pub fn x(&self) -> f32 {
        unsafe { self.pos.extract_unchecked(0) }
    }

    pub fn y(&self) -> f32 {
        unsafe { self.pos.extract_unchecked(1) }
    }

    pub fn angle(&self) -> f32 {
        f32::atan2(self.y(), self.x())
    }


    pub const fn from_pack(pos: f32x2) -> Self {
        Self {
            pos,
        }
    }

    pub fn from_angle(angle: f32) -> Self {
        //let angle = angle + std::f32::consts::FRAC_PI_2;
        let sin_cos = angle.sin_cos();
        Self {
            pos: f32x2::new(sin_cos.1, sin_cos.0)
        }
    }

    pub fn sq_magnitude(&self) -> f32 {
        (*self * *self).sum()
    }

    pub fn normalize(&mut self) {
        let sum = self.sq_magnitude().sqrt();
        *self /= sum
    }

    pub fn dot(&mut self, other: Vec2) -> f32 {
        (*self * other).sum()
    }

    pub fn rotate(&mut self, rotation: Vec2) {
        self.pos = f32x2::new(self.x() * rotation.x() - self.y() * rotation.y(),
                              self.x() * rotation.y() + self.y() * rotation.x());
    }

    pub fn sum(&mut self) -> f32 {
        self.pos.sum()
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2::from_pack(self.pos + rhs.pos)
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.pos += rhs.pos;
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2::from_pack(self.pos - rhs.pos)
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.pos -= rhs.pos;
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::from_pack(self.pos * rhs.pos)
    }
}

impl MulAssign<Vec2> for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        self.pos *= rhs.pos;
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2::from_pack(self.pos / rhs.pos)
    }
}

impl DivAssign<Vec2> for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        self.pos /= rhs.pos;
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: f32) -> Self::Output {
        Vec2::from_pack(self.pos + rhs)
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, rhs: f32) {
        self.pos += rhs;
    }
}

impl Sub<f32> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec2::from_pack(self.pos - rhs)
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.pos -= rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2::from_pack(self.pos * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.pos *= rhs;
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2::from_pack(self.pos / rhs)
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.pos /= rhs;
    }
}