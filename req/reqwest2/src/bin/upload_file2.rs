#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;
    let a = std::fs::read("C:/Users/Administrator/AppData/Roaming/JetBrains/IntelliJIdea2023.3/scratches/generated-requests.http")?;
    let b = std::fs::read("D:/projects/rust/eagle_rust/Cargo.toml")?;
    let form = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(a)
            .file_name("generated-requests.http"))
        .part("file2", reqwest::multipart::Part::bytes(b)
            .file_name("Cargo.toml"))
        ;
    let request = client.request(reqwest::Method::POST, "http://localhost:8115/sw/save2")
        .multipart(form);

    let response = request.send().await?;
    let hs = response.headers();
    for (a, b) in hs {
        println!("{:?}: {:?}" ,a , b);
    }
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}