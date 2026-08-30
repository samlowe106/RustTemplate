use clap::CommandFactory;
use clap_complete::{Shell, generate_to};
use std::env;
use std::io::Error;
use std::path::Path;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Ok(());
    };
    std::fs::create_dir_all(&outdir)?;

    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();

    for shell in [
        Shell::Bash,
        Shell::Zsh,
        Shell::Fish,
        Shell::PowerShell,
        Shell::Elvish,
    ] {
        generate_to(shell, &mut cmd, &name, &outdir)?;
    }

    let man = clap_mangen::Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;
    std::fs::write(Path::new(&outdir).join(format!("{name}.1")), buffer)?;

    Ok(())
}
