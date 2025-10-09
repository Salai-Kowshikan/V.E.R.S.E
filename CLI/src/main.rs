use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

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

fn pretty_print_models(body: &str) {
    match serde_json::from_str::<Value>(body) {
        Ok(Value::Array(items)) => {
            if items.is_empty() {
                println!("No models found.");
                return;
            }
            println!("Models ({}):", items.len());
            for (i, item) in items.iter().enumerate() {
                let id = item.get("id").and_then(|v| v.as_str()).unwrap_or("-");
                let name = item.get("name").and_then(|v| v.as_str()).unwrap_or("-");
                let vector = item.get("vectorFormat").and_then(|v| v.as_str()).unwrap_or("-");
                let created = item.get("createdAt").and_then(|v| v.as_str()).unwrap_or("-");
                let updated = item.get("updatedAt").and_then(|v| v.as_str()).unwrap_or("-");
                println!("\n{}. {}", i + 1, name);
                println!("   id:           {}", id);
                println!("   vectorFormat: {}", vector);
                println!("   createdAt:    {}", created);
                println!("   updatedAt:    {}", updated);
            }
        }
        _ => {
            // Fallback to raw output if parsing fails or response isn't an array
            println!("{}", body);
        }
    }
}

fn pretty_print_validation_request(body: &str) {
    match serde_json::from_str::<Value>(body) {
        Ok(Value::Object(obj)) => {
            let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or("-");
            let model_id = obj.get("modelId").and_then(|v| v.as_str()).unwrap_or("-");
            let verifier_id = obj.get("verifierId").and_then(|v| v.as_str()).unwrap_or("-");
            let elf_url = obj.get("elfFileUrl").and_then(|v| v.as_str()).unwrap_or("-");
            let json_url = obj.get("jsonUrl").and_then(|v| v.as_str()).unwrap_or("-");
            let status = obj.get("status").and_then(|v| v.as_str()).unwrap_or("-");
            let created = obj.get("createdAt").and_then(|v| v.as_str()).unwrap_or("-");

            let proof_hash = match obj.get("proofHash") {
                Some(Value::Array(arr)) => {
                    let nums: Vec<String> = arr
                        .iter()
                        .map(|x| match x {
                            Value::Number(n) => n.to_string(),
                            other => other.to_string(),
                        })
                        .collect();
                    format!("[{}]", nums.join(", "))
                }
                Some(other) => other.to_string(),
                None => "-".to_string(),
            };

            println!("Validation request submitted:\n");
            println!("  id:           {}", id);
            println!("  modelId:      {}", model_id);
            println!("  verifierId:   {}", verifier_id);
            println!("  elfFileUrl:   {}", elf_url);
            println!("  jsonUrl:      {}", json_url);
            println!("  proofHash:    {}", proof_hash);
            println!("  status:       {}", status);
            println!("  createdAt:    {}", created);
        }
        _ => println!("{}", body),
    }
}

