// editor.rs

use std::io::{self, Write, Read};
use crossterm::{terminal::{self, enable_raw_mode, disable_raw_mode}, cursor::{MoveTo, MoveDown}, ExecutableCommand};

pub fn run_editor() {
    enable_raw_mode().unwrap();
    let mut buffer = String::new();
    let mut line_count = 0;

    loop {
        // Clear the screen and move the cursor to the top
        io::stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
        
        // Print the current text buffer, ensuring to respect line breaks
        io::stdout().execute(MoveTo(0, 0)).unwrap();
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
                    } else if c == '\n' {  // Handle "Enter" key
                        buffer.push('\n');  // Add a newline to the buffer
                        line_count += 1;  // Increment the line counter
                    } else {
                        buffer.push(c);
                    }

                    // Move cursor down when a new line is added
                    if line_count > 0 {
                        io::stdout().execute(MoveDown(1)).unwrap();
                        line_count = 0;  // Reset line counter after moving down
                    }

                    // Immediately update the display after each character
                    io::stdout().execute(MoveTo(0, line_count as u16)).unwrap();
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
