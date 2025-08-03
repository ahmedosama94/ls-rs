use clap::Parser;
use colored::Colorize;
use std::{fs, os::unix::fs::PermissionsExt};

#[derive(Parser, Debug, Clone)]
pub struct LsArgs {
    #[arg(short = 'a', help = "do not ignore entries starting with .")]
    all: bool,

    #[arg(short = 'A', help = "do not list implied . and ..")]
    almost_all: bool,

    #[arg(short = 'l', help = "use a long listing format")]
    long_listing: bool,

    #[arg(value_name = "FILE", num_args = 1.., default_value = ".")]
    value: String,
}

impl LsArgs {
    pub fn exec(self) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries: Vec<_> = fs::read_dir(self.value)?
            .map(|entry| entry.expect("Failed to read DirEntry"))
            .collect();

        entries.sort_by(|a, b| {
            a.file_name()
                .into_string()
                .unwrap()
                .cmp(&b.file_name().into_string().unwrap())
        });

        let entries = entries
            .into_iter()
            .filter(|entry| {
                self.all
                    || self.almost_all
                    || !entry.file_name().into_string().unwrap().starts_with(".")
            })
            .map(|entry| {
                let entry_type = entry.file_type().expect("Failed to extract file type");
                let entry_name = entry
                    .file_name()
                    .into_string()
                    .expect("Failed to convert OsString to String");

                let entry_name = if entry_type.is_dir() {
                    entry_name.blue()
                } else {
                    entry_name.white()
                };

                if !self.long_listing {
                    entry_name
                } else {
                    let metadata = entry.metadata().expect("Failed to extract metadata");
                    let mode = metadata.permissions().mode();

                    let raw_type = mode & !0o777;
                    let mut output = String::new();

                    let ch = match raw_type {
                        0o100 => '-',
                        0o120 => 'l',
                        0o020 => 'c',
                        0o060 => 'b',
                        0o010 => 'p',
                        0o140 => 's',
                        0o040 => 'd',
                        _ => panic!("Unrecognized raw file type"),
                    };

                    output.push(ch);

                    let unix_permissions = mode & 0o777;

                    for el in [mode & 0o700 >> 2, mode & 0o070 >> 1, mode & 0o007 >> 1] {}

                    format!(
                        "{entry_name} {:#?}",
                        entry.metadata().unwrap().permissions()
                    )
                    .red()
                }
            });

        let entries = if self.all {
            vec![".".blue(), "..".blue()].into_iter().chain(entries)
        } else {
            vec![].into_iter().chain(entries)
        };

        for entry in entries {
            if true || !self.long_listing {
                print!("{}  ", entry);
            }
        }

        if true || !self.long_listing {
            println!();
        }

        Ok(())
    }
}
