use arkaive::config::Config;
use arkaive::config::{Cli, Commands};
use clap::Parser;
use reqwest::redirect::Policy;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(|| {
        let mut path = dirs::config_dir().unwrap();
        path.push("arkaive.toml");
        path
    });

    let mut client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .redirect(Policy::none())
        .build()
        .unwrap();

    let mut config = match Config::load(&config_path) {
        Ok(res) => res,
        Err(_) => {
            println!("File does not exist, creating new");
            Config {
                username: None,
                password: None,
            }
        }
    };

    match cli.command {
        Commands::SetConfig { username, password } => {
            if let Some(username) = username {
                config.set_username(&username);
            }
            if let Some(password) = password {
                config.set_password(&password);
            }
            config.save(config_path.as_path()).unwrap();
        }

        Commands::TestLogin { username, password } => {
            let config = match Config::load(&config_path) {
                Ok(res) => res,
                Err(_) => {
                    println!("File does not exist, creating new");
                    Config {
                        username: None,
                        password: None,
                    }
                }
            };
            let username = if let Some(username) = username.as_deref().or_else(|| config.username())
            {
                username
            } else {
                eprintln!("No username provided");
                return;
            };
            let password = if let Some(password) = password.as_deref().or_else(|| config.password())
            {
                password
            } else {
                eprintln!("No password provided");
                return;
            };

            if let Err(e) = arkaive::auth::auth(username, password, &mut client).await {
                eprintln!("Authentication failed: {:?}", e);
            } else {
                println!("Authentication is successful",);
            }

            config.save(&config_path).unwrap();
        }

        Commands::ListClasses { username, password, id_only } => {

            let username = if let Some(username) = username.as_deref().or_else(|| config.username())
            {
                username
            } else {
                eprintln!("No username provided");
                return;
            };
            let password = if let Some(password) = password.as_deref().or_else(|| config.password())
            {
                password
            } else {
                eprintln!("No password provided");
                return;
            };

            if let Err(e) = arkaive::auth::auth(username, password, &mut client).await {
                eprintln!("Authentication failed: {:?}", e);
            } 

            match arkaive::utils::list_classes(&mut client).await {
                Ok(data) => {
                    if id_only {
                        data.into_iter().for_each(|x| std::println!("{}", x.url.id()));
                    }
                    else {
                        data.into_iter().for_each(|x| std::println!("{}", x));
                    }
                }
                Err(e) => eprintln!("Authentication failed: {:?}", e),
            }

            config.save(&config_path).unwrap();
        }

        Commands::Checkin {
            class,
            username,
            password,
        } => {
            let username = if let Some(username) = username.as_deref().or_else(|| config.username())
            {
                username
            } else {
                eprintln!("No username provided");
                return;
            };
            let password = if let Some(password) = password.as_deref().or_else(|| config.password())
            {
                password
            } else {
                eprintln!("No password provided");
                return;
            };

            if let Err(e) = arkaive::auth::auth(username, password, &mut client).await {
                eprintln!("Authentication failed: {:?}", e);
                return ;
            } 

            match arkaive::utils::checkin(class, &mut client).await {
                Ok(data) => {
                    println!("{:?}", data)
                }
                Err(e) => eprintln!("Authentication failed: {:?}", e),
            }

            config.save(&config_path).unwrap();
        }
    }
}
