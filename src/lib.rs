use clap::Parser;
use colored::{ColoredString, Colorize};
use std::{fmt, fs};

#[derive(Parser, Debug, Clone)]
pub struct LsArgs {
    #[arg(short = 'a', help = "do not ignore entries starting with .")]
    all: bool,

    #[arg(short = 'A', help = "do not list implied . and ..")]
    almost_all: bool,

    #[arg(value_name = "FILE", num_args = 1.., default_value = ".")]
    value: String,
}

impl LsArgs {
    pub fn exec(self) -> Result<LsOutput, Box<dyn std::error::Error>> {
        let mut paths: Vec<_> = fs::read_dir(self.value)?
            .map(|entry| entry.expect("Failed to read DirEntry"))
            .collect();

        paths.sort_by(|a, b| {
            a.file_name()
                .into_string()
                .unwrap()
                .cmp(&b.file_name().into_string().unwrap())
        });

        let mut output = paths
            .iter()
            .filter(|entry| {
                self.all
                    || self.almost_all
                    || !entry.file_name().into_string().unwrap().starts_with(".")
            })
            .map(|entry| {
                let path = entry.file_name().into_string().unwrap();

                let file_type = match entry.file_type() {
                    Ok(val) => val,
                    Err(err) => panic!("Failed to check entry type for {}!\n{:#?}", path, err),
                };

                if file_type.is_dir() {
                    path.blue()
                } else {
                    path.into()
                }
            })
            .collect();

        let output = if self.all {
            let mut new_output = vec![".".blue(), "..".blue()];
            new_output.append(&mut output);

            new_output
        } else {
            output
        };

        Ok(LsOutput { entries: output })
    }
}

pub struct LsOutput {
    entries: Vec<ColoredString>,
}

impl fmt::Display for LsOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for colored_string in &self.entries {
            write!(f, "{}  ", colored_string)?;
        }

        Ok(())
    }
}
