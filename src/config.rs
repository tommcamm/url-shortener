use anyhow::Context;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub api_key: String,
    pub base_url: String,
    pub environment: Environment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Environment::Development),
            "production" | "prod" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        // Check for command line arguments
        let env_from_args = Self::check_command_line_args();

        // Check for config file
        let env_from_file = Self::check_config_file();

        // Check environment variable
        let env_from_var = Self::check_env_var();

        // Determine environment (command line takes precedence, then file, then env var, then default)
        let environment = env_from_args
            .or(env_from_file)
            .or(env_from_var)
            .unwrap_or(Environment::Development);

        // Log the active environment
        tracing::info!("Application running in {} mode", environment);

        Ok(Self {
            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            redis_url: std::env::var("REDIS_URL").context("REDIS_URL must be set")?,
            api_key: std::env::var("API_KEY").context("API_KEY must be set")?,
            base_url: std::env::var("BASE_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            environment,
        })
    }

    fn check_command_line_args() -> Option<Environment> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            if args.contains(&"--dev".to_string()) || args.contains(&"--development".to_string()) {
                Some(Environment::Development)
            } else if args.contains(&"--prod".to_string())
                || args.contains(&"--production".to_string())
            {
                Some(Environment::Production)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn check_config_file() -> Option<Environment> {
        let config_file = ".env.app";
        if Path::new(config_file).exists() {
            match fs::read_to_string(config_file) {
                Ok(contents) => {
                    for line in contents.lines() {
                        if line.starts_with("APP_ENV=") {
                            let value = line.trim_start_matches("APP_ENV=");
                            if let Ok(env) = Environment::from_str(value) {
                                return Some(env);
                            }
                        }
                    }
                    None
                }
                Err(_) => None,
            }
        } else {
            None
        }
    }

    fn check_env_var() -> Option<Environment> {
        std::env::var("APP_ENV")
            .ok()
            .and_then(|val| Environment::from_str(&val).ok())
    }
}
