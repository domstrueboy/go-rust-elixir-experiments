extern crate chunked_transfer;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::fs::File;
use chunked_transfer::Encoder;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => println!("Unable to connect: {}", e),
        }
    }
}

fn get_path(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            let path: Vec<&str> = req_str.lines().next().unwrap().split(" ").collect();
            path[1].to_string()
        }
        Err(e) => {
            println!("Unable to read stream: {}", e);
            "/".to_string()
        }
    }
}

fn response(path: &str, mut stream: TcpStream) {
    println!("{}", path);
    let file_params: Vec<&str> = path.split(['/', '_', '.'].as_ref()).collect();

    let name = file_params[0];
    let width = file_params[1];
    let height = file_params[2];
    let height = file_params[2];
    
    // for s in file_params {
    //     println!("{}", s);
    // }
    
    let file_path = format!("img/input/{}", path);

    let mut buf = Vec::new();
    let mut file = File::open(&file_path).unwrap();
    
    file.read_to_end(&mut buf).unwrap();

    let mut encoded = Vec::new();
    {
        let mut encoder = Encoder::with_chunks_size(&mut encoded, 8);
        encoder.write_all(&buf).unwrap();
    }

    let headers = [
        "HTTP/1.1 200 OK",
        "Content-type: image/jpeg",
        "Transfer-Encoding: chunked",
        "\r\n"
    ];
    let mut response = headers.join("\r\n")
        .to_string()
        .into_bytes();
    response.extend(encoded);

    match stream.write(&response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_client(stream: TcpStream) {
    response(&get_path(&stream), stream);
}

// extern crate image;

// use image::GenericImageView;

// fn main() {
//     let img = image::open("img/input/progressive.jpg").unwrap();
//     println!("dimensions {:?}", img.dimensions());
//     println!("{:?}", img.color());
//     img
//     .resize(160, 90, image::imageops::Lanczos3)
//     .save("img/output/progressive.jpg")
//     .unwrap();
// }