use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use crypto_hash::{Algorithm, Hasher};

const SUPPORTED_ALGORITHMS: [(Algorithm, &'static str); 4] = [
    (Algorithm::MD5, "MD5"),
    (Algorithm::SHA1, "SHA-1"),
    (Algorithm::SHA256, "SHA-256"),
    (Algorithm::SHA512, "SHA-512"),
];

fn main() -> Result<(), Box<dyn Error>> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 解析命令行参数
    let file_path = args.get(1).ok_or("File path not specified")?;

    // 打开文件
    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();

    // 分块计算并输出所有哈希值
    let mut hashers = vec![];
    for (algorithm, algorithm_name) in SUPPORTED_ALGORITHMS.iter() {
        let mut hasher = Hasher::new(*algorithm);
        hashers.push(hasher);

    }
    let mut file = File::open(file_path)?;

    loop {
        let mut buf = [0; 1024 * 256]; // 每块大小
        let bytes_read = file.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        for hasher in &mut hashers {
            hasher.write(&buf[..bytes_read]);
        }
        // 在内循环中更新哈希值
    }
    for (i, hasher) in hashers.iter_mut().enumerate() {
        let result = hasher.finish();
        let (_, algorithm_name) = SUPPORTED_ALGORITHMS[i];
        println!("{}: {}", algorithm_name, hex::encode(result));
    }

    Ok(())
}