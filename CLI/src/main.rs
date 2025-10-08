use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

#[derive(Serialize)]
struct RegisterRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
struct LoginRequest<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

#[derive(Serialize, Deserialize)]
struct AuthStore {
    access_token: String,
    token_type: String,
    expires_at: u64,
}

fn config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("VERSE_CONFIG_DIR") {
        return PathBuf::from(dir);
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("verse");
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config").join("verse")
}

fn auth_path() -> PathBuf {
    config_dir().join("auth.json")
}

fn save_auth(auth: &AuthStore) -> std::io::Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let path = auth_path();
    let json = serde_json::to_vec_pretty(auth).expect("serialize auth");
    let mut file = fs::File::create(&path)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = file.metadata()?.permissions();
        perms.set_mode(0o600);
        file.set_permissions(perms)?;
    }
    file.write_all(&json)?;
    Ok(())
}

fn load_auth() -> Result<AuthStore, String> {
    let path = auth_path();
    let data = fs::read_to_string(&path).map_err(|e| format!("Failed to read auth file ({}): {}", path.display(), e))?;
    let auth: AuthStore = serde_json::from_str(&data).map_err(|e| format!("Failed to parse auth file: {}", e))?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    if now >= auth.expires_at { return Err("Saved token has expired. Please run 'verse login' again.".into()); }
    Ok(auth)
}

