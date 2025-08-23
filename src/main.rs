use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Getconfig {},
}

#[derive(Serialize, Deserialize)]
struct Config {
    tool: Vec<ToolConfig>,
}

#[derive(Serialize, Deserialize)]
struct ToolConfig {
    name: String,
    exec: String,
}

fn get_home_dir() -> String {
    return match env::var("HOME") {
        Ok(val) => val,
        Err(_) => "/".to_string(),
    };
}

fn get_config_dir() -> String {
    return match env::var("XDG_CONFIG_HOME") {
        Ok(val) => val,
        Err(_) => match Path::new(&get_home_dir()).join(".config").to_str() {
            Some(p) => p.to_string(),
            None => "/".to_string(),
        },
    };
}

fn get_config_path() -> String {
    return match Path::new(&get_config_dir())
        .join("custom-completion-zsh")
        .join("config.toml")
        .to_str()
    {
        Some(p) => p.to_string(),
        None => "".to_string(),
    };
}

fn get_config() -> Config {
    let content: String = fs::read_to_string(get_config_path()).unwrap();
    let res: Config = toml::from_str(&content).unwrap();

    return res;
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommands::Getconfig {} => {
            println!("{}", get_config_path());
            let config = get_config();

            for i in 0..config.tool.len() {
                println!(
                    "name: {} exec: {}",
                    config.tool[i].name, config.tool[i].exec
                );
            }
        }
    }
}
