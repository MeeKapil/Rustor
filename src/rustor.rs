use std::io::{self, Write, Read};
use crossterm::{terminal::{self, enable_raw_mode, disable_raw_mode}, cursor::MoveTo, ExecutableCommand};

fn main() {
    enable_raw_mode().unwrap();
    let mut buffer = String::new();

    loop {
        // Clear the screen and move the cursor to the top
        io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        io::stdout().execute(MoveTo(0, 0)).unwrap();

        // Display the current text buffer immediately
        print!("{}", buffer);
        io::stdout().flush().unwrap();

        // Read the next byte from stdin
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;

                    // Handle 'q' immediately to break the loop
                    if c == 'q' {
                        disable_raw_mode().unwrap();  // Disable raw mode before returning
                        return;  // Exits the program
                    } else if c == '\x08' {  // Handle backspace (Ctrl + H)
                        buffer.pop();
                    } else {
                        buffer.push(c);
                    }

                    // Immediately update the display after each character
                    io::stdout().execute(MoveTo(0, 0)).unwrap();
                    print!("{}", buffer);
                    io::stdout().flush().unwrap();
                }
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            }
        }
    }
}