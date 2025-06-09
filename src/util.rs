use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn run_command_detached(cmd: &str) -> Result<()> {
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "nohup sh -c 'exec {}' >/dev/null 2>&1 &",
            cmd.replace("'", "'\"'\"'")
        ))
        .spawn()
        .expect("Failed to start command")
        .wait()
        .expect("Failed to wait for intermediate process");
    Ok(())
}

fn get_config_dir() -> Result<PathBuf> {
    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        Ok(PathBuf::from(xdg_config_home))
    } else if let Ok(home) = env::var("HOME") {
        Ok(PathBuf::from(home).join(".config"))
    } else {
        Err(eyre!("Unable to find Home"))
    }
}

pub fn load_keybindings(file: Option<String>) -> Result<String> {
    let file_path = match file {
        Some(x) => PathBuf::from(x),
        None => get_config_dir()?.join("modali").join("bindings.json"),
    };
    Ok(fs::read_to_string(file_path)?)
}
