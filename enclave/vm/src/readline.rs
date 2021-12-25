use std::io;
use std::path::Path;
use std::string::String;
use std::vec::Vec;
use std::boxed::Box;
use std::vec;
use std::format;
use std::string::ToString;
use std::print;
type OtherError = Box<dyn std::error::Error>;
type OtherResult<T> = Result<T, OtherError>;

pub enum ReadlineResult {
    Line(String),
    EOF,
    Interrupt,
    IO(std::io::Error),
    EncodingError,
    Other(OtherError),
}

#[allow(unused)]
mod basic_readline {
    use super::*;

    pub trait Helper {}
    impl<T> Helper for T {}

    pub struct Readline<H: Helper> {
        helper: H,
    }

    impl<H: Helper> Readline<H> {
        pub fn new(helper: H) -> Self {
            Readline { helper }
        }

        pub fn load_history(&mut self, _path: &Path) -> OtherResult<()> {
            Ok(())
        }

        pub fn save_history(&mut self, _path: &Path) -> OtherResult<()> {
            Ok(())
        }

        pub fn add_history_entry(&mut self, _entry: &str) -> OtherResult<()> {
            Ok(())
        }

        pub fn readline(&mut self, prompt: &str) -> ReadlineResult {
            use std::io::prelude::*;
            print!("{}", prompt);
            if let Err(e) = io::stdout().flush() {
                return ReadlineResult::IO(e);
            }

            match io::stdin().lock().lines().next() {
                Some(Ok(line)) => ReadlineResult::Line(line),
                None => ReadlineResult::EOF,
                Some(Err(e)) => match e.kind() {
                    io::ErrorKind::Interrupted => ReadlineResult::Interrupt,
                    io::ErrorKind::InvalidData => ReadlineResult::EncodingError,
                    _ => ReadlineResult::IO(e),
                },
            }
        }
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// mod rustyline_readline {
//     use super::*;

//     pub trait Helper: rustyline::Helper {}
//     impl<T: rustyline::Helper> Helper for T {}

//     pub struct Readline<H: Helper> {
//         repl: rustyline::Editor<H>,
//     }

//     impl<H: Helper> Readline<H> {
//         pub fn new(helper: H) -> Self {
//             use rustyline::{At, Cmd, CompletionType, Config, Editor, KeyPress, Movement, Word};
//             let mut repl = Editor::with_config(
//                 Config::builder()
//                     .completion_type(CompletionType::List)
//                     .tab_stop(8)
//                     .build(),
//             );
//             repl.bind_sequence(
//                 KeyPress::ControlLeft,
//                 Cmd::Move(Movement::BackwardWord(1, Word::Vi)),
//             );
//             repl.bind_sequence(
//                 KeyPress::ControlRight,
//                 Cmd::Move(Movement::ForwardWord(1, At::AfterEnd, Word::Vi)),
//             );
//             repl.set_helper(Some(helper));
//             Readline { repl }
//         }

//         pub fn load_history(&mut self, path: &Path) -> OtherResult<()> {
//             self.repl.load_history(path)?;
//             Ok(())
//         }

//         pub fn save_history(&mut self, path: &Path) -> OtherResult<()> {
//             if !path.exists() {
//                 if let Some(parent) = path.parent() {
//                     std::fs::create_dir_all(parent)?;
//                 }
//             }
//             self.repl.save_history(path)?;
//             Ok(())
//         }

//         pub fn add_history_entry(&mut self, entry: &str) -> OtherResult<()> {
//             self.repl.add_history_entry(entry);
//             Ok(())
//         }

//         pub fn readline(&mut self, prompt: &str) -> ReadlineResult {
//             use rustyline::error::ReadlineError;
//             match self.repl.readline(prompt) {
//                 Ok(line) => ReadlineResult::Line(line),
//                 Err(ReadlineError::Interrupted) => ReadlineResult::Interrupt,
//                 Err(ReadlineError::Eof) => ReadlineResult::EOF,
//                 Err(ReadlineError::Io(e)) => ReadlineResult::IO(e),
//                 #[cfg(unix)]
//                 Err(ReadlineError::Utf8Error) => ReadlineResult::EncodingError,
//                 #[cfg(windows)]
//                 Err(ReadlineError::Decode(_)) => ReadlineResult::EncodingError,
//                 Err(e) => ReadlineResult::Other(e.into()),
//             }
//         }
//     }
// }

// #[cfg(target_arch = "wasm32")]
use basic_readline as readline_inner;
// #[cfg(not(target_arch = "wasm32"))]
// use rustyline_readline as readline_inner;

pub use readline_inner::Helper;

pub struct Readline<H: Helper>(readline_inner::Readline<H>);

impl<H: Helper> Readline<H> {
    pub fn new(helper: H) -> Self {
        Readline(readline_inner::Readline::new(helper))
    }
    pub fn load_history(&mut self, path: &Path) -> OtherResult<()> {
        self.0.load_history(path)
    }
    pub fn save_history(&mut self, path: &Path) -> OtherResult<()> {
        self.0.save_history(path)
    }
    pub fn add_history_entry(&mut self, entry: &str) -> OtherResult<()> {
        self.0.add_history_entry(entry)
    }
    pub fn readline(&mut self, prompt: &str) -> ReadlineResult {
        self.0.readline(prompt)
    }
}
