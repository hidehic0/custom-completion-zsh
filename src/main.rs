use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs, process};
use tokio::process::Command;
use tokio::sync::mpsc;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    arg_required_else_help = true,
    about="A tool for zsh that automatically sets completion commands set by the user 
Linux only"
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    #[clap(about = "Get config")]
    Getconfig {},
    #[clap(about = "Generate the autocompletion script for the specified shell")]
    Build {},
}

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    tool: Vec<ToolConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
struct ToolConfig {
    name: String,
    exec: String,
}

fn get_home_dir() -> String {
    return env::var("HOME").unwrap_or_else(|_| "/".to_string());
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

fn get_compfile_dir() -> String {
    // Err(_) => match Path::new(&get_home_dir())
    //     .join(".local")
    //     .join("share")
    //     .to_str()
    // {
    //     Some(p) => p.to_string(),
    //     None => "".to_string(),
    // },
    let xdg_data_home: String = env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
        match Path::new(&get_home_dir())
            .join(".local")
            .join("share")
            .to_str()
        {
            Some(p) => p.to_string(),
            None => "".to_string(),
        }
    });

    return match Path::new(&xdg_data_home)
        .join("zsh")
        .join("custom-completion-zsh")
        .to_str()
    {
        Some(p) => p.to_string(),
        None => "".to_string(),
    };
}

fn crean_dir(dir: String) -> Result<usize, &'static str> {
    match fs::remove_dir_all(&dir) {
        Ok(_) => 0,
        Err(_) => -1,
    };

    // if delete_status == -1 {
    //     return Err("Could not remove directory");
    // }

    fs::create_dir_all(&dir).unwrap();

    return Ok(0);
}

async fn write_compfile(tool: ToolConfig, dir: String) {
    let comp_detail = Command::new("zsh")
        .arg("-c")
        .arg(&tool.exec)
        .output()
        .await
        .expect("");

    match fs::write(
        Path::new(&dir).join(format!("_{}", tool.name)),
        comp_detail.stdout,
    ) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            process::exit(256);
        }
    };
}

#[tokio::main]
async fn main() {
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
        SubCommands::Build {} => {
            let comp_dir = get_compfile_dir();

            match crean_dir(comp_dir.clone()) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error: {}", e);
                    process::exit(256);
                }
            }

            let config = get_config();
            let (tx1, mut rx1) = mpsc::channel::<String>(config.tool.len());

            for tool in config.tool.clone() {
                let tx1 = tx1.clone();
                let comp_dir = comp_dir.clone();
                tokio::spawn(async move {
                    write_compfile(tool.clone(), comp_dir).await;
                    tx1.send(format!("Completed: {}", tool.exec.clone()).to_string())
                        .await
                        .unwrap();
                    // println!("Completed: {}", config.tool[i].exec);
                });
            }
            drop(tx1);

            while let Some(msg) = rx1.recv().await {
                println!("{}", msg);
            }

            println!("Add {} to your fpath", comp_dir)
        }
    }
}
