use std::ops::Not;
use std::path::{Path, PathBuf};
use clap::{App, Arg};
use color_eyre::Report;

#[tokio::main]
async fn main() -> Result<(), Report> {
        color_eyre::install()?; // Initialize color-eyre.
    let app = App::new("upload")
        .about("upload file.")
        .arg(Arg::with_name("url").short("u")

            .default_value("http://localhost:8115/sw/save"))
        .arg(Arg::with_name("file").short("-f")
            .default_value("./Cargo.toml"))
        .get_matches();
    let url = app.value_of("url").unwrap();
    let file_path_s = app.value_of("file").unwrap();
    let file_path = PathBuf::from(file_path_s);
    if file_path.is_file().not() {
        panic!("file path {} no exist.", file_path_s);
    }
    let client = reqwest::Client::builder()
        .build()?;
    let a = std::fs::read(file_path_s)?;
    let form = reqwest::multipart::Form::new()
        .part("file", reqwest::multipart::Part::bytes(a)
            .file_name("generated-requests.http"));

    let request = client.request(reqwest::Method::POST, url)
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