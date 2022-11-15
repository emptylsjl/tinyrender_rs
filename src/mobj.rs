use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::str::{FromStr, SplitWhitespace};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::matrix::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Module {
    v: Vec<V4d>,
    f: Vec<Vec<usize>>,
    vt: Vec<V4d>,
    vn: Vec<V4d>
}

fn pnext<T: Debug + FromStr>(p: Option<&str>) -> T where <T as FromStr>::Err: Debug  {
    p.unwrap().parse().unwrap()
}

impl Module {
    pub fn new(obj_str: String) -> Module {

        let mut v: Vec<V4d> = vec![];
        let mut f: Vec<Vec<usize>> = vec![];
        let mut vt: Vec<V4d> = vec![];
        let mut vn: Vec<V4d> = vec![];
        for p in obj_str.split("\n") {
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
        Module { v, f, vt, vn }
    }

    pub fn v(&self) -> &Vec<V4d> {
        &self.v
    }

    pub fn f(&self) -> &Vec<Vec<usize>> {
        &self.f
    }

    pub fn vt(&self) -> &Vec<V4d> {
        &self.vt
    }

    pub fn vn(&self) -> &Vec<V4d> {
        &self.vn
    }
}
