use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::str::{FromStr, SplitWhitespace};
use nalgebra::Vector3;

use crate::define::*;
use crate::matrix::*;

pub struct Module {
    pub v: Vec<V4d>,
    pub f: Vec<Vec<usize>>,
    pub vt: Vec<V4d>,
    pub vn: Vec<V4d>
}

fn pnext<T: Debug + FromStr>(p: Option<&str>) -> T where <T as FromStr>::Err: Debug  {
    p.unwrap().parse().unwrap()
}

impl Module {
    pub fn new(fpath: String) -> Result<Module, Box<dyn Error>> {

        let d = fs::read_to_string(fpath)?;

        let mut v: Vec<V4d> = Vec::with_capacity(1258);
        let mut f: Vec<Vec<usize>> = Vec::with_capacity(2492);
        let mut vt: Vec<V4d> = Vec::with_capacity(1339);
        let mut vn: Vec<V4d> = Vec::with_capacity(1258);
        for p in d.split("\n") {
            let mut p = p.split_whitespace();
            match p.next().unwrap_or(" ") {
                "#" => {println!("{}", p.collect::<String>())}
                "v" => {v.push(V4d{x:pnext(p.next()), y:pnext(p.next()), z:pnext(p.next()), w:1.});}
                "f" => {f.push(p.flat_map(|x| x.split("/").map(|y| y.parse::<usize>().unwrap())).collect());}
                // "f" => {f.push((p.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()-1,p.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()-1,p.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()-1)); }
                "vt" => {vt.push(V4d{x:pnext(p.next()), y:pnext(p.next()), z:pnext(p.next()), w:1.});}
                "vn" => {vn.push(V4d{x:pnext(p.next()), y:pnext(p.next()), z:pnext(p.next()), w:1.});}
                _ => {}
            };
        }
        Ok(Module { v, f, vt, vn })
    }
}
