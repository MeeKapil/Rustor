use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io::{self, Write};

mod editor {
    pub mod terminal; // Declare the terminal module within the editor module
}

use editor::terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    buffer: Vec<String>,
}

impl Editor {
    // Use a normal function instead of const fn to initialize the buffer
    pub fn default() -> Self {
        Self {
            should_quit: false,
            buffer: vec!["".to_string()], // Initial buffer with one empty line
        }
    }

    // Run the editor
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    // Read and process user input in a loop
    fn repl(&mut self) -> Result<(), io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    // Refresh screen content
    fn refresh_screen(&self) -> Result<(), io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }
    fn draw_rows(&self) -> Result<(), io::Error> {
        let height = Terminal::size()?.1 as usize;  // Convert height to usize
        let content_height = self.buffer.len();
    
        // Draw each row from the buffer
        for (current_row, line) in self.buffer.iter().enumerate() {
            print!("{}{}", line, if current_row + 1 < height { "\r\n" } else { "" });
        }
    
        // Fill the remaining space with empty lines
        for _ in content_height..height {
            print!("~\r\n");
        }
        Ok(())
    }
    

    // // Draw the content of the buffer to the screen
    // fn draw_rows(&self) -> Result<(), io::Error> {
    //     let height = Terminal::size()?.1;
    //     let content_height = self.buffer.len();

    //     // Draw each row from the buffer
    //     for (current_row, line) in self.buffer.iter().enumerate() {
    //         print!("{}{}", line, if current_row + 1 < height { "\r\n" } else { "" });
    //     }

    //     // Fill the remaining space with empty lines
    //     for _ in content_height..height {
    //         print!("~\r\n");
    //     }
    //     Ok(())
    // }

    // Handle different types of events
    fn evaluate_event(&mut self, event: &Event) {
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                self.should_quit = true;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                self.handle_control_key(c);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: _,
                ..
            }) => {
                self.insert_char(c);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: _,
                ..
            }) => {
                self.backspace();
            }
            _ => {}
        }
    }

    // Handle control keys like 'Ctrl + S' for save
    fn handle_control_key(&mut self, key: &char) {
        match key {
            's' => {
                self.save_file().unwrap();
            }
            _ => {}
        }
    }

    // Insert a character into the last line
    fn insert_char(&mut self, c: &char) {
        if let Some(last_line) = self.buffer.last_mut() {
            last_line.push(*c);
        }
    }

    // Handle backspace
    fn backspace(&mut self) {
        if let Some(last_line) = self.buffer.last_mut() {
            last_line.pop();
        }
    }

    // Save the buffer content to a file
    fn save_file(&self) -> Result<(), io::Error> {
        let file_name = "output.txt"; // Add file name logic or user input here
        let mut file = std::fs::File::create(file_name)?;
        for line in &self.buffer {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
