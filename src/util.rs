use crate::parser::{self, Action, TypedAction};
use crate::whichtree::WhichTreeNode;
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

enum Filetype {
    Json(String),
    Ron(String),
}

fn load_config_file(input: Option<String>, name: &str) -> Result<Filetype> {
    let file_path = match input {
        Some(x) => {
            let path = PathBuf::from(&x);

            if path.exists() {
                Ok(path)
            } else {
                Err(eyre!("Input file {x} does not exists"))
            }
        }
        None => {
            let config_dir = get_config_dir()?.join("modali");
            let json_dir = config_dir.join(format!("{name}.json"));

            if json_dir.exists() {
                Ok(json_dir)
            } else {
                let ron_dir = config_dir.join(format!("{name}.ron"));
                if ron_dir.exists() {
                    Ok(ron_dir)
                } else {
                    Err(eyre!("Unable to find {name} file"))
                }
            }
        }
    }?;

    let contents = fs::read_to_string(&file_path)?;
    let ext = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| eyre!("File has no valid extension"))?;

    match ext {
        "json" => Ok(Filetype::Json(contents)),
        "ron" => Ok(Filetype::Ron(contents)),
        other => Err(eyre!("File type {} is not supported", other)),
    }
}

pub fn load_keybindings(input: Option<String>) -> Result<WhichTreeNode> {
    let file = load_config_file(input, "bindings")?;

    let actions: Vec<Action> = match file {
        Filetype::Json(x) => serde_json::from_str::<Vec<TypedAction>>(&x)?
            .into_iter()
            .map(Into::into)
            .collect(),
        Filetype::Ron(x) => ron::from_str(&x)?,
    };

    Ok(parser::actions_to_tree(&actions))
}
