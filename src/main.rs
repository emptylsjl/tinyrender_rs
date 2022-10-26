#![feature(str_split_as_str)]

mod tga;
mod mobj;
use mobj::Module;

use mem::swap;
use std::error::Error;
use std::{env, mem};
use std::iter::zip;

use rand::Rng;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, Rgb, Rgba, RgbaImage};
use nalgebra::Vector3;
use once_cell::sync::Lazy;

static CWD: Lazy<String> = Lazy::new(|| env::current_dir().unwrap().into_os_string().into_string().unwrap());

const WID: u32 = 1000;
const HIG: u32 = 1000;
const DEP: u32 = 255;

const BLACK: Rgba<u8> = Rgba([0, 0, 0, 0]);
const WHITE: Rgba<u8> = Rgba([255, 255, 255, 255]);
const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);
const GREEN: Rgba<u8> = Rgba([0, 255, 0, 255]);
const BLUE: Rgba<u8> = Rgba([0, 0, 255, 255]);

type V3i = Vector3<i32>;
type V3u = Vector3<u32>;
type V3f = Vector3<f32>;
type V3d = Vector3<f64>;

fn p3d(v: &Vector3<f64>) -> V3d {
    Vector3::new(((v[0]+1.) * (WID-1) as f64/2.),
                 ((v[1]+1.) * (HIG-1) as f64/2.),
                 ((v[2]+1.) * (DEP-1) as f64/2.)
    )
}

fn p3i(v: &Vector3<f64>) -> V3i {
    Vector3::new(((v[0]+1.) * (WID-1) as f64/2.) as i32,
                 ((v[1]+1.) * (HIG-1) as f64/2.) as i32,
                 ((v[2]+1.) * (DEP-1) as f64/2.) as i32
    )
}

fn draw_line<T: Pixel, U: GenericImage + GenericImageView<Pixel = T>>(p0: V3i, p1: V3i, img: &mut U, color: T) {
    let xlen = (p1.x-p0.x).abs();
    let ylen = (p1.y-p0.y).abs();
    let flag = (xlen < ylen) as i32;
    let len = xlen * (flag^1) + ylen * flag;

    for i in 0..len {
        img.put_pixel((p0.x+(p1.x-p0.x)*i/len) as u32, (p0.y+(p1.y-p0.y)*i/len) as u32, color);
    }
}

fn triangle(v012: [V3i; 3], vt012: [V3i; 3], zbuf: &mut Vec<u8>, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, ttv: &RgbaImage) {
    let [mut v0, mut v1, mut v2] = v012;
    let [mut vt0, mut vt1, mut vt2] = vt012;

    if v1.x > v0.x { swap(&mut v0, &mut v1); swap(&mut vt0, &mut vt1); }
    if v2.x > v0.x { swap(&mut v2, &mut v0); swap(&mut vt2, &mut vt0); }
    if v2.x > v1.x { swap(&mut v2, &mut v1); swap(&mut vt2, &mut vt1); }
    let x02l = v0.x - v2.x;
    let x01l = v0.x - v1.x;
    let x12l = v1.x - v2.x;

    let z = ((v0.z + v1.z + v2.z) / 3) as u8;
    for i in 0..x02l {

        let x02 = (v2.x - v0.x) * i / x02l + v0.x;
        let y02 = (v2.y - v0.y) * i / x02l + v0.y;
        let y11 = if i < x01l {v0.y+(v1.y-v0.y)*i/x01l} else {v1.y+(v2.y-v1.y)*(i-x01l)/x12l};
        let ylen = y02 - y11;
        let yneg = (ylen > 0) as i32 - (ylen < 0) as i32;

        let vtx02 = (vt2.x - vt0.x) * i / x02l + vt0.x;
        let vty02 = (vt2.y - vt0.y) * i / x02l + vt0.y;
        let vty11 = if i < x01l {vt0.y+(vt1.y-vt0.y)*i/x01l} else {vt1.y+(vt2.y-vt1.y)*(i-x01l)/x12l};
        let vtylen = vty02 - vty11;
        let vtyneg = (vtylen > 0) as i32 - (vtylen < 0) as i32;

        let y11l = ylen as i32 * yneg + 1;
        let vty11l = vtylen as i32 * vtyneg + 1;
        (0..y11l).for_each(|y|{
            let (vx, vy) = (x02 as u32, (y11+y*yneg).unsigned_abs());
            if zbuf[(vx *HIG+ vy) as usize] < z {
                img.put_pixel(vx, vy, *ttv.get_pixel(vtx02 as u32, (vty11+(y*vty11l*vtyneg/y11l)).unsigned_abs()));
                zbuf[(vx *HIG+ vy) as usize] = z;
            }
        })
    }
}


trait PixRevs {
    fn revs(&mut self);
}

impl PixRevs for RgbaImage {
    fn revs(&mut self) {
        let w = self.width();
        let h = self.height();
        for x0 in 0..w/2 {
            for y0 in 0..h {
                let x1 = w - 1 - x0;
                let y1 = h - 1 - y0;
                let temp = *self.get_pixel(x0, y0);
                self.put_pixel(x0, y0, *self.get_pixel(x1, y1));
                self.put_pixel(x1, y1, temp);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{

    let md = Module::new(CWD.clone() + "/res/obj/african_head.obj").unwrap();
    let mut ttv = image::open(CWD.clone() + "/res/img/african_head_diffuse.png").unwrap().to_rgba8();
    let mut img = image::RgbaImage::new(WID, HIG);
    let mut zbuf: Vec<u8> = vec![0; (HIG * WID) as usize];
    ttv.revs();
    let (ttvx, ttvy) = (ttv.width() as f64, ttv.height() as f64);

    let light_dir = Vector3::new(0f64,0f64,-1f64);
    for f in md.f {
        let (v0, v1, v2) = (&md.v[f[0]-1], &md.v[f[3]-1], &md.v[f[6]-1]);
        let vt012 = [&md.vt[f[1]-1], &md.vt[f[4]-1], &md.vt[f[7]-1]].map(|x| Vector3::new((x.x*ttvx) as i32, (x.y*ttvy) as i32, 0));
        let intensity = light_dir.dot(&(v0 - v2).cross(&(v1 - v2)).normalize().scale(-1.));

        let l = (intensity * 255f64) as u8;
        if intensity > 0f64 {
            triangle([p3i(v0), p3i(v1), p3i(v2)], vt012, &mut zbuf, &mut img, &ttv);
        }
    }

    img.revs();
    img.save(CWD.clone() + "/res/img/a.png").unwrap();
    Ok(())
}