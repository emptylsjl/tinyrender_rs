
use std::default::Default;
use std::iter::Product;
use std::ops::{Add, Div, Mul, Sub};

use num_traits::{real::Real, Pow};

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

// trait Num: Add + Add<Output=Self> + Sub + Sub<Output=Self> + Mul + Mul<Output=Self> + Div + Div<Output=Self> + Copy {}

impl<T: Real + Copy> Add<V3t<T>> for V3t<T> {
    type Output = V3t<T>;

    fn add(self, rhs: V3t<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<T: Real + Copy> Sub<V3t<T>> for V3t<T> {
    type Output = V3t<T>;

    fn sub(self, rhs: V3t<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<T: Real + Copy> V3t<T> {

    pub fn new(x: T, y: T, z: T) -> V3t<T> {
        V3t{ x, y, z }
    }

    pub fn cross(&self, rhs: V3t<T>) -> V3t<T> {
        V3t {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn dot(&self, rhs: V3t<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn norm(&self) -> T {
        let a = self.x*self.x + self.y*self.y + self.z*self.z;
        a.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        V3t { x: self.x / n, y: self.y / n, z: self.z / n}
    }

    pub fn scale(&self, t: T) -> Self {
        V3t { x: self.x * t, y: self.y * t, z: self.z * t}
    }
}

impl<T: Real + Copy> Add<V4t<T>> for V4t<T> {
    type Output = V4t<T>;
    fn add(self, rhs: V4t<T>) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z, w: self.w + rhs.w }
    }
}

impl<T: Real + Copy> Sub<V4t<T>> for V4t<T> {
    type Output = V4t<T>;
    fn sub(self, rhs: V4t<T>) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z, w: self.w - rhs.w }
    }
}

impl<T: Real + Copy> V4t<T> {

    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn dot(&self, rhs: V4t<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z + self.w + rhs.w
    }

    pub fn v3t(&self) -> V3t<T> {
        V3t { x: self.x, y: self.y, z: self.z}
    }

    pub fn norm(&self) -> T {
        let a = self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w;
        a.sqrt()
    }

    pub fn normalize(&self) -> Self {
        let n = self.norm();
        V4t { x: self.x / n, y: self.y / n, z: self.z / n, w: self.w / n}
    }

    pub fn scale(&self, t: T) -> Self {
        V4t { x: self.x * t, y: self.y * t, z: self.z * t, w: self.w * t}
    }
}

impl<T: Real + Copy> Add<Mx4t<T>> for Mx4t<T> {
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

impl<T: Real + Copy> Sub<Mx4t<T>> for Mx4t<T> {
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

impl<T: Real + Copy> Mul<Mx4t<T>> for Mx4t<T> {
    type Output = Mx4t<T>;
    fn mul(self, rhs: Mx4t<T>) -> Self::Output {
        Mx4t {
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

impl<T: Real + Copy> Mul<&V4t<T>> for Mx4t<T> {
    type Output = V4t<T>;
    fn mul(self, rhs: &V4t<T>) -> Self::Output {
        V4t {
            x: self.m11*rhs.x + self.m12*rhs.y + self.m13*rhs.z + self.m14*rhs.w,
            y: self.m21*rhs.x + self.m22*rhs.y + self.m23*rhs.z + self.m24*rhs.w,
            z: self.m31*rhs.x + self.m32*rhs.y + self.m33*rhs.z + self.m34*rhs.w,
            w: self.m41*rhs.x + self.m42*rhs.y + self.m43*rhs.z + self.m44*rhs.w,
        }
    }
}

impl<T: Real + Copy> Mx4t<T> {
    pub fn new(m11: T, m12: T, m13: T, m14: T, m21: T, m22: T, m23: T, m24: T, m31: T, m32: T, m33: T, m34: T, m41: T, m42: T, m43: T, m44: T) -> Mx4t<T> {
        Mx4t {
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        }
    }

    pub fn identity() -> Mx4d {
        Mx4d { m11: 1., m22: 1., m33: 1., m44: 1., ..Default::default() }
    }

    pub fn scale([x, y, z]: [f64;3]) -> Mx4d {
        Mx4d { m11: x, m22: y, m33: z, m44: 1., ..Default::default() }
    }

    pub fn proj([x, y, z]: [f64;3]) -> Mx4d {
        Mx4d { m11: 1., m22: 1., m33: 1., m44: 1., m43: 1./z, ..Default::default() }
    }

    pub fn rot([x, y, z]: [f64;3]) -> Mx4d {
        match ((x!=0.), (y!=0.), (z!=0.)) {
            (true, _, _) => { Mx4d { m11: x.cos(), m12: -x.sin(), m21: x.sin(), m22: x.cos(), m33: 1., m44: 1., ..Default::default() } }
            (_, true, _) => { Mx4d { m11: y.cos(), m13: -y.sin(), m31: y.sin(), m33: y.cos(), m22: 1., m44: 1., ..Default::default() } }
            (_, _, true) => { Mx4d { m22: z.cos(), m23: -z.sin(), m32: z.sin(), m33: z.cos(), m11: 1., m44: 1., ..Default::default() } }
            _ =>     {panic!("meow")}
        }
    }

    pub fn trans([x, y, z]: [f64;3]) -> Mx4d {
        Mx4d {
            m11: 1., m22: 1., m33: 1., m44: 1.,
            m14: x, m24: y, m34: z,
            ..Default::default()
        }
    }
}