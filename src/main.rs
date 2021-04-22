use std::io::{stdin, stdout, Write};
use structopt::StructOpt;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use sus::config;

macro_rules! exit {
    ($code: literal) => {{
        println!("\r");
        std::process::exit($code)
    }};
}

macro_rules! log {
    ($message: ident) => {
        println!(
            "{msg}{reset}",
            msg = $message,
            reset = color::Fg(color::Reset)
        );
    };

    ($message: ident, $color: ident) => {
        println!(
            "{clr}{msg}{reset}",
            msg = $message,
            clr = color::Fg(color::$color),
            reset = color::Fg(color::Reset)
        )
    };
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sus")]
struct Cli {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(StructOpt, Debug)]
enum Sub {
    #[structopt(name = "install")]
    Install { package: String },
    #[structopt(name = "ping")]
    Ping,
}

const HELP: &str = "SUS - The Software Update Service";

fn main() {
    match Cli::from_args().cmd {
        Sub::Ping => {
            let addr = (*config::CONFIG).mirrors();
            let rs = sus::ping(addr);

            if rs {
                exit!(0);
            } else {
                exit!(1);
            }
        }
        Sub::Install { package: pkg } => {
            println!("{}", pkg);
        }
    }

    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();
    log!(HELP, Yellow);
    for key in stdin.keys() {
        let k = key.unwrap();
        if k != Key::Null {
            match k {
                Key::Char(c) => log!(c),
                _ => (),
            }
            match k {
                Key::Ctrl('c') | Key::Char('q') | Key::Esc => exit!(130),
                _ => (),
            }
        }
    }

    stdout.flush().unwrap();
}
