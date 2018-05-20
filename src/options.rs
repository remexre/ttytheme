use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct Options {
    /// Turns off message output.
    #[structopt(short = "q", long = "quiet")]
    pub quiet: bool,

    /// The theme file.
    #[structopt(parse(from_os_str))]
    pub theme_path: PathBuf,

    /// Increases the verbosity. Default verbosity is errors only.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbose: usize,
}

impl Options {
    /// Sets up logging as specified by the `-q` and `-v` flags.
    pub fn start_logger(&self) {
        if !self.quiet {
            let r = ::stderrlog::new().verbosity(self.verbose).init();
            if let Err(err) = r {
                error!("Logging couldn't start: {}", err);
            }
        }
    }
}
