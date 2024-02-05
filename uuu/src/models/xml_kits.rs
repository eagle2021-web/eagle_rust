use std::{fs, io};
use std::path::Path;
use std::str::FromStr;
use druid_shell::Application;
use crate::models::dependency::Dependency;
use crate::models::settings::Settings;

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

fn cliboard_to_suf()-> Option<String> {
    let mut suf = String::new();
    let mut clipboard = Application::new().unwrap().clipboard();
    if let Some(contents) = clipboard.get_string() {
        return dependency_to_suf(&contents);
    }
    None
}
fn dependency_to_suf(contents: &str) -> Option<String> {
    let mut suf = String::new();
    if contents.contains("groupId") {
        // Use serde_xml_rs to deserialize the XML string into a Settings object
        let mut dep : Dependency = serde_xml_rs::from_str(&contents).expect("parse dependency err.");
        if let Some(group) = dep.group_id {
            let mut tmp = group.split(".");
            let mut s = String::new();
            while let Some(part) = tmp.next() {
                s.push_str(part);
                s.push_str("/");
            }
            s = String::from_str(&s[0..s.len() - 1]).unwrap();
            dep.group_id = Some(s);
        }
        let arr = vec![dep.group_id, dep.artifact_id, dep.version];
        for x in arr {
            if x.is_none() {
                break;
            }
            let s = x.unwrap();
            suf.push_str(&s);
            suf.push_str("/");
        }
    } else if contents.contains(":") {

    }
    if suf.is_empty() {
        return None;
    }
    println!("suf = {:?}", suf);
    Some(suf)
}
#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;
    use std::str::FromStr;
    use super::{dependency_to_suf, read_settings, read_settings_get_tuple, read_settings_get_urls};

    #[test]
    fn test_read_urls(){
        let home_s =  env::var("USERPROFILE").expect("get home path failed.");
        let mut settings_path = PathBuf::from_str(&home_s).expect("to path_buf err");
        settings_path.push(".m2/settings.xml");
        println!("{:?}", settings_path);
        let settings_result = read_settings_get_tuple(settings_path).unwrap();
        println!("settings_result = {:?}", settings_result)
    }

    #[test]
    fn test_dependency_to_suf(){
        let s = r#"<dependency>
            <groupId>com.eagle</groupId>
            <artifactId>framework</artifactId>
            <version>1.2.3</version>
        </dependency>"#;
        let suf = dependency_to_suf(s);
        let s = suf.as_ref().unwrap();
        assert_eq!("com/eagle/framework/1.2.3/", s);
        println!("suf = {:?}", suf);
    }
}