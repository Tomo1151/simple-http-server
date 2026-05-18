use std::{io::{self, Read}, net::{SocketAddr, TcpListener, TcpStream}};

fn main() {
    const PORT: u16 = 1151;
    let listener = match bind_available_port(PORT) {
        Ok(listener) => listener,
        Err(_) => panic!("Failed to create TCP listener."),
    };

    // ストリームからメッセージを表示
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection_stream(stream);
    }
}

// TCPストリームのハンドリング関数
fn handle_connection_stream(mut stream: TcpStream) {
    // バッファを確保
    let mut buffer = [0; 1024];

    // ストリームから読み取ったデータをバッファへ読み込み
    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
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

