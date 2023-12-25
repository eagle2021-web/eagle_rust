use tokio::fs:: File;
use tokio::io::AsyncWriteExt;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fmt::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "https://www.zenodo.org/record/7834392/files/full_dataset.tsv.gz.part-ac?download=1";
    let url = "https://www.zenodo.org/records/7562456/files/annealed%20samples.zip?download=1";
    // let url = "https://dl.vmecum.com/VMware.exe";
    // let url = "https://dl.hdslb.com/bili/bililive/win/Livehime-Win-beta-4.57.1.6192-x64.exe";
    // let url = "https://huggingface.co/microsoft/phi-2/resolve/main/model-00001-of-00002.safetensors";
    // let url = "https://sys-10-1.xiaoguaniu.com/win10/202312/G_WIN10_X64_19045.3803.iso?time=1703515342&ip=121.35.3.236&secret=28495047c58237590cdc4ec168b3a44a";

    let url_mod = reqwest::Url::parse(url)?;
    let path = url_mod.path();
    let filename = *path.split("/").collect::<Vec<&str>>().last().unwrap();
    println!("filename = {}", filename);
    let mut file_path = PathBuf::from("d:/temp/");
    file_path = file_path.join(filename);
    let client = reqwest::Client::new();

    // 创建一个 HeaderMap，并添加自定义的 header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::RANGE, reqwest::header::HeaderValue::from_static("bytes=0-"));

    let mut response = client.get(url).headers(headers).send().await?;
    for x in response.headers() {
        println!("{}: {:?}", x.0, x.1);
    }

    let mut file = File::create(&file_path).await?;

    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded_size = 0u64;
    let pb = ProgressBar::new(total_size);

    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    pb.finish_with_message("downloaded");
    while let Some(chunk) = response.chunk().await.unwrap() {
        file.write_all(&chunk).await?;
        downloaded_size += chunk.len() as u64;
        let msg = format!("Downloaded: {} / {} bytes", downloaded_size, total_size);
        pb.set_message(msg);
        pb.set_position(downloaded_size);
    }

    // 下载完成后关闭文件
    file.sync_all().await?;
    pb.finish_with_message("Download complete");
    
    Ok(())
}
