use clap::Parser;
use colored::{ColoredString, Colorize};
use std::{fmt, fs};

#[derive(Parser, Debug, Clone)]
pub struct LsArgs {
    #[arg(value_name = "FILE", num_args = 1.., default_value = ".")]
    value: String,
}

impl LsArgs {
    pub fn exec(self) -> Result<LsOutput, Box<dyn std::error::Error>> {
        let args = self.clone();
        let mut paths: Vec<_> = fs::read_dir(self.value)?
            .map(|entry| entry.expect("Failed to read DirEntry"))
            .collect();

        paths.sort_by(|a, b| {
            a.file_name()
                .into_string()
                .unwrap()
                .cmp(&b.file_name().into_string().unwrap())
        });

        let output = paths
            .iter()
            .filter(|entry| !entry.file_name().into_string().unwrap().starts_with("."))
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
