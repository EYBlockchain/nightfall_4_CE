use std::{
    collections::BTreeSet,
    env, fs,
    fs::OpenOptions,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde_json::{Value, json};

const DEFAULT_PORT: u16 = 8081;
const WEBHOOK_DIR: &str = ".nightfall/webhook";
const PID_FILE: &str = ".nightfall/webhook/webhook.pid";
const LOG_FILE: &str = ".nightfall/webhook/webhook.log";
const EVENTS_FILE: &str = ".nightfall/webhook/events.jsonl";

pub struct LocalWebhook {
    pub port: u16,
    pub events_path: PathBuf,
    pub log_path: PathBuf,
}

pub fn ensure_local(port: u16) -> Result<LocalWebhook, String> {
    ensure_webhook_dir()?;

    if health_ok(port) {
        return Ok(local_webhook(port));
    }

    if tcp_port_open(port) {
        return Err(format!(
            "Port {port} is already in use, but it does not look like the nf4 testing webhook."
        ));
    }

    let exe = env::current_exe().map_err(|err| format!("Could not find nf4 executable: {err}"))?;
    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .map_err(|err| format!("Failed to open {LOG_FILE}: {err}"))?;
    let err_log = log
        .try_clone()
        .map_err(|err| format!("Failed to clone webhook log handle: {err}"))?;

    let child = Command::new(exe)
        .args(["webhook", "serve", &port.to_string()])
        .env("NF4_WEBHOOK_EVENTS_FILE", EVENTS_FILE)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(err_log))
        .spawn()
        .map_err(|err| format!("Failed to start local testing webhook: {err}"))?;

    fs::write(PID_FILE, child.id().to_string())
        .map_err(|err| format!("Failed to write {PID_FILE}: {err}"))?;

    for _ in 0..20 {
        if health_ok(port) {
            return Ok(local_webhook(port));
        }
        thread::sleep(Duration::from_millis(250));
    }

    Err(format!(
        "Local testing webhook did not become healthy. Check {LOG_FILE}."
    ))
}

pub fn start_command(port: Option<u16>) -> Result<(), String> {
    let port = port.unwrap_or(DEFAULT_PORT);
    let local = ensure_local(port)?;

    println!("Local testing webhook OK");
    println!("  url: http://127.0.0.1:{}/webhook", local.port);
    println!("  events: {}", local.events_path.display());
    println!("  logs: {}", local.log_path.display());
    Ok(())
}

pub fn serve(port: u16) -> Result<(), String> {
    ensure_webhook_dir()?;
    let events_path = events_path();
    let listener = TcpListener::bind(("0.0.0.0", port))
        .map_err(|err| format!("Failed to bind local testing webhook on port {port}: {err}"))?;

    println!("nf4 testing webhook listening on 0.0.0.0:{port}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(err) = handle_stream(stream, &events_path) {
                    eprintln!("webhook request failed: {err}");
                }
            }
            Err(err) => eprintln!("webhook connection failed: {err}"),
        }
    }
    Ok(())
}

pub fn status() -> Result<(), String> {
    let pid = fs::read_to_string(PID_FILE)
        .ok()
        .map(|pid| pid.trim().to_string());
    let port = DEFAULT_PORT;

    println!("Local testing webhook status");
    match pid {
        Some(pid) if process_alive(&pid) => println!("  process: running, pid {pid}"),
        Some(pid) => println!("  process: not running, stale pid {pid}"),
        None => println!("  process: no pid file"),
    }
    println!(
        "  health: {}",
        if health_ok(port) {
            "OK"
        } else {
            "not reachable on default port"
        }
    );
    println!("  default_url: http://127.0.0.1:{port}/webhook");
    println!("  events: {}", Path::new(EVENTS_FILE).display());
    println!("  logs: {}", Path::new(LOG_FILE).display());
    Ok(())
}

pub fn logs() -> Result<(), String> {
    print_last_lines(LOG_FILE, "No webhook logs found yet.")
}

pub fn events() -> Result<(), String> {
    match fs::read_to_string(EVENTS_FILE) {
        Ok(source) if !source.trim().is_empty() => {
            print!("{source}");
            Ok(())
        }
        _ => {
            println!("No webhook events found yet.");
            Ok(())
        }
    }
}

pub fn salts() -> Result<(), String> {
    let source = fs::read_to_string(EVENTS_FILE).unwrap_or_default();
    let salts = collect_salts_from_events(&source);

    if salts.is_empty() {
        println!("No withdraw_fund_salt values found yet.");
    } else {
        for salt in salts {
            println!("{salt}");
        }
    }
    Ok(())
}

fn local_webhook(port: u16) -> LocalWebhook {
    LocalWebhook {
        port,
        events_path: PathBuf::from(EVENTS_FILE),
        log_path: PathBuf::from(LOG_FILE),
    }
}

