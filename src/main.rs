use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> io::Result<()> {
    let db = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("P2P Key-Value DB running on 127.0.0.1:8080");

    for stream in listener.incoming() {
        let db = Arc::clone(&db);
        thread::spawn(move || {
            if let Ok(stream) = stream {
                handle_connection(stream, db);
            }
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<HashMap<String, String>>>) {
    let mut reader = io::BufReader::new(&stream);
    let mut buffer = String::new();

    loop {
        buffer.clear();
        if reader.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "GET" => {
                if parts.len() != 2 {
                    writeln!(stream, "Invalid GET command").unwrap();
                    continue;
                }
                let key = parts[1];
                let value = db.lock().unwrap().get(key).cloned();
                match value {
                    Some(v) => writeln!(stream, "{}", v).unwrap(),
                    None => writeln!(stream, "Key not found").unwrap(),
                }
            }
            "SET" => {
                if parts.len() != 3 {
                    writeln!(stream, "Invalid SET command").unwrap();
                    continue;
                }
                let key = parts[1].to_string();
                let value = parts[2].to_string();
                db.lock().unwrap().insert(key, value);
                writeln!(stream, "OK").unwrap();
            }
            "EXIT" => break,
            _ => writeln!(stream, "Unknown command").unwrap(),
        }
    }
}