fn pretty_print_pending_validations(body: &str) {
    match serde_json::from_str::<Value>(body) {
        Ok(Value::Object(obj)) => {
            let models = obj.get("models").and_then(|v| v.as_array());
            if models.is_none() {
                println!("{}", body);
                return;
            }
            let models = models.unwrap();
            let mut total_pending = 0usize;
            let mut any = false;
            for (mi, m) in models.iter().enumerate() {
                let model_id = m.get("id").and_then(|v| v.as_str()).unwrap_or("-");
                let model_name = m.get("name").and_then(|v| v.as_str()).unwrap_or("");
                // Own the array to avoid borrowing a temporary
                let vrs: Vec<Value> = m
                    .get("validationRequests")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();
                let pending: Vec<&Value> = vrs
                    .iter()
                    .filter(|vr| vr.get("status").and_then(|s| s.as_str()) == Some("pending"))
                    .collect();
                if pending.is_empty() {
                    continue;
                }
                any = true;
                println!(
                    "Model {} (id: {}{}):",
                    mi + 1,
                    model_id,
                    if model_name.is_empty() { String::new() } else { format!(", name: {}", model_name) }
                );
                for (i, vr) in pending.iter().enumerate() {
                    let id = vr.get("id").and_then(|v| v.as_str()).unwrap_or("-");
                    let verifier_id = vr.get("verifierId").and_then(|v| v.as_str()).unwrap_or("-");
                    let elf_url = vr.get("elfFileUrl").and_then(|v| v.as_str()).unwrap_or("-");
                    let created = vr.get("createdAt").and_then(|v| v.as_str()).unwrap_or("-");
                    println!("  {}. validation:", i + 1);
                    println!("     id:         {}", id);
                    println!("     verifierId: {}", verifier_id);
                    println!("     elfFileUrl: {}", elf_url);
                    println!("     status:     pending");
                    println!("     createdAt:  {}", created);
                }
                total_pending += pending.len();
                println!("");
            }
            if !any {
                println!("No pending validation requests found.");
            } else {
                println!("Total pending: {}", total_pending);
            }
        }
        _ => println!("{}", body),
    }
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
                .about("Build the ZK guest and send a validation request for a model")
                .arg(
                    Arg::new("model-id")
                        .long("model-id")
                        .short('m')
                        .help("The target model ID to request validation for")
                        .value_name("MODEL_ID")
                        .required(true),
                )
                .arg(
                    Arg::new("dir")
                        .short('d')
                        .long("dir")
                        .help("Path to the ZK-guest workspace directory")
                        .value_name("PATH")
                        .default_value("../ZK-guest"),
                )
                .arg(
                    Arg::new("elf")
                        .long("elf")
                        .help("Path to an already exported ELF file (skips autodetect)")
                        .value_name("FILE")
                        .required(false),
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
                .arg(
                    Arg::new("requests")
                        .long("requests")
                        .help("List pending validation requests for your models")
                        .required(false)
                        .action(clap::ArgAction::SetTrue),
                )
                .subcommand(
                    Command::new("list")
                        .about("List all of your models"),
                )
                .subcommand(
                    Command::new("new")
                        .about("Create a new model")
                        .long_about(
                            "Create a new model for the authenticated user.\n\nFields:\n  - vectorFormat: The size/order of the input vector and which feature goes at each index.\n  - name: A short name for the model (e.g., Skin cancer prediction).\n  - description: More about how the model was trained and what its predictions mean.\n\nExample JSON body that the API receives:\n{\n  \"vectorFormat\": \"len=3; x[0]=age, x[1]=bmi, x[2]=bp\",\n  \"name\": \"Skin cancer prediction\",\n  \"description\": \"Trained on dermatoscopic images; outputs malignancy probability.\"\n}"
                        )
                        .after_help(
                            "Tip: ensure you are logged in (verse login) so your JWT is available for Authorization."
                        )
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
                                .help("Description: how the model was trained and what its predictions mean")
                                .value_name("TEXT")
                                .required(false),
                        ),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("request", sub_m)) => {
            // Create/send validation request path
            let model_id = sub_m
                .get_one::<String>("model-id")
                .map(String::as_str)
                .expect("--model-id is required");

            let dir = sub_m
                .get_one::<String>("dir")
                .map(String::as_str)
                .unwrap_or("ZK-guest");

            let explicit_elf = sub_m.get_one::<String>("elf").map(String::as_str);

            // 1) Build/Run guest to produce/export the ELF (if user didn't supply an ELF)
            if explicit_elf.is_none() {
                println!("Running `cargo run --release` in: {}", dir);
                let status = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("--release")
                    .current_dir(dir)
                    .status();
                match status {
                    Ok(s) => {
                        if !s.success() {
                            if let Some(code) = s.code() { eprintln!("Guest run failed with exit code {}", code); }
                            else { eprintln!("Guest run terminated by signal"); }
                            std::process::exit(1);
                        }
                    }
                    Err(e) => { eprintln!("Failed to execute cargo: {}", e); std::process::exit(1); }
                }
            }
            let elf_path: PathBuf = if let Some(p) = explicit_elf {
                PathBuf::from(p)
            } else {
                PathBuf::from("/home/kowshik/V.E.R.S.E/ZK-guest/LinearRegression_exported")
            };

            if !elf_path.exists() { eprintln!("ELF file not found: {}", elf_path.display()); std::process::exit(1); }

            // Read hash value exported by guest (LinearRegression_ID_exported)
            let id_path = PathBuf::from(dir).join("LinearRegression_ID_exported");
            let hash_value = match fs::read_to_string(&id_path) {
                Ok(s) => s.trim().to_string(),
                Err(e) => {
                    eprintln!(
                        "Failed to read hash value from {}: {}. Ensure the guest wrote 'LinearRegression_ID_exported'.",
                        id_path.display(), e
                    );
                    std::process::exit(1);
                }
            };

            let auth = match load_auth() {
                Ok(a) => a,
                Err(e) => { eprintln!("{}", e); std::process::exit(1); }
            };

            // 4) Send multipart POST to /api/model/validation-request
            let url = std::env::var("VERSE_API_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());
            let endpoint = format!("{}/api/model/validation-request", url.trim_end_matches('/'));
            println!("Uploading validation request for model {} with ELF: {}", model_id, elf_path.display());

            let client = reqwest::blocking::Client::new();
            let file_name = elf_path.file_name().and_then(|s| s.to_str()).unwrap_or("guest.elf");
            let file = match fs::File::open(&elf_path) {
                Ok(f) => f,
                Err(e) => { eprintln!("Failed to open ELF file: {}", e); std::process::exit(1); }
            };

            let part = reqwest::blocking::multipart::Part::reader(file)
                .file_name(file_name.to_string())
                .mime_str("application/octet-stream").unwrap();

            let form = reqwest::blocking::multipart::Form::new()
                .text("model_id", model_id.to_string())
                .text("hashValue", hash_value)
                .part("elf_file", part);

            match client.post(endpoint)
                .header(AUTHORIZATION, format!("Bearer {}", auth.access_token))
                .multipart(form)
                .send() {
                Ok(resp) => {
                    let status = resp.status();
                    match resp.text() {
                        Ok(body) => {
                            if status.is_success() { pretty_print_validation_request(&body); std::process::exit(0); }
                            else { eprintln!("Request failed ({}): {}", status, body); std::process::exit(1); }
                        }
                        Err(e) => { eprintln!("Failed to read response body: {}", e); std::process::exit(1); }
                    }
                }
                Err(e) => { eprintln!("HTTP request error: {}", e); std::process::exit(1); }
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

            // If --requests flag is provided, list pending validation requests
            if sub_m.get_flag("requests") {
                let auth = match load_auth() {
                    Ok(a) => a,
                    Err(e) => { eprintln!("{}", e); std::process::exit(1); }
                };
                let endpoint = format!("{}/api/model/validations", base);
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
                                    pretty_print_pending_validations(&body);
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
                                        if body.trim().is_empty() || body.trim() == "[]" {
                                            println!("No models found.");
                                        } else {
                                            pretty_print_models(&body);
                                        }
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