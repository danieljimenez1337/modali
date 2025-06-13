use color_eyre::eyre::{Context, Result};
use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};

use clap::Parser;

mod gui;
mod input;
mod parser;
mod util;
mod whichtree;

#[derive(Parser, Debug, Default)]
#[command(name = "Modali", about = "Which Like Launcher")]
pub struct Args {
    /// Input keybindings file. If you don't want to use default location.
    #[arg(short, long)]
    pub input: Option<String>,

    /// Input Style file. If you don't want to use default location.
    #[arg(short, long)]
    pub style: Option<String>,

    /// Check Bindings and Style File for Errors
    #[arg(short, long)]
    pub validate: bool,
}

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    gui::Modali::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((1200, 400)),
            exclusive_zone: 0,
            anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode,
            layer: Layer::Overlay,
            ..Default::default()
        },
        flags: args,
        ..Default::default()
    })
    .wrap_err("Application Error")
}
