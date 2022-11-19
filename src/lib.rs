#![feature(str_split_as_str)]
#![feature(once_cell)]

mod mobj;
mod matrix;

use matrix::*;
use mobj::Module;

use std::mem::swap;
use std::f64::consts::PI;
use std::io::Bytes;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData, console::log_1};
use serde::{ Deserializer, Serializer, Deserialize, Serialize };
use wasm_bindgen::convert::IntoWasmAbi;

struct Dim {
    w: u32,
    h: u32,
    d: u8
}

fn p3i(v: &V4d, d: &Dim) -> V3i {
    V3t {
        // x: (v.x * d.w as f64) as i32,
        // y: (v.y * d.h as f64) as i32,
        // z: (v.z * d.d as f64) as i32,
        x: ((v.x + 1.) * (d.w - 1) as f64 / 2.) as i32,
        y: ((v.y + 1.) * (d.h - 1) as f64 / 2.) as i32,
        z: ((v.z + 1.) * (d.d - 1) as f64 / 2.) as i32
    }
}

fn p3d(v: &V4d, d: &Dim) -> V3d {
    V3t {
        x: (v.x+1.) * (d.w-1) as f64/2.,
        y: (v.y+1.) * (d.h-1) as f64/2.,
        z: (v.z+1.) * (d.d-1) as f64/2.
    }
}

type Ctx = CanvasRenderingContext2d;

fn xy([v0, v1, v2]: [&V3i; 3], [i, x02l, x01l, x12l]: [i32; 4]) -> [i32; 4] {
    let x02 = (v2.x - v0.x) * i / x02l + v0.x;
    let y11 = if i < x01l {v0.y+(v1.y-v0.y)*i/x01l} else {v1.y+(v2.y-v1.y)*(i-x01l)/x12l};
    let yl = (v2.y - v0.y) * i / x02l + v0.y - y11;
    let yneg = (yl > 0) as i32 - (yl < 0) as i32;
    let y11l = yl * yneg + 1;
    [x02, y11, yneg, y11l]
}

