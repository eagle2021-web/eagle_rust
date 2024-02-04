mod models;
use serde::Deserialize;
use std::{env, fs};
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::models::settings::Settings;

// 注意添加 xmlns 字段对应 XML namespace，以确保反序列化成功

fn read_settings<P: AsRef<Path>>(path: P) -> Result<Settings, io::Error> {
    // Read the file to a string
    let xml_content = fs::read_to_string(path)?;

    // Use serde_xml_rs to deserialize the XML string into a Settings object
    serde_xml_rs::from_str(&xml_content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}
fn read_settings_get_tuple<P: AsRef<Path>>(path: P)-> Result<Vec<(String, String)>, io::Error>{
    let sets = read_settings(path).expect("parse settings err");
    let mut prefix_arr = vec![];
    println!("sets = {:?}", sets);
    let vs = sets.mirrors.mirror.unwrap_or(vec![]);
    for x in &vs {
        let id = x.id.as_ref().unwrap().to_owned();
        let url = x.url.as_ref().unwrap().to_owned();
        prefix_arr.push((id, url));
    }
    let vs = sets.profiles.profile.unwrap_or(vec![]);
    for x in &vs {
        if let Some(reps1) = x.repositories.as_ref() {
            let tmp = vec![];
            let reps1 = reps1.repository.as_ref().unwrap_or(&tmp);
            for rep in reps1 {
                let id = rep.id.clone();
                let url = rep.url.clone();
                prefix_arr.push((id, url));
            }
        }
        if let Some(reps2) = x.plugin_repositories.as_ref() {
            let tmp = vec![];
            let reps2 = reps2.plugin_repository.as_ref().unwrap_or(&tmp);
            for rep in reps2 {
                let id = rep.id.clone();
                let url = rep.url.clone();
                prefix_arr.push((id, url));
            }
        }
    }
    Ok(prefix_arr)
}
fn read_settings_get_urls<P: AsRef<Path>>(path: P)-> Result<Vec<String>, io::Error>{
    let sets = read_settings(path).expect("parse settings err");
    let prefix_arr = vec![];
    let s = sets.mirrors.mirror.unwrap();
    println!("{:?}", s);
    Ok(prefix_arr)
}
fn main() {
    // Specify the path to your settings.xml file
    let home_s =  env::var("USERPROFILE").expect("get home path failed.");
    let mut settings_path = PathBuf::from_str(&home_s).expect("to path_buf err");
    settings_path.push(".m2/settings.xml");
    println!("{:?}", settings_path);

    let settings_result = read_settings(settings_path);

    match settings_result {
        Ok(settings) => {
            println!("Settings: {:?}", settings);
        },
        Err(e) => {
            println!("Error reading settings: {}", e);
        },
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;
    use std::str::FromStr;
    use crate::{read_settings, read_settings_get_tuple, read_settings_get_urls};

    #[test]
    fn test_read_urls(){
        let home_s =  env::var("USERPROFILE").expect("get home path failed.");
        let mut settings_path = PathBuf::from_str(&home_s).expect("to path_buf err");
        settings_path.push(".m2/settings.xml");
        println!("{:?}", settings_path);
        let settings_result = read_settings_get_tuple(settings_path).unwrap();
        println!("settings_result = {:?}", settings_result)
    }
}