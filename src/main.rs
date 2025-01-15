// use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
// use crossterm::{execute, terminal::{Clear, ClearType}};
// use std::io::{self, Write};

// mod editor {
//     pub mod terminal; // Declare the terminal module within the editor module
// }

// use editor::terminal::Terminal;

// pub struct Editor {
//     should_quit: bool,
//     buffer: Vec<String>,
// }

// impl Editor {
//     // Default editor constructor
//     pub fn default() -> Self {
//         Self {
//             should_quit: false,
//             buffer: vec!["".to_string()], // Initial buffer with one empty line
//         }
//     }

//     // Run the editor with UI
//     pub fn run(&mut self) {
//         Terminal::initialize().unwrap();
//         let result = self.repl();
//         Terminal::terminate().unwrap();
//         result.unwrap();
//     }

//     // Read and process user input in a loop
//     fn repl(&mut self) -> Result<(), io::Error> {
//         loop {
//             self.refresh_screen()?;
//             if self.should_quit {
//                 break;
//             }
//             let event = read()?;
//             self.evaluate_event(&event);
//         }
//         Ok(())
//     }

//     // Refresh screen content
//     fn refresh_screen(&self) -> Result<(), io::Error> {
//         execute!(io::stdout(), Clear(ClearType::All))?; // Use execute! to clear the terminal
//         if self.should_quit {
//             Terminal::clear_screen()?;
//             print!("Goodbye.\r\n");
//         } else {
//             self.draw_ui()?;
//         }
//         Ok(())
//     }

//     // Draw the editor UI with rows and interactive options
//     fn draw_ui(&self) -> Result<(), io::Error> {
//         let height = Terminal::size()?.1 as usize;  // Terminal height
//         let content_height = self.buffer.len();

//         // Display the text content
//         for (current_row, line) in self.buffer.iter().enumerate() {
//             print!("{}{}", line, if current_row + 1 < height { "\r\n" } else { "" });
//         }

//         // Fill the remaining space with empty lines
//         for _ in content_height..height {
//             print!("~\r\n");
//         }

//         // Display instructions at the bottom
//         print!("\r\n[Ctrl + Q to quit] [Ctrl + S to save]");

//         // Move the cursor back to the top
//         Terminal::move_cursor_to(0, 0)?;
//         Ok(())
//     }

//     // Handle different types of events
//     fn evaluate_event(&mut self, event: &Event) {
//         match event {
//             Event::Key(KeyEvent {
//                 code: KeyCode::Char('q'),
//                 modifiers: KeyModifiers::CONTROL,
//                 ..
//             }) => {
//                 self.should_quit = true;
//             }
//             Event::Key(KeyEvent {
//                 code: KeyCode::Char(c),
//                 modifiers: KeyModifiers::CONTROL,
//                 ..
//             }) => {
//                 self.handle_control_key(c);
//             }
//             Event::Key(KeyEvent {
//                 code: KeyCode::Char(c),
//                 modifiers: _,
//                 ..
//             }) => {
//                 self.insert_char(c);
//             }
//             Event::Key(KeyEvent {
//                 code: KeyCode::Backspace,
//                 modifiers: _,
//                 ..
//             }) => {
//                 self.backspace();
//             }
//             _ => {}
//         }
//     }

//     // Handle control keys like 'Ctrl + S' for save
//     fn handle_control_key(&mut self, key: &char) {
//         match key {
//             's' => {
//                 self.save_file().unwrap();
//             }
//             _ => {}
//         }
//     }

//     // Insert a character into the last line
//     fn insert_char(&mut self, c: &char) {
//         if let Some(last_line) = self.buffer.last_mut() {
//             last_line.push(*c);
//         }
//     }

//     // Handle backspace
//     fn backspace(&mut self) {
//         if let Some(last_line) = self.buffer.last_mut() {
//             last_line.pop();
//         }
//     }

//     // Save the buffer content to a file
//     fn save_file(&self) -> Result<(), io::Error> {
//         let file_name = "output.txt"; // Hardcoded file name for simplicity
//         let mut file = std::fs::File::create(file_name)?;
//         for line in &self.buffer {
//             writeln!(file, "{}", line)?;
//         }
//         Ok(())
//     }
// }

// fn main() {
//     let mut editor = Editor::default();
//     editor.run();
// }
use druid::{
    widget::{Button, Flex, TextBox},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};
use std::fs::File;
use std::io::{self, Write};

#[derive(Clone, Data, Lens)]
pub struct Editor {
    buffer: String, // Editor text buffer as a single string
}

impl Editor {
    // Default editor constructor
    pub fn new() -> Self {
        Self {
            buffer: String::new(), // Initialize with an empty buffer
        }
    }

    // Save the buffer content to a file
    pub fn save_file(&self) -> Result<(), io::Error> {
        let file_name = "output.txt"; // Hardcoded file name for simplicity
        let mut file = File::create(file_name)?;
        writeln!(file, "{}", self.buffer)?;
        Ok(())
    }
}

pub fn build_ui() -> impl Widget<Editor> {
    let save_button = Button::new("Save").on_click(|_ctx, data: &mut Editor, _env| {
        data.save_file().unwrap();
    });

    let text_box = TextBox::new().with_text_size(16.0).lens(Editor::buffer);

    let ui = Flex::column()
        .with_child(text_box)
        .with_child(save_button)
        .padding(10.0);

    ui
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .title("Rustor - Custom Code Editor")
        .window_size((800.0, 600.0));

    let editor = Editor::new();

    AppLauncher::with_window(main_window)
        .launch(editor)
        .expect("Failed to launch the application");
}
