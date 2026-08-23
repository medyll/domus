use serde_json::Value;
use std::path::Path;
use std::process::Command;

const FORBIDDEN_FRAMEWORKS: &[&str] = &["dioxus", "tauri"];

#[test]
fn workspace_does_not_depend_on_competing_frameworks() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("domius-cli should be inside the workspace");
    let manifest_path = workspace_root.join("Cargo.toml");

    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--manifest-path",
        ])
        .arg(&manifest_path)
        .output()
        .expect("cargo metadata should start");

    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let metadata: Value =
        serde_json::from_slice(&output.stdout).expect("cargo metadata should return valid JSON");
    let packages = metadata["packages"]
        .as_array()
        .expect("cargo metadata should contain packages");

    let mut offenders = packages
        .iter()
        .filter_map(|package| package["name"].as_str())
        .filter(|name| {
            FORBIDDEN_FRAMEWORKS
                .iter()
                .any(|framework| *name == *framework || name.starts_with(&format!("{framework}-")))
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();

    offenders.sort();
    offenders.dedup();

    assert!(
        offenders.is_empty(),
        "competing application frameworks are forbidden in the Domius workspace: {}",
        offenders.join(", ")
    );
}
