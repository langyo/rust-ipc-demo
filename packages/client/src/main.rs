use anyhow::Context;
use interprocess::local_socket::{LocalSocketStream, NameTypeSupport};
use std::io::{prelude::*, BufReader};

pub fn main() -> anyhow::Result<()> {
    let name = {
        use NameTypeSupport::*;
        match NameTypeSupport::query() {
            OnlyPaths => "/tmp/example.sock",
            OnlyNamespaced | Both => "@example.sock",
        }
    };

    let mut buffer = String::with_capacity(128);

    let conn = LocalSocketStream::connect(name).context("Failed to connect to server")?;
    let mut conn = BufReader::new(conn);

    conn.get_mut()
        .write_all(b"Hello from client!\n")
        .context("Socket send failed")?;

    conn.read_line(&mut buffer)
        .context("Socket receive failed")?;

    print!("Server answered: {}", buffer);

    buffer.clear();

    conn.get_mut()
        .write_all(b"Hello from client2!\n")
        .context("Socket send failed")?;

    conn.read_line(&mut buffer)
        .context("Socket receive failed")?;

    print!("Server answered: {}", buffer);

    Ok(())
}
