use std::fs::OpenOptions;
use std::io::Write;

use failure::{Error, ResultExt};

use theme::Theme;

macro_rules! write_channel {
    ($theme:expr, $n:tt, $path:expr) => {
        (|| -> Result<(), Error> {
            let mut buf = Vec::new();
            let mut first = true;
            for color in $theme.iter() {
                if first {
                    first = false;
                } else {
                    write!(buf, ",")?;
                }
                write!(buf, "{}", color.$n)?;
            }

            let mut f = OpenOptions::new()
                .read(true)
                .write(true)
                .append(false)
                .truncate(true)
                .create(false)
                .create_new(false)
                .open($path)?;
            f.write_all(&buf)?;
            f.flush()?;
            Ok(())
        })()
    };
}

/// Applies a theme.
pub fn apply_theme(theme: &Theme) -> Result<(), Error> {
    write_channel!(theme, 0, "/sys/module/vt/parameters/default_red")
        .context("Couldn't set red channel")?;
    write_channel!(theme, 1, "/sys/module/vt/parameters/default_grn")
        .context("Couldn't set green channel")?;
    write_channel!(theme, 2, "/sys/module/vt/parameters/default_blu")
        .context("Couldn't set blue channel")?;
    Ok(())
}
