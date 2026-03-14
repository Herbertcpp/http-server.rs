use std::{collections::HashMap, env::join_paths, fmt::format, fs, hash::Hash, io::{Read, Write}, net::{TcpListener, TcpStream}, sync::LazyLock};
use sqlite::{self, Connection, State};

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

fn handleClient(s : &mut TcpStream) {
    let mut buffer = [0u8; 1024];
    let bytes_read : usize = s.read(&mut buffer).unwrap();
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

    let contents = fs::read(file).expect("Error reading file");

    let ct = content_type(&file);

    let resp = format!(
    "HTTP/1.1 200 OK\r\n\
    Content-Type: {}\r\n\
    Content-Length: {}\r\n\r\n",
    ct, contents.len()
    );

    s.write(&resp.as_bytes());
    s.write(&contents).unwrap();
}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        handleClient(&mut stream);

    }

}
