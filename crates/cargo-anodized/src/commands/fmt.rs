use anodized_fmt::{Config, check_file, format_file};
use std::{
    collections::BTreeSet as Set,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

type Result<T> = std::result::Result<T, Error>;

pub fn fmt(
    check: bool,
    packages: &[String],
    manifest_path: Option<&Path>,
    all: bool,
) -> Result<()> {
    let mut command = Command::new("cargo");
    command.arg("fmt");

    for package in packages {
        command.args(["--package", package]);
    }
    if let Some(manifest_path) = manifest_path {
        command.args(["--manifest-path"]);
        command.arg(manifest_path);
    }
    if all {
        command.arg("--all");
    }
    if check {
        command.arg("--check");
    }

    let output = command.args(["--", "--verbose"]).output()?;

    let paths = formatted_paths(&output.stdout)?;
    for path in &paths {
        println!("{}", path.display());
    }

    if !output.status.success() {
        return Err(Error::CargoFmtFailed(output.status));
    }

    let config = Config::load()?;
    if check {
        let mut all_formatted = true;
        for path in paths {
            let source = fs::read_to_string(path)?;
            all_formatted &= check_file(&source, &config)?;
        }

        if !all_formatted {
            return Err(Error::CheckFailed);
        }
    } else {
        for path in paths {
            let source = fs::read_to_string(&path)?;
            let formatted = format_file(&source, &config)?;

            if formatted != source {
                fs::write(path, formatted)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("`cargo fmt` output was not valid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("`cargo fmt` failed with status {0}")]
    CargoFmtFailed(ExitStatus),

    #[error("some files need formatting with `anodized-fmt`")]
    CheckFailed,

    #[error(transparent)]
    Config(#[from] anodized_fmt::ConfigError),

    #[error(transparent)]
    Format(#[from] anodized_fmt::FormatError),
}

fn formatted_paths(output: &[u8]) -> Result<Set<PathBuf>> {
    let output = std::str::from_utf8(output)?;
    output
        .lines()
        .filter_map(|line| line.strip_prefix("Formatting "))
        .map(PathBuf::from)
        .map(|path| path.canonicalize().map_err(Error::from))
        .collect()
}
