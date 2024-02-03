use termion::event::Key;

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }
    
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(1, 1);
        if self.should_quit {
            Terminal::clear_screen();
            println!("GOODBYE! \r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(1, 1);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    pub fn default() -> Self {
        Self { 
            should_quit : false,
            terminal : Terminal::default().expect("ERROR: Failed to initialize terminal"),
        }
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            _ => (),
        }
        Ok(())
    }
    
    fn draw_rows(&self) {
        let height = self.terminal.size().height - 1; 
        let width = self.terminal.size().width - 1; 
        for row in 0..height {
            Terminal::clear_current_line();
            if row == height / 3 {
                Terminal::cursor_position(row, width / 3); 
                println!("Edito -- the editor {}\r", VERSION);
            } else {
                println!(".\r");
            }
        }
    }
}

fn die(e: std::io::Error) {
    print!("{}", termion::clear::All);
    panic!("{}", e);
}