fn triangle(v012: [V3t<i32>; 3], t012: [V3t<i32>; 3], img: &mut [u8], vtt: &[u8], vidm: &Dim, zbuf: &mut [u8]){
    let [mut v0, mut v1, mut v2] = v012;
    let [mut t0, mut t1, mut t2] = t012;
    let [w, h] = [vidm.w as i32, vidm.h as i32];

    if v1.x > v0.x { swap(&mut v0, &mut v1); swap(&mut t0, &mut t1); }
    if v2.x > v0.x { swap(&mut v2, &mut v0); swap(&mut t2, &mut t0); }
    if v2.x > v1.x { swap(&mut v2, &mut v1); swap(&mut t2, &mut t1); }
    let [x02l, x01l, x12l] = [v0.x-v2.x, v0.x-v1.x, v1.x-v2.x];

    let z = ((v0.z + v1.z + v2.z) / 3) as u8;
    for i in 0..x02l {
        let [vx02, vy11, vyneg, vy11l] = xy([&v0,&v1,&v2], [i, x02l, x01l, x12l]);
        let [tx02, ty11, tyneg, ty11l] = xy([&t0,&t1,&t2], [i, x02l, x01l, x12l]);

        for y in 0..vy11l {
            // let (vx, vy) = (vx02, (vy11+y*vyneg));
            let [vx, vy] = [vx02, (vy11+y*vyneg)];
            if vx < w && vy< h && zbuf[(vx*w+vy) as usize] < z {
                // let [tx, ty] = [tx02 as usize, (ty11+(y*ty11l*tyneg/vy11l)) as usize];
                (0..4).for_each(|i| {img[i+4*(vy*h+vx) as usize] = vtt[i+4*((1023-(ty11+(y*ty11l*tyneg/vy11l)))*1024+tx02) as usize];});
                // log_1(&format!("{vx} - {vy}, {}, {}, {}, {w}, {h}", vx*4*w+vy, vx*4*w+vy*4, vx*h+vy).into());
                // rctx.put_image_data(dt, vx as f64, vy as f64).unwrap();
                // rctx.set_fill_style(&format!("#{l:x}{l:x}{l:x}FF").into());
                // rctx.fill_rect(vx as f64, vy as f64, 1., 1.);
                zbuf[(vx*h+vy) as usize] = z;
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Trans {
    t: u8,
    v: [f64; 3]
}

#[derive(Debug, Default, Deserialize)]
struct Pt {
    x: f64,
    y: f64
}

#[derive(Debug, Deserialize)]
struct Opt {
    trans: Vec<Trans>,
    cvs_xy: Pt,
}

#[derive(Debug, Serialize, Default)]
struct Stat {
    draw_count: u32,
}


#[wasm_bindgen]
#[derive(Default)]
pub struct Obj {
    md: Option<Module>,
    vtt: Option<Vec<u8>>,
    vtt_xy: Pt
}

fn transform(li: &Vec<Trans>) -> Mx4d {
    let mut t = Mx4d::rot([PI,0.,0.]);
    for i in li {
        t = t * match i.t {
            0 => {Mx4d::rot(i.v)}
            1 => {Mx4d::trans(i.v)}
            2 => {Mx4d::scale(i.v)}
            3 => {Mx4d::proj(i.v)}
            _ => {Mx4d::identity()}
        }
    }
    t
}


#[wasm_bindgen]
pub fn rload(md_str: Option<String>, vtt: Option<Vec<u8>>, xy: Vec<f64>) -> Obj{
    match (md_str, vtt) {
        (Some(s), Some(v)) => { Obj { md: Some(Module::new(s)), vtt: Some(v), vtt_xy: Pt {x: xy[0], y: xy[1]} } }
        (None, Some(v)) => { Obj { vtt: Some(v), vtt_xy: Pt {x: xy[0], y: xy[1]}, ..Default::default() } }
        (Some(s), None) => { Obj { md: Some(Module::new(s)), ..Default::default() } }
        _ => {panic!("miao!")}
    }
}

#[wasm_bindgen]
pub fn draw(obj: Obj, rctx: &Ctx, opt: JsValue) -> Result<(), JsValue>{

    let opt: Opt = serde_wasm_bindgen::from_value(opt).unwrap();
    let (md, vtt) = (obj.md.unwrap(), obj.vtt.unwrap());
    let mut stat = Stat::default();
    let t = transform(&opt.trans);

    let [ttvx, ttvy] = [obj.vtt_xy.x, obj.vtt_xy.y];
    let d = if opt.cvs_xy.x > opt.cvs_xy.y {opt.cvs_xy.y as u32} else { opt.cvs_xy.x as u32};
    let vdim = Dim {w: d, h: d, d: 255};
    let mut img: Vec<u8> = vec![0; (d*d*4) as usize];

    let mut zbuf: Vec<u8> = vec![0; d.pow(2) as usize];
    let facing = V3d::new(0., 0., 1.);
    let transV = md.v().iter().map(|v| t * v).collect::<Vec<V4d>>();
    for f in md.f() {
        let v = [transV[f[0]-1], transV[f[3]-1], transV[f[6]-1]];
        let front = facing.dot((v[0]- v[2]).v3t().cross((v[1]- v[2]).v3t()).normalize().scale(-1.));
        if front > 0f64 {
            let v012 = v.map(|x| p3i(&x, &vdim));
            let t012 = [&md.vt()[f[1]-1], &md.vt()[f[4]-1], &md.vt()[f[7]-1]].map(|x| V3i{x: (x.x*ttvx) as i32, y: (x.y*ttvy) as i32, z: 0});
            // let l = (front * 255.) as u8;
            // rctx.set_fill_style(&format!("#{l:x}{l:x}{l:x}FF").into());
            triangle(v012, t012, &mut img, &vtt, &vdim, &mut zbuf);
        } else {
            stat.draw_count += 1;
        }
    }
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&img), d, d)?;
    // log_1(&format!("{img:?}").into());
    rctx.put_image_data(&data, 0.0, 0.0)?;
    log_1(&stat.draw_count.into());
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
