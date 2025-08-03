use clap::Parser;
use ls::LsArgs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = LsArgs::parse();

    args.exec()?;

    Ok(())
}
