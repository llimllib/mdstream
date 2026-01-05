use mdstream::StreamingParser;
use std::io::{self, Read};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for --list-themes flag
    if args.len() > 1 && args[1] == "--list-themes" {
        println!("Available syntax highlighting themes:");
        for theme in StreamingParser::list_themes() {
            println!("  {}", theme);
        }
        return Ok(());
    }

    // Get theme from --theme parameter, environment variable, or use default
    let theme = if args.len() > 2 && args[1] == "--theme" {
        args[2].clone()
    } else {
        env::var("MDSTREAM_THEME").unwrap_or_else(|_| "base16-ocean.dark".to_string())
    };

    let mut parser = StreamingParser::with_theme(&theme);
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
