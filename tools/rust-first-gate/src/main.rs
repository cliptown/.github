//! Organization-local policy validation. Existing Python automation remains
//! allowlisted until a parity-tested Rust replacement is reviewed.
use std::fs;
use std::path::Path;
use std::process::{Command, ExitCode};

const LEGACY_PYTHON_AUTOMATION: &[&str] = &[
    "scripts/repository_relationships_lib.py",
    "scripts/validate_baseline.py",
    "scripts/validate_public_privacy.py",
    "scripts/validate_repository_relationships.py",
    "scripts/validate_workflow_security.py",
];

const REQUIRED_FILES: &[&str] = &[
    "README.md",
    "profile/README.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
    "SUPPORT.md",
    "CODE_OF_CONDUCT.md",
    "GOVERNANCE.md",
    "AGENTS.md",
    "agents.md",
    "organization-policy.json",
    "repository-relationships.json",
];

fn added_python_scripts<'a>(tracked_paths: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    tracked_paths
        .into_iter()
        .filter(|path| path.ends_with(".py") && !LEGACY_PYTHON_AUTOMATION.contains(path))
        .map(str::to_owned)
        .collect()
}

fn tracked_paths(root: &Path) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .args(["ls-files", "-z", "--cached"])
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot inspect tracked files: {error}"))?;
    if !output.status.success() {
        return Err("git ls-files failed; refusing to skip script inspection".into());
    }
    output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| {
            std::str::from_utf8(path)
                .map(str::to_owned)
                .map_err(|_| "tracked path is not valid UTF-8".into())
        })
        .collect()
}

fn has_rule(content: &str) -> bool {
    content.contains("Do not add Python scripts or Python test harnesses.")
        && content.contains("Use Rust instead.")
        && content.contains("Python")
}

fn check(root: &Path) -> Result<(), String> {
    for file in REQUIRED_FILES {
        let bytes = fs::read(root.join(file))
            .map_err(|_| format!("required baseline file missing: {file}"))?;
        if bytes.is_empty() {
            return Err(format!("required baseline file empty: {file}"));
        }
    }

    for file in ["organization-policy.json", "repository-relationships.json"] {
        let bytes = fs::read(root.join(file))
            .map_err(|_| format!("unable to read JSON baseline: {file}"))?;
        serde_json::from_slice::<serde_json::Value>(&bytes)
            .map_err(|_| format!("invalid JSON baseline: {file}"))?;
    }

    let agents = fs::read_to_string(root.join("agents.md"))
        .map_err(|_| "cannot read lowercase agent policy")?;
    let compatibility = fs::read_to_string(root.join("AGENTS.md"))
        .map_err(|_| "cannot read uppercase agent policy")?;
    if !has_rule(&agents) || !has_rule(&compatibility) || agents != compatibility {
        return Err("canonical and compatibility agent policies must match the Rust-first rule".into());
    }

    let violations = added_python_scripts(tracked_paths(root)?.iter().map(String::as_str));
    if !violations.is_empty() {
        return Err(format!(
            "new Python scripts are prohibited; use Rust instead: {}",
            violations.join(", ")
        ));
    }
    Ok(())
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| ".".into());
    if args.next().is_some() {
        eprintln!("usage: cliptown-rust-first-gate [repository-root]");
        return ExitCode::from(2);
    }
    match check(Path::new(&root)) {
        Ok(()) => {
            println!("Rust-first policy, JSON baselines, and legacy Python allowlist verified");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Rust-first policy: {error}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_automation_is_not_removed_prematurely() {
        assert!(added_python_scripts(LEGACY_PYTHON_AUTOMATION.iter().copied()).is_empty());
    }

    #[test]
    fn rejects_new_python_tests_scripts_and_automation() {
        let violations = added_python_scripts([
            "scripts/new_test.py",
            "tools/generator.py",
            "test/security_test.py",
            "README.md",
            "src/main.rs",
        ]);
        assert_eq!(
            violations,
            vec![
                "scripts/new_test.py",
                "tools/generator.py",
                "test/security_test.py",
            ]
        );
    }

    #[test]
    fn json_is_parsed_strictly_not_merely_nonempty() {
        assert!(serde_json::from_str::<serde_json::Value>(r#"{"ok":true}"#).is_ok());
        assert!(serde_json::from_str::<serde_json::Value>(r#"{"ok":}"#).is_err());
    }

    #[test]
    fn rust_first_policy_requires_the_actual_agent_instruction() {
        assert!(has_rule("Do not add Python scripts or Python test harnesses. Use Rust instead. Python"));
        assert!(!has_rule("Prefer Rust where convenient."));
    }
}
