use std::{collections::HashMap, env::join_paths, fmt::format, fs, hash::Hash, sync::LazyLock};
use sqlite::{self, Connection, State};
use tokio::{self, io::AsyncReadExt, io::AsyncWriteExt, net::TcpStream, net::TcpListener};

static file_paths : LazyLock<HashMap<&'static str, &'static str>> =
 LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("/", "index.html");
    map.insert("/favicon.ico", "favicon.ico");
    map.insert("/style.css", "style.css");
    map.insert("/script.js", "script.js");
    map
});

fn content_type(s : &str) -> &str {
    if s.ends_with(".html") {
        "text/html"
    } else if s.ends_with(".css") {
        "text/css" 
    } else if s.ends_with(".js") {
       "application/js" 
    } 
    else if s.ends_with(".favicon.ico") {
        "image/x-icon"
    }
    else {
        "application/octet-stream"
    }
}

async fn handleClient(s : &mut TcpStream) {
    let mut buffer = [0u8; 1024];
    let bytes_read : usize = s.read(&mut buffer).await.unwrap();
    let req = String::from_utf8_lossy(&buffer[..bytes_read]);
    let split_req : Vec<&str> = req.split(' ').collect();
    let method = split_req.get(0).unwrap();
    let path = split_req.get(1).unwrap();


    let file = match file_paths.get(path) {
        Some(e) => *e,
        None => {
            println!("Couldn't find file {}", path);
            return;
        }
    };

    let contents = tokio::fs::read(file).await.expect("Error reading file");

    let ct = content_type(&file);

    let resp = format!(
    "HTTP/1.1 200 OK\r\n\
    Content-Type: {}\r\n\
    Content-Length: {}\r\n\r\n",
    ct, contents.len()
    );

    let _ = s.write(&resp.as_bytes()).await.unwrap();
    let _ = s.write(&contents).await.unwrap();
}

#[tokio::main]
async fn main() {

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {

    let (mut s, addr) = listener.accept().await.unwrap();

        handleClient(&mut s).await;

        println!("Handling client");

    }
}