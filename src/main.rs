use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs, io, process};
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
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    #[clap(about = "Get config")]
    Getconfig {},
    #[clap(about = "Generate the autocompletion script for the specified shell")]
    Build {
        #[arg(short, long, help = "Keep existing ones")]
        keep: bool,
        #[arg(short, long)]
        quiet: bool,
    },
    Completion {
        #[clap(short, long)]
        shell: Shell,
    },
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

async fn write_compfile(tool: ToolConfig, dir: String, keep: bool) -> bool {
    if Path::new(&dir).join(format!("_{}", tool.name)).exists() && keep {
        return true;
    }

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

    return false;
}

fn print_completions<G: Generator>(generator: G, cmd: &mut clap::Command) {
    generate(
        generator,
        cmd,
        cmd.get_name().to_string(),
        &mut io::stdout(),
    );
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommands::Getconfig {} => {
            match fs::exists(get_config_path()) {
                Ok(t) => {
                    if !t {
                        println!("No settings found");
                        process::exit(1);
                    }
                }
                Err(_) => {
                    println!("No settings found");
                    process::exit(1);
                }
            }

            println!("{}", get_config_path());
            let config = get_config();

            for i in 0..config.tool.len() {
                println!(
                    "name: {} exec: {}",
                    config.tool[i].name, config.tool[i].exec
                );
            }
        }
        SubCommands::Build { keep, quiet } => {
            let comp_dir = get_compfile_dir();

            if !keep {
                match crean_dir(comp_dir.clone()) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {}", e);
                        process::exit(256);
                    }
                }
            }

            match fs::exists(get_config_path()) {
                Ok(t) => {
                    if !t {
                        println!("No settings found");
                        process::exit(1);
                    }
                }
                Err(_) => {
                    println!("No settings found");
                    process::exit(1);
                }
            }

            let config = get_config();
            let (tx1, mut rx1) = mpsc::channel::<String>(config.tool.len());

            for tool in config.tool.clone() {
                let tx1 = tx1.clone();
                let comp_dir = comp_dir.clone();
                let keep = keep.clone();
                tokio::spawn(async move {
                    let kept = write_compfile(tool.clone(), comp_dir, keep).await;
                    tx1.send(
                        format!(
                            "{}: {}",
                            if kept { "Kept" } else { "Completed" },
                            tool.exec.clone()
                        )
                        .to_string(),
                    )
                    .await
                    .unwrap();
                    // println!("Completed: {}", config.tool[i].exec);
                });
            }
            drop(tx1);

            while let Some(msg) = rx1.recv().await {
                if !quiet {
                    println!("{}", msg);
                }
            }

            if !quiet {
                println!("Add {} to your fpath", comp_dir)
            }
        }
        SubCommands::Completion { shell } => {
            let mut cmd = Cli::command();
            print_completions(*shell, &mut cmd);
        }
    }
}
