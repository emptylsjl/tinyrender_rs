
use std::default::Default;
use std::ops::{Add, Div, Mul, Sub};
use crate::define::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct V4t<T> {
    pub x: T, pub y: T, pub z: T, pub w: T
}

#[derive(Debug, Default)]
pub struct V3t<T> {
    pub x: T, pub y: T, pub z: T
}

#[derive(Debug, Default)]
pub struct Mx3t<T> {
    m11: T, m12: T, m13: T,
    m21: T, m22: T, m23: T,
    m31: T, m32: T, m33: T,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Mx4t<T> {
    m11: T, m12: T, m13: T, m14: T,
    m21: T, m22: T, m23: T, m24: T,
    m31: T, m32: T, m33: T, m34: T,
    m41: T, m42: T, m43: T, m44: T,
}

pub type V3d = V3t<f64>;
pub type V3i = V3t<i32>;

pub type V4d = V4t<f64>;
pub type V4i = V4t<i32>;

pub type Mx4d = Mx4t<f64>;

trait Num: Add + Add<Output=Self> + Sub + Sub<Output=Self> + Mul + Mul<Output=Self> + Div + Div<Output=Self> + Copy {}

impl<T: Num> Add<V3t<T>> for V3t<T> {
    type Output = V3t<T>;

