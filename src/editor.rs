use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    // Add any necessary fields here
}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    print!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(err) => println!("Error: {err}"),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
