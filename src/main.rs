const GREETING: &str = "السلام عليكم ورحمة الله وبركاته!";

fn main() {
    println!("{GREETING}");
    popup(GREETING);
}

/// Native message box via Windows PowerShell (ships with Windows); the text goes in through an environment variable.
#[cfg(windows)]
fn popup(text: &str) {
    use std::process::Command;

    let shown = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Add-Type -AssemblyName PresentationFramework; [void][System.Windows.MessageBox]::Show($env:RUST_LAB_TEXT, 'rust-lab')",
        ])
        .env("RUST_LAB_TEXT", text)
        .status()
        .is_ok_and(|s| s.success());
    if !shown {
        eprintln!("(no desktop session, console only): {text}");
    }
}

/// Native dialog via AppleScript (`osascript` ships with macOS).
#[cfg(target_os = "macos")]
fn popup(text: &str) {
    use std::process::Command;

    let script = format!(
        "display dialog \"{text}\" buttons {{\"OK\"}} default button \"OK\" with title \"rust-lab\""
    );
    let shown = Command::new("osascript")
        .args(["-e", &script])
        .status()
        .is_ok_and(|s| s.success());
    if !shown {
        eprintln!("(no desktop session, console only): {text}");
    }
}

/// GTK `zenity` dialog where available, console fallback otherwise.
#[cfg(target_os = "linux")]
fn popup(text: &str) {
    use std::process::Command;

    let shown = Command::new("zenity")
        .args(["--info", "--title", "rust-lab", "--text", text])
        .status()
        .is_ok_and(|s| s.success());
    if !shown {
        eprintln!("(no desktop session, console only): {text}");
    }
}

/// Any other OS: console only.
#[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
fn popup(text: &str) {
    eprintln!("(console only): {text}");
}

#[cfg(test)]
mod tests {
    use super::GREETING;

    #[test]
    fn greeting_is_the_salaam() {
        assert!(GREETING.starts_with("السلام"));
    }
}
