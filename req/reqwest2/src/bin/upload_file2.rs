use std::fs::File;
use std::io::Read;
use reqwest::multipart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = reqwest::Client::builder()
    //     .build()?;
    // let a = std::fs::read("C:/Users/Administrator/AppData/Roaming/JetBrains/IntelliJIdea2023.3/scratches/generated-requests.http")?;
    // let b = std::fs::read("D:/projects/rust/eagle_rust/Cargo.toml")?;
    // let form = reqwest::multipart::Form::new()
    //     .part("file", reqwest::multipart::Part::bytes(a)
    //         .file_name("generated-requests.http"))
    //     .part("file2", reqwest::multipart::Part::bytes(b)
    //         .file_name("Cargo.toml"))
    //     ;
    // let request = client.request(reqwest::Method::POST, "http://localhost:8115/sw/save2")
    //     .multipart(form);
    //
    // let response = request.send().await?;
    // let hs = response.headers();
    // for (a, b) in hs {
    //     println!("{:?}: {:?}" ,a , b);
    // }
    // let body = response.text().await?;
    //
    // println!("{}", body);
    upload_file().await?;
    Ok(())
}
async fn upload_file() -> Result<(), Box<dyn std::error::Error>> {
    // 打开要上传的文件
    let mut file = File::open("C:/Users/Administrator/AppData/Roaming/JetBrains/IntelliJIdea2023.3/scratches/generated-requests.http")?;

    // 创建一个向服务器发送请求的客户端
    let client = reqwest::Client::new();

    // 创建一个空的缓冲区来存储文件内容
    let mut buffer = Vec::new();

    // 将文件读取到缓冲区
    file.read_to_end(&mut buffer)?;

    // 构建一个文件part
    let form = multipart::Form::new()
        .part("file", multipart::Part::bytes(buffer).file_name("filename.ext"));

    // 发送请求并等待响应
    let response = client
        .post("http://localhost:8115/sw/save")
        .multipart(form)
        .send()
        .await?;

    // 检查响应状态码
    if response.status().is_success() {
        println!("文件上传成功!");
        Ok(())
    } else {
        println!("文件上传失败!");
        Err("上传失败".into())
    }
}
