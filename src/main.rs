use mdstream::StreamingParser;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut parser = StreamingParser::new();
    let mut buffer = [0u8; 4096];
    let mut stdin = io::stdin();

    // Read and process stdin in chunks
    loop {
        let bytes_read = stdin.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // EOF
        }

        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        let output = parser.feed(&chunk);
        print!("{}", output);
    }

    // Flush any remaining buffered content
    let output = parser.flush();
    print!("{}", output);

    Ok(())
}
