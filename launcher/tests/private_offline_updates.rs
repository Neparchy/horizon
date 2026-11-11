//! A previously staged upstream update must not replace an offline launcher.
use std::process::{Command, Stdio};

#[test]
fn an_offline_launcher_ignores_an_old_update_handoff() {
    let root = std::env::temp_dir().join(format!(
        "horizon-offline-update-handoff-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    let home = root.join("launcher");
    let stage =
        home.join(".launcher-update")
            .join(format!("{}-{}", std::process::id(), "0".repeat(32)));
    std::fs::create_dir_all(&stage).unwrap();
    let payload = stage.join("horizon-launcher.exe");
    let bytes = b"an old staged update that must not be installed or removed";
    std::fs::write(&payload, bytes).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_horizon-launcher"))
        .arg("apply-update")
        .arg("launcher@99.0.0")
        .arg(&payload)
        .env("HORIZON_HOME", root.join("home"))
        .env("HORIZON_LAUNCHER_HOME", &home)
        .env("XDG_DATA_HOME", root.join("data"))
        .env("XDG_CONFIG_HOME", root.join("config"))
        .env("HORIZON_LAUNCHER_TRASH", root.join("trash"))
        .env("HORIZON_LAUNCHER_CLIPBOARD", root.join("clipboard"))
        .env("HORIZON_LAUNCHER_DESKTOP", root.join("desktop"))
        .env("HORIZON_NO_AUDIO", "1")
        .env("HORIZON_TELEMETRY_URL", "off")
        // Invalid digest ensures a regression cannot install the dummy payload.
        .env("HORIZON_LAUNCHER_UPDATE_SHA256", "0".repeat(64))
        .env("HORIZON_LAUNCHER_NO_SELF_UPDATE", "1")
        .env_remove("APPIMAGE")
        .env_remove("APPDIR")
        .stdin(Stdio::null())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(std::fs::read(&payload).unwrap(), bytes);
    assert!(!home.join(".launcher-update-failed").exists());
    std::fs::remove_dir_all(&root).unwrap();
}
