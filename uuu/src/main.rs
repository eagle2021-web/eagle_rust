mod models;
use serde::Deserialize;
use std::{env, fs};
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use druid_shell::Application;
use crate::models::dependency::Dependency;
use crate::models::settings::Settings;

// 注意添加 xmlns 字段对应 XML namespace，以确保反序列化成功
#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    // let a = req().await.unwrap();
    // cliboard_to_suf();
    Ok(())
}

async fn req()-> Result<bool, reqwest::Error> {
    // Specify the path to your settings.xml file
    let home_s =  env::var("USERPROFILE").expect("get home path failed.");
    let mut settings_path = PathBuf::from_str(&home_s).expect("to path_buf err");
    settings_path.push(".m2/settings.xml");
    println!("{:?}", settings_path);
    // 创建一个新的 reqwest 客户端实例
    let client = reqwest::Client::new();
    // 使用客户端通过 HTTP GET 发送请求
    // 创建一个新的 reqwest 客户端实例
    let client = reqwest::Client::new();
    // 使用客户端通过 HTTP GET 发送请求，并添加User-Agent
    let resp = client
        .get("https://repo.maven.apache.org/maven2/org/frankframework/")
        // .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:15.0) Gecko/20100101 Firefox/15.0.1")
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send().await?;

    // 我们检查 HTTP 请求返回的状态是成功
    if resp.status().is_success() {
        // 然后，我们从响应体中读取主体内容并转换为 UTF-8 文本
        let body = resp.text().await?;
        // 打印出来
        println!("{}", body);
    }
    else {
        println!("{}", resp.status());
        return Ok(false);
    }

    Ok(true)
}

