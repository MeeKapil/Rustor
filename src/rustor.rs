use log::{info, error};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // Initialize the logger
    env_logger::init();

    // Step 2: Setting up the Terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; // Enter alternate screen mode
    terminal::enable_raw_mode()?; // Enable raw mode to capture input immediately

    // Step 3: Initializing Variables
    let mut buffer = String::new(); // Text buffer for the editor
    let mut cursor_pos = (0, 0);    // Cursor position (x, y)

    // Step 4: Asking for Filename (User Input)
    println!("Enter the filename to open or create:");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    let filename = filename.trim();

    // Debug print to show the filename entered
    println!("You entered: {}", filename);

    // Step 5: Load File Contents (If Exists)
    if Path::new(filename).exists() {
        buffer = fs::read_to_string(filename)?;
    }

    // Step 6: Start the Main Editing Loop
    loop {
        // Clear the screen and render the buffer
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
        for (i, line) in buffer.lines().enumerate() {
            println!("{:4}: {}", i + 1, line);
        }

        // Move cursor to the last position after printing the buffer
        execute!(stdout, cursor::MoveTo(cursor_pos.0 as u16, cursor_pos.1 as u16))?;
        stdout.flush()?; // Flush the output to ensure it appears immediately

        // Step 7: Handle User Input (Key Presses)
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                // Log the key press
                info!("Key pressed: {:?}", code);

                match code {
                    KeyCode::Char(c) => {
                        // Add a character to the buffer
                        buffer.push(c);
                        cursor_pos.0 += 1; // Move cursor position to the right
                    }
                    KeyCode::Enter => {
                        // Add a new line
                        buffer.push('\n');
                        cursor_pos = (0, cursor_pos.1 + 1); // Move to the next line
                    }
                    KeyCode::Backspace => {
                        // Remove the last character
                        if !buffer.is_empty() {
                            buffer.pop();
                            if cursor_pos.0 > 0 {
                                cursor_pos.0 -= 1; // Move cursor left
                            } else if cursor_pos.1 > 0 {
                                cursor_pos.1 -= 1; // Move cursor to the previous line
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Exit the editor when Esc key is pressed
                        println!("\nESC pressed! Exiting...");
                        break;
                    }
                    KeyCode::F(2) => {
                        // Save the file
                        let mut file = File::create(filename)?;
                        file.write_all(buffer.as_bytes())?;
                        println!("\nFile saved!");
                    }
                    _ => {}
                }
            }
        }
    }

    // Step 8: Clean up the Terminal on Exit
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?; // Exit alternate screen mode
    terminal::disable_raw_mode()?; // Disable raw mode
    Ok(())
}
