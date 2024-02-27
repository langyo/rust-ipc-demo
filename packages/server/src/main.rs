use anyhow::Context;
use interprocess::local_socket::{LocalSocketListener, LocalSocketStream, NameTypeSupport};
use std::io::{self, prelude::*, BufReader};

pub fn main() -> anyhow::Result<()> {
    fn handle_error(conn: io::Result<LocalSocketStream>) -> Option<LocalSocketStream> {
        match conn {
            Ok(c) => Some(c),
            Err(e) => {
                eprintln!("Incoming connection failed: {}", e);
                None
            }
        }
    }

    let name = {
        use NameTypeSupport::*;
        match NameTypeSupport::query() {
            OnlyPaths => "/tmp/example.sock",
            OnlyNamespaced | Both => "@example.sock",
        }
    };

    let listener = match LocalSocketListener::bind(name) {
        Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
            return Err(e.into());
        }
        x => x?,
    };

    println!("Server running at {}", name);

    let mut buffer = String::with_capacity(128);

    for conn in listener.incoming().filter_map(handle_error) {
        let mut conn = BufReader::new(conn);
        println!("Incoming connection!");

        conn.read_line(&mut buffer)
            .context("Socket receive failed")?;

        conn.get_mut().write_all(b"Hello from server!\n")?;

        print!("Client answered: {}", buffer);

        buffer.clear();

        conn.read_line(&mut buffer)
            .context("Socket receive failed")?;

        conn.get_mut().write_all(b"Hello from server2!\n")?;

        print!("Client answered: {}", buffer);

        buffer.clear();
    }
    Ok(())
}
