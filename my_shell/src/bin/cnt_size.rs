use std::fs;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use itertools::Itertools;
use std::fmt::Debug;
use common_utils::replace_str;
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

    let it = walkdir::WalkDir::new(&p)
        .max_depth(1)
        .min_depth(1)
        .into_iter();
    for a in it {
        let dir = a.unwrap();
        let sub_path = dir.path().clone();
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

#[cfg(test)]
mod test {
    use rayon::prelude::*;
    use std::fs;
    use test_cfg::project_root;


    #[test]
    fn test_cpu_num() {
        let num = num_cpus::get();
        println!("cpu_num = {}", num);
        let num = num_cpus::get_physical();
        println!("cpu_physical_num =  {num}");
    }

    #[test]
    fn test_a() {
        let path = project_root();
        let p = path.join("my_shell/src/bin/cnt_size.rs");
        let p2 = path.join("my_shell/src/bin/cp_bin.rs");
        let input = vec![p, p2];
        input
        .par_iter()
        .for_each(|sub_path| {
            let f = fs::File::open(sub_path).expect("open file err");
            let cnt = f.metadata().expect("read metadata err").len() as usize;
            println!("{cnt}");
        })
        ;
    }

    #[test]
    fn test_b() {
        let p = project_root();

        let dir = walkdir::WalkDir::new(p)
            .min_depth(1)
            .max_depth(100);
        let mut arr = vec![];
        for v in dir.into_iter() {
            if let Ok(a) = v {
                arr.push(a.file_name().to_os_string());
            }
        }
        print!("len = {}", arr.len());

    }
}