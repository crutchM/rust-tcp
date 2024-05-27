use std::vec;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use model::models;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("пиздец");
    
    println!("Socket opened on 8080 addr");

    for stream in listener.incoming(){
        match stream {
            Ok(stream ) => {
                std::thread::spawn(move || {
                    handle_client(stream);
            });
            }

            Err(err) => {
                println!("Error on accepting connection {}", err)
            }
        }
    }
}


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            if size == 0 {
                false
            } else {
                if let Ok(s) = std::str::from_utf8(&buffer[0..size]) {
                    println!("Получено от клиента: {}", s);

                }
                let respText = "пошел нахуй".to_string();

                
                let header = models::Header::new("Content-Length".to_string(),
                 respText.chars().count().to_string());

                let resp = models::Response{
                    protocol: "HTTP".to_string(),
                    version: "1.1".to_string(),
                    status: "200 OK".to_string(),
                    headers: vec![header],
                    body: models::Body { value: (respText) },
                };

                let response = resp.ToString();
                println!("response: {}", response);
                
                stream.write_all(response.as_bytes()).unwrap();
                true
            }
        }
        Err(_) => {
            eprintln!("Произошла ошибка при чтении/записи");
            false
        }
    } {}
}



mod model;