    fn add(self, rhs: V3t<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T: Num> Sub<V3t<T>> for V3t<T> {
    type Output = V3t<T>;

    fn sub(self, rhs: V3t<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T: Num> V3t<T> {
    pub fn cross(self, rhs: V3t<T>) -> V3t<T> {
        V3t {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn dot(self, rhs: V3t<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z
    }
}

impl<T: Num> V4t<T> {
    pub fn dot(self, rhs: V4t<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z + self.w + rhs.w
    }
}

impl<T: Num> Add<V4t<T>> for V4t<T> {
    type Output = V4t<T>;
    fn add(self, rhs: V4t<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl<T: Num> Sub<V4t<T>> for V4t<T> {
    type Output = V4t<T>;
    fn sub(self, rhs: V4t<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl<T: Num> Add<Mx4t<T>> for Mx4t<T> {
    type Output = Mx4t<T>;
    fn add(self, rhs: Mx4t<T>) -> Self::Output {
        Self {
            m11: self.m11 + rhs.m11, m12: self.m12 + rhs.m12, m13: self.m13 + rhs.m13, m14: self.m14 + rhs.m14,
            m21: self.m21 + rhs.m21, m22: self.m22 + rhs.m22, m23: self.m23 + rhs.m23, m24: self.m24 + rhs.m24,
            m31: self.m31 + rhs.m31, m32: self.m32 + rhs.m32, m33: self.m33 + rhs.m33, m34: self.m34 + rhs.m34,
            m41: self.m41 + rhs.m41, m42: self.m42 + rhs.m42, m43: self.m43 + rhs.m43, m44: self.m44 + rhs.m44
        }
    }
}

impl<T: Num> Sub<Mx4t<T>> for Mx4t<T> {
    type Output = Mx4t<T>;
    fn sub(self, rhs: Mx4t<T>) -> Self::Output {
        Self {
            m11: self.m11 - rhs.m11, m12: self.m12 - rhs.m12, m13: self.m13 - rhs.m13, m14: self.m14 - rhs.m14,
            m21: self.m21 - rhs.m21, m22: self.m22 - rhs.m22, m23: self.m23 - rhs.m23, m24: self.m24 - rhs.m24,
            m31: self.m31 - rhs.m31, m32: self.m32 - rhs.m32, m33: self.m33 - rhs.m33, m34: self.m34 - rhs.m34,
            m41: self.m41 - rhs.m41, m42: self.m42 - rhs.m42, m43: self.m43 - rhs.m43, m44: self.m44 - rhs.m44
        }
    }
}

impl Mul<Mx4d> for Mx4d {
    type Output = Mx4d;
    fn mul(self, rhs: Mx4d) -> Self::Output {
        Mx4d {
            m11: self.m11*rhs.m11 + self.m12*rhs.m21 + self.m13*rhs.m31 + self.m14*rhs.m41,
            m12: self.m11*rhs.m12 + self.m12*rhs.m22 + self.m13*rhs.m32 + self.m14*rhs.m42,
            m13: self.m11*rhs.m13 + self.m12*rhs.m23 + self.m13*rhs.m33 + self.m14*rhs.m43,
            m14: self.m11*rhs.m14 + self.m12*rhs.m24 + self.m13*rhs.m34 + self.m14*rhs.m44,

            m21: self.m21*rhs.m11 + self.m22*rhs.m21 + self.m23*rhs.m31 + self.m24*rhs.m41,
            m22: self.m21*rhs.m12 + self.m22*rhs.m22 + self.m23*rhs.m32 + self.m24*rhs.m42,
            m23: self.m21*rhs.m13 + self.m22*rhs.m23 + self.m23*rhs.m33 + self.m24*rhs.m43,
            m24: self.m21*rhs.m14 + self.m22*rhs.m24 + self.m23*rhs.m34 + self.m24*rhs.m44,

            m31: self.m31*rhs.m11 + self.m32*rhs.m21 + self.m33*rhs.m31 + self.m34*rhs.m41,
            m32: self.m31*rhs.m12 + self.m32*rhs.m22 + self.m33*rhs.m32 + self.m34*rhs.m42,
            m33: self.m31*rhs.m13 + self.m32*rhs.m23 + self.m33*rhs.m33 + self.m34*rhs.m43,
            m34: self.m31*rhs.m14 + self.m32*rhs.m24 + self.m33*rhs.m34 + self.m34*rhs.m44,

            m41: self.m41*rhs.m11 + self.m42*rhs.m21 + self.m43*rhs.m31 + self.m44*rhs.m41,
            m42: self.m41*rhs.m12 + self.m42*rhs.m22 + self.m43*rhs.m32 + self.m44*rhs.m42,
            m43: self.m41*rhs.m13 + self.m42*rhs.m23 + self.m43*rhs.m33 + self.m44*rhs.m43,
            m44: self.m41*rhs.m14 + self.m42*rhs.m24 + self.m43*rhs.m34 + self.m44*rhs.m44,
        }
    }
}

impl Mul<&V4d> for Mx4d {
    type Output = V4d;
    fn mul(self, rhs: &V4d) -> Self::Output {
        V4d {
            x: self.m11*rhs.x + self.m12*rhs.y + self.m13*rhs.z + self.m14*rhs.w,
            y: self.m21*rhs.x + self.m22*rhs.y + self.m23*rhs.z + self.m24*rhs.w,
            z: self.m31*rhs.x + self.m32*rhs.y + self.m33*rhs.z + self.m34*rhs.w,
            w: self.m41*rhs.x + self.m42*rhs.y + self.m43*rhs.z + self.m44*rhs.w,
        }
    }
}

impl<T: Num> Mx4t<T> {
    pub fn new(m11: T, m12: T, m13: T, m14: T, m21: T, m22: T, m23: T, m24: T, m31: T, m32: T, m33: T, m34: T, m41: T, m42: T, m43: T, m44: T) -> Mx4t<T> {
        Mx4t {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        }
    }
}

impl Mx4d {
    pub fn identity() -> Mx4d {
        Mx4d { m11: 1., m22: 1., m33: 1., m44: 1., ..Default::default() }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Mx4d {
        Mx4d { m11: x, m22: y, m33: z, m44: 1., ..Default::default() }
    }

    pub fn rot(axis: i32, a: f64) -> Mx4d {
        match axis {
            XAXIS => { Mx4d { m11: a.cos(), m12: -a.sin(), m21: a.sin(), m22: a.cos(), m33: 1., m44: 1., ..Default::default() } }
            YAXIS => { Mx4d { m11: a.cos(), m13: -a.sin(), m31: a.sin(), m33: a.cos(), m22: 1., m44: 1., ..Default::default() } }
            ZAXIS => { Mx4d { m22: a.cos(), m23: -a.sin(), m32: a.sin(), m33: a.cos(), m11: 1., m44: 1., ..Default::default() } }
            _ =>     {panic!("meow")}
        }
    }

    pub fn trans(x: f64, y: f64, z: f64) -> Mx4d {
        Mx4d {
            m11: 1., m22: 1., m33: 1., m44: 1.,
            m14: x, m24: y, m34: z,
            ..Default::default()
        }
    }
}