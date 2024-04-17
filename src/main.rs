use std::io::{BufRead, Write};

const PORT: &str = "127.0.0.1:9998";


fn do_server() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind(PORT)?;
    println!("listening on http://{}", PORT);

    for mut stream in listener.incoming().flatten() {
        let addr = stream.peer_addr()?;
        let mut reader = std::io::BufReader::new(&mut stream);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        match line.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                println!("request from {} for {}", addr, resource);
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line)?;
                    if line.trim().is_empty() {
                        break;
                    }
                }
                let mut path = std::path::PathBuf::new();
                path.push("static");
                path.push(resource.trim_start_matches('/'));
                if resource.ends_with('/') {
                    path.push("index.html");
                }
                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
                stream.write_all(&std::fs::read(path)?)?;  // TODO: not secure
            }
            _ => todo!()
        }
    }

    return Ok(());
}


fn main() {
    if let Err(error) = do_server() {
        println!("Error: {}", error);
    }
}
