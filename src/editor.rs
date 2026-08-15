mod editorcommand;
mod statusbar;
mod terminal;
mod view;

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};

use editorcommand::EditorCommand;
use statusbar::StatusBar;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
use view::View;

#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    total_lines: usize,
    current_line_index: usize,
    is_modified: bool,
    file_name: Option<String>,
}

pub struct Editor {
    should_quit: bool,
    view: View,
    status_bar: StatusBar,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        // If the program crashes (panics), restore the terminal before printing the panic message.
        // Without this, the terminal would probably stay in raw mode, leaving it looking broken after the program exits.
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;
        let mut view = View::new(2);
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }

        Ok(Self {
            should_quit: false,
            view,
            status_bar: StatusBar::new(1),
        })
    }

    pub fn run(&mut self) {
        loop {
            let status = self.view.get_status();
            self.status_bar.update_status(status);

            self.refresh_screen();
            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(_err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Cound not read event: {_err:?}");
                    }
                }
            }
        }
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match event {
            Event::Key(KeyEvent { kind, .. }) => kind == KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                        if let EditorCommand::Resize(size) = command {
                            self.status_bar.resize(size);
                        }
                    }
                }
                // don't crash
                Err(_) => {}
            }
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Received and discarded unsupported or non-press event.");
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        // We're basically ignoring any error here, even on debug. None of these steps is even noteworthy (well, maybe execute), and would at most result in a caret briefly not being visible or something similar.

        self.view.render();
        self.status_bar.render();
        // let _ = to ignore the Result that must be used.
        let _ = Terminal::move_caret_to(self.view.caret_position());

        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        // We ignore all error cases here to not cause a double panic. If Editor drops we don't need the terminal any more anyways.
        let _ = Terminal::print("Goodbye. \r\n");
    }
}
