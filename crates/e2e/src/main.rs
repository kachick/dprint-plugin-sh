use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --package e2e -- <check|bump> [test_name]");
        std::process::exit(1);
    }

    let action = &args[1];
    let test_name = args.get(2).map(|s| s.as_str());

    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("Failed to find repo root")
        .to_path_buf();

    let plugin_path = std::env::var("PLUGIN_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            repo_root.join("target/wasm32-unknown-unknown/debug/dprint_plugin_sh.wasm")
        });

    if !plugin_path.exists() {
        eprintln!(
            "Plugin wasm not found at {}. Run `cargo x build` first.",
            plugin_path.display()
        );
        std::process::exit(1);
    }

    match action.as_str() {
        "check" => run_check(&repo_root, &plugin_path, test_name),
        "bump" => run_bump(&repo_root, &plugin_path, test_name),
        other => {
            eprintln!("Unknown action: {other}");
            std::process::exit(1);
        }
    }
}

fn find_target_file(dir: &Path) -> Option<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .ok()?
        .filter_map(|e| e.ok().map(|entry| entry.path()))
        .filter(|p| p.is_file())
        .collect();
    entries.sort();

    for path in entries {
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if file_name.starts_with("expected.") || file_name == ".envrc" {
                return Some(path);
            }
        }
    }
    None
}

fn get_test_dirs(repo_root: &Path, test_name: Option<&str>) -> Vec<PathBuf> {
    let tests_root = repo_root.join("tests");
    if let Some(name) = test_name {
        let dir = tests_root.join(name);
        if !dir.is_dir() {
            eprintln!("Test directory not found: {}", dir.display());
            std::process::exit(1);
        }
        vec![dir]
    } else {
        let mut dirs = Vec::new();
        for entry in fs::read_dir(&tests_root).expect("Failed to read tests directory") {
            let entry = entry.expect("Failed to read test entry");
            let path = entry.path();
            if path.is_dir() && find_target_file(&path).is_some() {
                dirs.push(path);
            }
        }
        dirs.sort();
        dirs
    }
}

fn find_raw_path(repo_root: &Path, dir: &Path) -> PathBuf {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut files: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|entry| entry.path()))
            .filter(|p| p.is_file())
            .collect();
        files.sort();

        for path in files {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("raw.") {
                    return path;
                }
            }
        }
    }
    repo_root.join("tests/raw.sh")
}

fn run_check(repo_root: &Path, plugin_path: &Path, test_name: Option<&str>) {
    let test_dirs = get_test_dirs(repo_root, test_name);

    for dir in test_dirs {
        let target_path = find_target_file(&dir).unwrap_or_else(|| {
            eprintln!(
                "target file (expected.* or .envrc) not found in {}",
                dir.display()
            );
            std::process::exit(1);
        });
        let target_file_name = target_path.file_name().unwrap();

        // 1. `dprint check --plugins=<PLUGIN_PATH> <target_file>`
        let status = Command::new("dprint")
            .current_dir(&dir)
            .arg("check")
            .arg(format!("--plugins={}", plugin_path.display()))
            .arg(target_file_name)
            .status()
            .expect("Failed to run dprint check");

        if !status.success() {
            eprintln!(
                "dprint check failed for {} in {}",
                target_path.display(),
                dir.display()
            );
            std::process::exit(1);
        }

        // 2. Format raw file and compare with expected
        let raw_path = find_raw_path(repo_root, &dir);
        let raw_content = fs::read(&raw_path).expect("Failed to read raw file");
        let expected_content =
            fs::read_to_string(&target_path).expect("Failed to read expected file");

        let mut child = Command::new("dprint")
            .current_dir(&dir)
            .arg("fmt")
            .arg("--stdin")
            .arg(target_file_name)
            .arg(format!("--plugins={}", plugin_path.display()))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn dprint fmt");

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&raw_content)
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to read dprint output");
        if !output.status.success() {
            eprintln!("dprint fmt failed for {}", raw_path.display());
            std::process::exit(1);
        }

        let actual_content = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
        if actual_content != expected_content {
            eprintln!(
                "Difference detected between formatted output and {}",
                target_path.display()
            );
            print_diff(&target_path, &actual_content);
            std::process::exit(1);
        }
    }
}

fn run_bump(repo_root: &Path, plugin_path: &Path, test_name: Option<&str>) {
    let test_dirs = get_test_dirs(repo_root, test_name);

    for dir in test_dirs {
        let target_path = find_target_file(&dir).unwrap_or_else(|| {
            eprintln!(
                "target file (expected.* or .envrc) not found in {}",
                dir.display()
            );
            std::process::exit(1);
        });
        let target_file_name = target_path.file_name().unwrap();

        let raw_path = find_raw_path(repo_root, &dir);
        let raw_content = fs::read(&raw_path).expect("Failed to read raw file");

        let mut child = Command::new("dprint")
            .current_dir(&dir)
            .arg("fmt")
            .arg("--stdin")
            .arg(target_file_name)
            .arg(format!("--plugins={}", plugin_path.display()))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to spawn dprint fmt");

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&raw_content)
                .expect("Failed to write to stdin");
        }

        let output = child
            .wait_with_output()
            .expect("Failed to read dprint output");
        if !output.status.success() {
            eprintln!("dprint fmt failed for {}", raw_path.display());
            std::process::exit(1);
        }

        fs::write(&target_path, output.stdout).expect("Failed to write expected file");
        println!("Updated {}", target_path.display());
    }
}

fn print_diff(expected_path: &Path, actual_content: &str) {
    if let Ok(mut child) = Command::new("diff")
        .arg("-u")
        .arg(expected_path)
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(actual_content.as_bytes());
        }
        let _ = child.wait();
    } else {
        eprintln!("(diff command not available)");
    }
}
