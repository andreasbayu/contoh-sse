use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration, fs,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8989").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";


    if path(&http_request, "events") {

        let response = format!("{status_line}\r\nContent-Type: text/event-stream\r\nCache-Control: no-cache\r\nConnection: keep-alive\r\nAccess-Control-Allow-Origin: *\r\n\r\n");
    
        stream.write_all(response.as_bytes()).unwrap();
    
        stream.flush().unwrap();

        for i in 1..=10 {
            let content = format!("data: Message {} \n\n", i);
            let length = content.len();
            let response = format!("Content-Length: {length}\r\n{content}\r\n");

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();

            thread::sleep(Duration::from_secs(3));
        }
    } else if path(&http_request, "home") {
        let contents = fs::read_to_string("index.html").unwrap();
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}");
    
        stream.write_all(response.as_bytes()).unwrap();

    }
}

fn path(context: &Vec<String>, endpoint: &str) -> bool {
    context.iter().any(|line| line.contains(&format!("/{}", endpoint)))
}