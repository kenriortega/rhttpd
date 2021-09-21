use std::{
    fmt::format,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    // iniciar el servidor
    let addr = "localhost:5000";
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Server running on {}", &addr);
    // escuchar por conexiones
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client! from {:?}", stream.peer_addr());
                handle_conn(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
// manejar las conexiones
fn handle_conn(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).unwrap();
    println!("{}", String::from_utf8_lossy(&buf[..]));

    // reponder al cliente
    let get = b"GET / HTTP/1.1";
    if buf.starts_with(get) {
        send_index(stream);
    } else {
        send_not_found(stream);
    }
}

fn send_index(mut stream: TcpStream) {
    let contents = fs::read_to_string("index.html").unwrap();
    // Carriege return CR \r
    // Line feed LF \n
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
fn send_not_found(mut stream: TcpStream) {
    let contents = fs::read_to_string("404.html").unwrap();
    // Carriege return CR \r
    // Line feed LF \n
    let response = format!(
        "HTTP/1.1 404 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
