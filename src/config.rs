use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Config {
    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = Some(username.to_string());
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = Some(password.to_string());
    }

    pub fn save(&self, path: &std::path::Path) -> std::io::Result<()> {
        let data = toml::to_string(self).unwrap();
        std::fs::write(path, data)
    }

    pub fn load(path: &std::path::Path) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        toml::from_str(&data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    SetConfig {
        #[clap(short, long)]
        username: Option<String>,
        #[clap(short, long)]
        password: Option<String>,
    },

    TestLogin {
        #[clap(short, long)]
        username: Option<String>,
        #[clap(short, long)]
        password: Option<String>,
    },

    ListClasses {
        #[clap(short, long)]
        username: Option<String>,
        #[clap(short, long)]
        password: Option<String>,

        #[clap(short, long("id-only"))]
        id_only: bool,
    },

    Checkin {
        class: i32,
        #[clap(short, long)]
        username: Option<String>,
        #[clap(short, long)]
        password: Option<String>,
    },
}