fn main() {
    let matches = Command::new("verse")
        .version("1.0")
        .author("Salai Kowshikan")
        .about("This is V.E.R.S.E, a command line tool to provide a model validation interface that protects the privacy of both the parties involved.")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .help("Sets your name")
                .value_name("NAME"),
        )
        .subcommand(
            Command::new("request")
                .about("Makes a validation request to the provided model owner")
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                        .help("Path to the ZK-guest workspace directory")
                        .value_name("PATH")
                        .default_value("../ZK-guest"),
                ),
        )
        .subcommand(
            Command::new("register")
                .about("Register a new user on V.E.R.S.E")
                .arg(
                    Arg::new("email")
                        .help("Email address to register")
                        .value_name("EMAIL")
                        .required(true),
                )
                .arg(
                    Arg::new("password")
                        .help("Password for the account")
                        .value_name("PASSWORD")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("login")
                .about("Log into V.E.R.S.E")
                .arg(
                    Arg::new("email")
                        .help("Email address")
                        .value_name("EMAIL")
                        .required(true),
                )
                .arg(
                    Arg::new("password")
                        .help("Password")
                        .value_name("PASSWORD")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("model")
                .about("Manage your models")
                .subcommand(
                    Command::new("list")
                        .about("List all of your models"),
                )
                .subcommand(
                    Command::new("new")
                        .about("Create a new model")
                        .arg(
                            Arg::new("vector-format")
                                .long("vector-format")
                                .help("Vector format description, e.g. feature index mapping")
                                .value_name("FORMAT")
                                .required(true),
                        )
                        .arg(
                            Arg::new("name")
                                .long("name")
                                .help("Name of the model (e.g., Skin cancer prediction)")
                                .value_name("NAME")
                                .required(true),
                        )
                        .arg(
                            Arg::new("description")
                                .long("description")
                                .help("A longer description of how the model was trained and its predictions")
                                .value_name("TEXT")
                                .required(false),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("request", sub_m)) => {
            let dir = sub_m
                .get_one::<String>("dir")
                .map(String::as_str)
                .unwrap_or("ZK-guest");

            println!("Running `cargo run --release` in: {}", dir);

            let status = std::process::Command::new("cargo")
                .arg("run")
                .arg("--release")
                .current_dir(dir)
                .status();

            match status {
                Ok(s) => {
                    if let Some(code) = s.code() {
                        std::process::exit(code);
                    } else {
                        eprintln!("Process terminated by signal");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute cargo: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("register", sub_m)) => {
            let email = sub_m
                .get_one::<String>("email")
                .map(String::as_str)
                .expect("email is required");
            let password = sub_m
                .get_one::<String>("password")
                .map(String::as_str)
                .expect("password is required");

            let url = std::env::var("VERSE_API_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
            let endpoint = format!("{}/api/users/register", url.trim_end_matches('/'));

            let payload = RegisterRequest { email, password };

            println!("Registering '{}' at {}...", email, endpoint);

            let client = reqwest::blocking::Client::new();
            match client.post(endpoint).json(&payload).send() {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.text() {
                        Ok(body) => {
                            if status.is_success() {
                                println!("Success: {}", body);
                                std::process::exit(0);
                            } else {
                                eprintln!("Registration failed ({}): {}", status, body);
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to read response body: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("HTTP request error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("login", sub_m)) => {
            let email = sub_m
                .get_one::<String>("email")
                .map(String::as_str)
                .expect("email is required");
            let password = sub_m
                .get_one::<String>("password")
                .map(String::as_str)
                .expect("password is required");

            let url = std::env::var("VERSE_API_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
            let endpoint = format!("{}/api/users/login", url.trim_end_matches('/'));

            let payload = LoginRequest { email, password };

            println!("Logging in '{}' at {}...", email, endpoint);

            let client = reqwest::blocking::Client::new();
            match client.post(endpoint).json(&payload).send() {
                Ok(resp) => {
                    let status = resp.status();
                    if status.is_success() {
                        match resp.json::<TokenResponse>() {
                            Ok(token) => {
                                let now = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap_or_default()
                                    .as_secs();
                                // subtract a small skew (30s) to avoid edge expiry
                                let expires_at = now + token.expires_in.saturating_sub(30);
                                let store = AuthStore {
                                    access_token: token.access_token,
                                    token_type: token.token_type,
                                    expires_at,
                                };
                                if let Err(e) = save_auth(&store) {
                                    eprintln!("Login succeeded but failed to save token: {}", e);
                                    std::process::exit(1);
                                }
                                println!(
                                    "Login successful. Token saved to {}",
                                    auth_path().display()
                                );
                                std::process::exit(0);
                            }
                            Err(e) => {
                                eprintln!("Failed to parse token response: {}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        match resp.text() {
                            Ok(body) => {
                                eprintln!("Login failed ({}): {}", status, body);
                            }
                            Err(e) => eprintln!("Login failed ({}), and couldn't read body: {}", status, e),
                        }
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("HTTP request error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("model", sub_m)) => {
            // Common setup
            let url = std::env::var("VERSE_API_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
            let base = url.trim_end_matches('/');
            let client = reqwest::blocking::Client::new();

            match sub_m.subcommand() {
                Some(("list", _)) => {
                    let auth = match load_auth() {
                        Ok(a) => a,
                        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
                    };
                    let endpoint = format!("{}/api/model", base);
                    match client
                        .get(endpoint)
                        .header(AUTHORIZATION, format!("Bearer {}", auth.access_token))
                        .send()
                    {
                        Ok(resp) => {
                            let status = resp.status();
                            match resp.text() {
                                Ok(body) => {
                                    if status.is_success() {
                                        if body.trim().is_empty() || body.trim() == "[]" { println!("No models found."); }
                                        else { println!("{}", body); }
                                        std::process::exit(0);
                                    } else {
                                        eprintln!("List failed ({}): {}", status, body);
                                        std::process::exit(1);
                                    }
                                }
                                Err(e) => { eprintln!("Failed to read response body: {}", e); std::process::exit(1); }
                            }
                        }
                        Err(e) => { eprintln!("HTTP request error: {}", e); std::process::exit(1); }
                    }
                }
                Some(("new", sub_new)) => {
                    #[derive(Serialize)]
                    struct ModelCreate<'a> { vectorFormat: &'a str, name: &'a str, description: Option<&'a str> }

                    let auth = match load_auth() {
                        Ok(a) => a,
                        Err(e) => { eprintln!("{}", e); std::process::exit(1); }
                    };

                    let vector_format = sub_new.get_one::<String>("vector-format").map(String::as_str).expect("--vector-format is required");
                    let name = sub_new.get_one::<String>("name").map(String::as_str).expect("--name is required");
                    let description = sub_new.get_one::<String>("description").map(String::as_str);

                    let payload = ModelCreate { vectorFormat: vector_format, name, description };
                    let endpoint = format!("{}/api/model", base);
                    println!("Creating model '{}'...", name);
                    match client
                        .post(endpoint)
                        .header(AUTHORIZATION, format!("Bearer {}", auth.access_token))
                        .header(CONTENT_TYPE, "application/json")
                        .json(&payload)
                        .send()
                    {
                        Ok(resp) => {
                            let status = resp.status();
                            match resp.text() {
                                Ok(body) => {
                                    if status.is_success() { println!("Success: {}", body); std::process::exit(0); }
                                    else { eprintln!("Create failed ({}): {}", status, body); std::process::exit(1); }
                                }
                                Err(e) => { eprintln!("Failed to read response body: {}", e); std::process::exit(1); }
                            }
                        }
                        Err(e) => { eprintln!("HTTP request error: {}", e); std::process::exit(1); }
                    }
                }
                _ => {
                    eprintln!("Use: verse model list | verse model new --vector-format <FORMAT> --name <NAME> [--description <TEXT>]");
                    std::process::exit(2);
                }
            }
        }
        _ => {
            println!("This is V.E.R.S.E, a command line tool to provide a model validation interface that protects the privacy of both the parties involved. Use --help for more information.");
        }
    }
}