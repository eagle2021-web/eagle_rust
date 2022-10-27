use std::fmt::{Debug, format};
use std::fs;
use std::fs::File;
use std::io::Seek;
use std::path::Path;
use clap::{App, Arg};

fn main() {
    let app = App::new("cnt_size")
        .about("count dir size")
        .arg(Arg::with_name("root").short("r")
            .default_value("./"))
        .get_matches();
    let root_dir = app.value_of("root").expect("no root arg");
    let cnt = cnt_dir_size(root_dir);
    echo_size(cnt);
}

fn cnt_dir_size<P: AsRef<Path> + Debug>(p: P) -> usize {
    let d = walkdir::WalkDir::new(&p);
    let mut cnt = 0;
    let mut it = d.into_iter().skip(1);
    // it.skip_current_dir();
    for a in it {
        let a = a.unwrap();
        let sub_path = a.path().clone();
        // println!("sub_path = {:?}", sub_path);
        if sub_path.is_dir() {
            cnt += cnt_dir_size(sub_path);
        } else {
            let f = fs::File::open(sub_path).expect("open file err");
            cnt += f.metadata().expect("read metadata err").len() as usize;
        }
    }

    return cnt;
}

fn echo_size(cnt: usize) {
    const SIZE: usize = 1024;
    let mut cnt = cnt / 8;
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