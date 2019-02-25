use std::net::{UdpSocket};
use std::io::{self, BufRead};
use std::env;
use std::thread;
use std::sync::Arc;
use std::str;



fn main() {
    let args: Vec<String> = env::args().collect();
    let origin = &args[1];
    let destination = &args[2];

    println!("starting chat between {} and {}", origin, destination);
    
    chat(&origin, destination);
}

fn chat(origin: &String, destination: &String) {
    let socket = Arc::new(UdpSocket::bind(origin).expect(&*format!("Unable to bind to {}", origin)));

    let socket_sender = socket.clone();

    thread::spawn(move || {
        loop {
            let mut buf = [0; 1024];
            let (_, src_addr) = socket_sender.recv_from(&mut buf).unwrap();
            let msg = str::from_utf8(&buf).unwrap();
            println!("{}: {}", src_addr, msg);
        }
    });

    let stdin = io::stdin();

    for line_res in stdin.lock().lines() {

        let line = line_res.unwrap();
        println!("sending {}", line);
        socket.send_to((&line[..]).as_bytes(), destination).unwrap();

    }
}
