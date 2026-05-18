# Rust製 簡易HTTPサーバの実装

## TCP接続をリッスンする
`std::net::TcpListener` を使用することでTCP接続をリッスンすることができる．
`TcpListener::bind(addr)` でリスナーを取得できる．

## Rustについて
### Result<T, E> / unwrap()
正常な時にTの値，失敗時にEを返す．
?演算子でエラーを上位の関数に伝播させることができる．
これはmatch式で値を取り出し，失敗時に早期returnする書き方のシンタックスシュガーである．

```rust
// 値の受け取り方 (?演算子)
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // 正常終了時の値を返す時には Ok() を使用する
    Ok(content)
}

// 値の受け取り方 (match式)
let result = match read_file("testfile.txt") {
    Ok(content) => println!(content),
    Err(e) => println!("Error: {:?}", e),
};

// 値の受け取り方 (if let 式)
let result: Result<String, std::io::Error> = read_file("testfile.txt");
if let Ok(result) = result {
    println!(result)
}
```

このエラーハンドリングを省略したいときには，unwrap() を使用することができる．
unwrap() を使用すると，正常時はTの値を返し，失敗時はパニックする．

```rust
let result = read_file("testfile.txt").unwrap();
```

unwrap_or() を使用すると失敗時の代替値を指定でき，パニックしない．
