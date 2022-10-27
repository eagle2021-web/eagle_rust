use std::fmt::{Debug, format};
use std::fs;
use std::fs::File;
use std::io::Seek;
use std::mem::replace;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use itertools::Itertools;
use common_utils::{replace_str};
fn main() {
    let app = App::new("cnt_size")
        .about("count dir size")
        .arg(Arg::with_name("root").short("r")
            .default_value("./"))
        .get_matches();
    let root_dir = app.value_of("root").expect("no root arg");
    let mut path_list = vec![];
    let cnt = cnt_dir_size(root_dir, &mut path_list);
    let arr = path_list.into_iter()
        .map(|(a, b)|(a, replace_str(b.to_str().unwrap())))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
    .map(|v| v).collect_vec();

    arr.iter().for_each(|v| println!("{:?}", v));
    echo_size(cnt);
}

fn cnt_dir_size<P>(p: P, path_list: &mut Vec<(usize, PathBuf)>)
                   -> usize where P: AsRef<Path> + Debug + Into<PathBuf> {
    let mut cnt = 0;

    let mut d = walkdir::WalkDir::new(&p)
        .max_depth(1)
        .min_depth(1);
    let mut it = d.into_iter();
    // it.skip_current_dir();
    for a in it {
        let a = a.unwrap();
        let sub_path = a.path().clone();
        // println!("sub_path = {:?}", sub_path);
        if sub_path.is_dir() {
            cnt += cnt_dir_size(sub_path, path_list);
        } else {
            let f = fs::File::open(sub_path).expect("open file err");
            cnt += f.metadata().expect("read metadata err").len() as usize;
        }
    }
    let pb: PathBuf = p.into();
    path_list.push((cnt, pb));
    return cnt;
}

fn echo_size(cnt: usize) {
    const SIZE: f64 = 1024_f64;
    let mut cnt = cnt as f64;
    let mut index = 0;
    let candidates = vec!["B", "KB", "MB", "GB"];
    while cnt >= SIZE {
        cnt /= SIZE;
        index += 1;
    }
    let suf = candidates[index];
    let s = format!("{}{}", cnt, suf);
    println!("size = {}", s);
}