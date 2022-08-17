use serde_json;
use reqwest;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = match env::args().nth(1) {
        None => "https://jsonplaceholder.typicode.com/posts".to_string(),
        Some(s) => s
    };
    let js: serde_json::Value = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .no_proxy()
        .build()
        .unwrap()
        .get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let output_json_path = match env::args().nth(2) {
        None => {
            println!("{}", "输出json数据在tmp.json");
            "./tmp.json".to_string()
        }
        Some(s) => s
    };
    fs::write(&output_json_path, js.to_string())
        .expect("Failed to write!");
    assert!(js.is_array());
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use reqwest;

    #[actix_rt::test]
    async fn test_a() {
        let url = "https://item.jd.com/100031943534.html#none";
        let text = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .no_proxy()
            .build()
            .unwrap()
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{:?}", text);
        fs::write("tmp.html", text).expect("Failed to write");
    }
}