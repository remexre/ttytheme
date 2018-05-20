//! A utility to set colors in the Linux framebuffer console.

extern crate failure;
#[cfg(not(debug_assertions))]
#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate stderrlog;
#[macro_use]
extern crate structopt;
extern crate toml;

mod color;
mod options;
mod platform;
mod theme;

use std::fs::File;
use std::io::Read;
use std::process::exit;

use failure::Error;
use structopt::StructOpt;
use toml::from_str;

use options::Options;
use platform::apply_theme;

fn main() {
    let options = Options::from_args();
    setup_panic();
    options.start_logger();

    if let Err(err) = run(options) {
        let mut first = true;
        let num_errs = err.causes().count();
        if num_errs <= 1 {
            error!("{}", err);
        } else {
            for cause in err.causes() {
                if first {
                    first = false;
                    error!("           {}", cause);
                } else {
                    error!("caused by: {}", cause);
                }
            }
        }
        debug!("{}", err.backtrace());
        exit(1);
    }
}

fn run(options: Options) -> Result<(), Error> {
    let mut f = File::open(&options.theme_path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let theme = from_str(&s)?;
    apply_theme(&theme)
}

#[cfg(debug_assertions)]
fn setup_panic() {}

#[cfg(not(debug_assertions))]
fn setup_panic() {
    setup_panic!();
}
