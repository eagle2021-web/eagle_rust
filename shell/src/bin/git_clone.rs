use std::{env, fs, thread};
use std::ops::Index;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use serde_json::{json, Value};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::time::Duration;

fn main() {
    let path = env::args().nth(1).expect("no path");
    let s = fs::read_to_string(path).expect("Failed to read");
    let arr_value: Value = serde_json::from_str(&s).expect("not value");
    // println!("{:?}", arr_value);
    let (ts, tr) = mpsc::channel();
    let mut res = json!([]);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let arr = Arc::new(arr_value.as_array().unwrap().clone());
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        let ts = mpsc::Sender::clone(&ts);
        let arr = Arc::clone(&arr);
        let handle = thread::spawn(move || {
            loop {
                let index;
                {
                    let mut num = counter.lock().unwrap();

                    index = *num;
                    if index >= arr.len() as i32 {
                        break;
                    }
                    *num += 1;
                }
                let value = &(*arr);
                let clone_obj = &value[index as usize];
                clone_one(&ts, clone_obj);
            }
        });
        handles.push(handle);
    }
    drop(ts);
    let mut res = json!([]);
    for v in tr {
        res.as_array_mut().unwrap().push(v);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    fs::write("tmp.json", res.to_string()).expect("Failed to write");
}

fn clone_one(sender: &Sender<Value>, value: &Value) {
    let obj = value.as_object().expect("not obj");
    let url = obj.get("url").expect("no url").as_str().expect("not str");
    let tag = obj.get("tag").expect("no tag").as_str().expect("not str");
    let path = url.replace("https://", "d:/tmp/");
    let path_buf = PathBuf::from_str(&path).expect("not path");
    if path_buf.exists() {
        fs::remove_dir_all(path_buf).unwrap();
    }
    let mut c = Command::new("git");
    c.args(["clone", "--depth", "1", "--branch", tag, url, &path]);
    // --depth 1 --branch <tag_name> <repo_url>
    let output = c.output().expect("no output");
    let code = output.status.code().unwrap_or(-1);
    let mut clone_ok = true;
    if code != 0 {
        clone_ok = false;
    }
    let mut js = value.clone();
    js["clone_ok"] = json!(clone_ok);
    js["clone_msg"] = json!("ok");
    if !clone_ok {
        let msg;
        unsafe {
            msg = String::from_utf8_unchecked(output.stderr.clone());
        }
        js["clone_msg"] = json!(&msg);
    }
    println!("{:?}", output);
    sender.send(js);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_a() {
        let a = vec!["aaa"];
        let b = a[0];
    }
}