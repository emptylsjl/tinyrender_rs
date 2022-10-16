#![feature(str_split_as_str)]


mod tga;
mod mobj;

use mem::swap;
use mobj::Module;

use std::error::Error;
use std::{env, mem};
use rand::Rng;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Primitive, Rgb, Rgba};
use nalgebra::Vector3;

#[derive(Default)]
struct P2i {
    x: i32,
    y: i32
}


fn p2u(v: &Vector3<f64>) -> P2i {
    P2i {x: ((v[0]+1.) * 800./2.) as i32, y: ((v[1]+1.) * 800./2.) as i32}
}

fn draw_line<T: Pixel, U: GenericImage + GenericImageView<Pixel = T>>(p0: P2i, p1: P2i, img: &mut U, color: T) {
    let xlen = (p1.x-p0.x).abs();
    let ylen = (p1.y-p0.y).abs();
    let flag = (xlen < ylen) as i32;
    let len = xlen * (flag^1) + ylen * flag;

    for i in 0..len {
        img.put_pixel((p0.x+(p1.x-p0.x)*i/len) as u32, (p0.y+(p1.y-p0.y)*i/len) as u32, color);
    }

}

fn triangle<T: Pixel, U: GenericImage + GenericImageView<Pixel = T>>((mut p0, mut p1, mut p2): (P2i, P2i, P2i), img: &mut U, color: T) {

    if p1.x > p0.x { swap(&mut p0, &mut p1); }
    if p2.x > p0.x { swap(&mut p2, &mut p0); }
    if p2.x > p1.x { swap(&mut p2, &mut p1); }
    let x02len = (p2.x - p0.x).abs();
    let x01len = (p0.x - p1.x).abs();
    let x12len = (p1.x - p2.x).abs();

    for i in 0..x02len {

        let x02 = p0.x + ((i * (p2.x - p0.x)) / x02len);
        let y02 = p0.y + ((i * (p2.y - p0.y)) / x02len);
        let y11;
        if i < x01len { y11 = (p0.y+(p1.y- p0.y)*i/x01len); }
        else { y11 = (p1.y+(p2.y- p1.y)*(i-x01len)/x12len); }

        let ylen = y02 - y11;
        let neg = (ylen > 0) as i32 - (ylen < 0) as i32;

        (0..ylen*neg+1).for_each(|y|{
            img.put_pixel(x02 as u32, (y11+y*neg) as u32, color);
        })
    }
}

fn main() -> Result<(), Box<dyn Error>>{

    // let mut white = [255, 255, 255, 255];
    // let mut red = [255, 0, 0, 255];
    // let mut green = [0, 255, 0, 255];
    // let mut blue = [0, 0, 255, 255];
    // white.reverse();
    // red.reverse();
    // green.reverse();
    // blue.reverse();
    // let white = Rgba(white);
    // let red = Rgba(red);
    // let green = Rgba(green);
    // let blue = Rgba(blue);


    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let md = Module::new(cwd.clone() + "/res/obj/african_head.obj").unwrap();
    let mut img = image::RgbaImage::new(801, 801);

    // draw_line(P2i{x: 20,  y: 34},  P2i{x: 744, y: 400}, &mut img, red);
    // draw_line(P2i{x: 120, y: 434}, P2i{x: 444, y: 400}, &mut img, green);
    // draw_line(P2i{x: 330, y: 463}, P2i{x: 594, y: 200}, &mut img, blue);
    //
    // draw_line(P2i{x: 10, y: 10}, P2i{ x: 790, y: 10}, &mut img, white);

    let light_dir = Vector3::new(0f64,0f64,-1f64);
    for f in md.f {
        let (i, j, k) = (&md.v[f[0]], &md.v[f[1]], &md.v[f[2]]);
        let pts = (p2u(i), p2u(j), p2u(k));
        //
        let n = (i-k).cross(&(j-k));
        let b = n.normalize();
        let intensity = light_dir.dot(&b.scale(-1.));
        let c = (intensity * 255f64) as u8;

        if intensity > 0f64 {
            triangle(pts, &mut img, Rgba([c, c, c, c]));
        }
    }

    img.reverse();
    img.save(cwd.clone() + "/res/img/b.png").unwrap();
    Ok(())
}