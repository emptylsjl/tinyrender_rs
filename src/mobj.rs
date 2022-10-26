use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::str::{FromStr, SplitWhitespace};
use nalgebra::Vector3;

pub struct Module {
    pub v: Vec<Vector3<f64>>,
    pub f: Vec<Vec<usize>>,
    pub vt: Vec<Vector3<f64>>,
    pub vn: Vec<Vector3<f64>>
}

fn pnext<T: Debug + FromStr>(p: Option<&str>) -> T where <T as FromStr>::Err: Debug  {
    p.unwrap().parse().unwrap()
}

impl Module {
    pub fn new(fpath: String) -> Result<Module, Box<dyn Error>> {

        let d = fs::read_to_string(fpath)?;

        let mut v: Vec<Vector3<f64>> = Vec::with_capacity(1258);
        let mut f: Vec<Vec<usize>> = Vec::with_capacity(2492);
        let mut vt: Vec<Vector3<f64>> = Vec::with_capacity(1339);
        let mut vn: Vec<Vector3<f64>> = Vec::with_capacity(1258);
        for p in d.split("\n") {
            let mut p = p.split_whitespace();
            match p.next().unwrap_or(" ") {
                "#" => {println!("{}", p.collect::<String>())}
                "v" => {v.push(Vector3::new(pnext(p.next()), pnext(p.next()), pnext(p.next())));}
                "f" => {f.push(p.flat_map(|x| x.split("/").map(|y| y.parse::<usize>().unwrap())).collect());}
                // "f" => {f.push((p.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()-1,p.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()-1,p.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()-1)); }
                "vt" => {vt.push(Vector3::new(pnext(p.next()), pnext(p.next()), pnext(p.next())));}
                "vn" => {vn.push(Vector3::new(pnext(p.next()), pnext(p.next()), pnext(p.next())));}
                _ => {}
            };
        }
        Ok(Module { v, f, vt, vn })
    }
}
