use std::env;
use std::ops::{Add, Mul};
use image::Rgba;
use nalgebra::{Vector2, Vector3, SMatrix, Matrix, Const, ArrayStorage, Vector4};
use once_cell::sync::Lazy;

pub type V4t<T> = Vector4<T>;
pub type V4i = Vector4<i32>;
pub type V4u = Vector4<u32>;
pub type V4f = Vector4<f32>;
pub type V4d = Vector4<f64>;

pub type V3t<T> = Vector3<T>;
pub type V3i = Vector3<i32>;
pub type V3u = Vector3<u32>;
pub type V3f = Vector3<f32>;
pub type V3d = Vector3<f64>;

pub type V2t<T> = Vector2<T>;
pub type V2i = Vector2<i32>;
pub type V2u = Vector2<u32>;
pub type V2f = Vector2<f32>;
pub type V2d = Vector2<f64>;

pub type Mx2t<T> = SMatrix<T, 2, 2>;
pub type Mx3t<T> = SMatrix<T, 3, 3>;
pub type Mx4t<T> = SMatrix<T, 4, 4>;
pub type Mx5t<T> = SMatrix<T, 5, 5>;

pub type Mx2d = SMatrix<f64, 2, 2>;
pub type Mx3d = SMatrix<f64, 3, 3>;
pub type Mx4d = SMatrix<f64, 4, 4>;
pub type Mx5d = SMatrix<f64, 5, 5>;

pub type Mx2i = SMatrix<i32, 2, 2>;
pub type Mx3i = SMatrix<i32, 3, 3>;
pub type Mx4i = SMatrix<i32, 4, 4>;
pub type Mx5i = SMatrix<i32, 5, 5>;

pub static CWD: Lazy<String> = Lazy::new(|| env::current_dir().unwrap().into_os_string().into_string().unwrap());

pub const WID: u32 = 1000;
pub const HIG: u32 = 1000;
pub const DEP: u32 = 255;

pub const BLACK: Rgba<u8> = Rgba([0, 0, 0, 0]);
pub const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
pub const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
pub const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
pub const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);

pub fn as_v3d(v4d: V4d) -> V3d {
    V3d::new(v4d.x, v4d.y, v4d.z)
}