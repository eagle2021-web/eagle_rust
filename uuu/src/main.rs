mod models;
use serde::Deserialize;
use std::{env, fs};
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::models::settings::Settings;

// 注意添加 xmlns 字段对应 XML namespace，以确保反序列化成功
fn main() {
    // Specify the path to your settings.xml file
    let home_s =  env::var("USERPROFILE").expect("get home path failed.");
    let mut settings_path = PathBuf::from_str(&home_s).expect("to path_buf err");
    settings_path.push(".m2/settings.xml");
    println!("{:?}", settings_path);

}

