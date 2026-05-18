use std::{io::Read, net::{TcpListener, TcpStream}};

fn main() {
    // リッスンするアドレスの定義
    let address = "127.0.0.1";
    let port = "1151";

    // TCPリスナーを作成し，指定アドレスをリッスン
    let listener = TcpListener::bind(format!("{}:{}", address, port)).unwrap();

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
