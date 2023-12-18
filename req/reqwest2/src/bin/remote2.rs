use ssh2::Session;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::path::Path;
use std::fs::File;

fn main() {
    let local_path = "D:/projects/rust/eagle_rust/Cargo.lock";
    let remote_path = "/opt/Cargo.lll";
    let host = "192.168.0.131";
    let port = 22;
    let username = "root";
    let password = "1";

    // 建立 TCP 连接
    let tcp = TcpStream::connect((host, port)).unwrap();
    let mut session = Session::new().unwrap();
    // 设置 SSH 会话使用的 TCP 连接
    session.set_tcp_stream(tcp);
    // 执行 SSH 握手
    session.handshake().unwrap();
    // 进行密码认证
    session.userauth_password(username, password).unwrap();

    // 开始使用 SFTP 会话
    let sftp = session.sftp().unwrap();

    // 打开本地文件进行读取
    let mut file = File::open(&local_path).unwrap();
    let file_size = file.metadata().unwrap().len();
    let mut reader = BufReader::new(file);
    let remote_path = Path::new(remote_path);
    // 创建远程文件
    let mut remote_file = sftp.create(&remote_path).unwrap();

    // 将本地文件的内容写入远程文件
    let mut buffer = vec![0; 4096];
    let mut written = 0;
    while written < file_size {
        let len = reader.read(&mut buffer).unwrap();
        if len == 0 {
            // 文件读取完毕
            break;
        }

        // 写入数据到远程文件
        remote_file.write_all(&buffer[..len]).unwrap();
        written += len as u64;
    }

    // 数据传输完成后，我们可以关闭 SFTP 会话
    println!("File transfer complete!");
}