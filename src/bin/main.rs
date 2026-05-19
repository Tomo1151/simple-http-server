extern crate simple_http_server;

// use std::{fs::File, io::{self, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;

use simple_http_server::ThreadPool;


fn main() {
    const THREAD_MAX: usize = 16;
    const PORT: u16 = 1151;

    let listener = match bind_available_port(PORT) {
        Ok(listener) => listener,
        Err(_) => panic!("Failed to create TCP listener."),
    };

    let pool = ThreadPool::new(THREAD_MAX);

    // ストリームからメッセージを表示
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection_stream(stream);
        });
    }

    println!("Server shutting down.");
}

// TCPストリームのハンドリング関数
fn handle_connection_stream(mut stream: TcpStream) {
    // バッファを確保
    let mut buffer = [0; 1024];

    // ストリームから読み取ったデータをバッファへ読み込み
    stream.read(&mut buffer).unwrap();

    // u8 を安全にUTF-8に変換して表示
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // テンプレートHTMLを取得
    let mut file = File::open("templates/index.html").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // レスポンスをストリームへ書き込み
    let response_status = "HTTP/1.1 200 OK";
    let response_header = "";
    let response_body   = content;

    stream.write(
        format!(
            "{}\r\n{}\r\n{}",
            response_status,
            response_header,
            response_body,
        ).as_bytes()
    ).unwrap();
    stream.flush().unwrap();
}

// 使用できるポートでTCPリスナーを作成する関数
fn bind_available_port(initial_port: u16) -> Result<TcpListener, io::Error> {
    const MAX_TRIES: u16 = 10;

    // 使用できるポートが見つかるまで試行
    for i in 0..MAX_TRIES {
        let port = initial_port + i;
        let address = SocketAddr::from(([127, 0, 0, 1], port));
        
        match TcpListener::bind(address) {
            Ok(listener) => {
                println!("🎉 TCP connection listening on port {} !", port);
                return Ok(listener);
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::AddrInUse {
                    continue;
                }
                return Err(e);
            }
        }
    }

    // 全てのポートが使用中の場合はエラーを返す
    Err(io::Error::new(
        io::ErrorKind::AddrInUse,
        "Failed to bind TCP listener because of port in use."
    ))
}