fn ensure_webhook_dir() -> Result<(), String> {
    fs::create_dir_all(WEBHOOK_DIR).map_err(|err| format!("Failed to create {WEBHOOK_DIR}: {err}"))
}

fn events_path() -> PathBuf {
    env::var("NF4_WEBHOOK_EVENTS_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(EVENTS_FILE))
}

fn handle_stream(mut stream: TcpStream, events_path: &Path) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let request = read_request(&mut stream)?;

    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/health") => {
            write_response(&mut stream, 200, "application/json", r#"{"ok":true}"#)
        }
        ("POST", "/webhook") => {
            append_event(events_path, &request.body)?;
            write_response(&mut stream, 200, "text/plain", "OK")
        }
        _ => write_response(&mut stream, 404, "text/plain", "not found"),
    }
}

struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

fn read_request(stream: &mut TcpStream) -> std::io::Result<HttpRequest> {
    let mut request = Vec::new();
    let mut buffer = [0_u8; 4096];
    let header_end;

    loop {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "connection closed before headers",
            ));
        }
        request.extend_from_slice(&buffer[..count]);
        if let Some(index) = find_header_end(&request) {
            header_end = index;
            break;
        }
        if request.len() > 1024 * 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "request headers too large",
            ));
        }
    }

    let header_text = String::from_utf8_lossy(&request[..header_end]);
    let mut lines = header_text.lines();
    let request_line = lines.next().unwrap_or_default();
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string();
    let content_length = lines
        .find_map(|line| {
            let (key, value) = line.split_once(':')?;
            key.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .unwrap_or(0);

    let body_start = header_end + 4;
    while request.len() < body_start + content_length {
        let count = stream.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        request.extend_from_slice(&buffer[..count]);
    }

    let body_end = request.len().min(body_start + content_length);
    Ok(HttpRequest {
        method,
        path,
        body: request[body_start..body_end].to_vec(),
    })
}

fn find_header_end(bytes: &[u8]) -> Option<usize> {
    bytes.windows(4).position(|window| window == b"\r\n\r\n")
}

fn append_event(path: &Path, body: &[u8]) -> std::io::Result<()> {
    let payload = serde_json::from_slice::<Value>(body)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(body).trim().to_string()));
    let event = json!({
        "received_at_unix": unix_timestamp(),
        "payload": payload,
    });

    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{event}")?;
    Ok(())
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &str,
) -> std::io::Result<()> {
    let status_text = match status {
        200 => "OK",
        404 => "Not Found",
        _ => "OK",
    };
    write!(
        stream,
        "HTTP/1.1 {status} {status_text}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

fn health_ok(port: u16) -> bool {
    let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    if stream
        .write_all(b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n")
        .is_err()
    {
        return false;
    }
    let mut response = String::new();
    stream.read_to_string(&mut response).is_ok() && response.starts_with("HTTP/1.1 200")
}

fn tcp_port_open(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).is_ok()
}

fn process_alive(pid: &str) -> bool {
    Command::new("kill")
        .args(["-0", pid])
        .status()
        .is_ok_and(|status| status.success())
}

fn print_last_lines(path: &str, empty_message: &str) -> Result<(), String> {
    let source = fs::read_to_string(path).unwrap_or_default();
    if source.trim().is_empty() {
        println!("{empty_message}");
        return Ok(());
    }

    let lines = source.lines().collect::<Vec<_>>();
    let start = lines.len().saturating_sub(200);
    for line in &lines[start..] {
        println!("{line}");
    }
    Ok(())
}

fn collect_salts_from_events(source: &str) -> BTreeSet<String> {
    let mut salts = BTreeSet::new();
    for line in source.lines().filter(|line| !line.trim().is_empty()) {
        if let Ok(value) = serde_json::from_str::<Value>(line) {
            collect_salts_from_value(&value, &mut salts);
        }
    }
    salts
}

fn collect_salts_from_value(value: &Value, salts: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                if matches!(key.as_str(), "withdraw_fund_salt" | "withdrawFundSalt") {
                    if let Some(salt) = value.as_str() {
                        salts.insert(salt.to_string());
                    }
                }
                collect_salts_from_value(value, salts);
            }
        }
        Value::Array(values) => {
            for value in values {
                collect_salts_from_value(value, salts);
            }
        }
        Value::String(value) => {
            if value.contains("withdraw_fund_salt") || value.contains("withdrawFundSalt") {
                if let Ok(nested) = serde_json::from_str::<Value>(value) {
                    collect_salts_from_value(&nested, salts);
                }
            }
        }
        _ => {}
    }
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::collect_salts_from_events;

    #[test]
    fn extracts_withdraw_salts_from_webhook_events() {
        let source = r#"
{"payload":{"response":"{\"withdraw_fund_salt\":\"0xabc\"}"}}
{"payload":{"withdrawFundSalt":"0xdef"}}
"#;
        let salts = collect_salts_from_events(source);
        assert!(salts.contains("0xabc"));
        assert!(salts.contains("0xdef"));
    }
}
