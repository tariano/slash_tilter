#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use arboard::Clipboard;
use std::env;

#[cfg(target_os = "windows")]
use msgbox;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Check if help flag is provided
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        show_message_box("Usage", &get_usage_message());
        return Ok(());
    }

    // Validate arguments
    let valid_args = [
        "--quote-if-needed", "-q",
        "--quote-always", "-Q",
        "--no-quote", "-n",
        "--help", "-h"
    ];

    for arg in &args[1..] {
        if !valid_args.contains(&arg.as_str()) {
            show_message_box("Error", &format!("Invalid argument: {}\n\n{}", arg, get_usage_message()));
            return Ok(());
        }
    }

    let current_dir = env::current_dir()?;
    let mut path_str = current_dir.to_str().unwrap().replace('\\', "/");

    // Ensure the path ends with a slash
    if !path_str.ends_with('/') {
        path_str.push('/');
    }

    // Convert "C:/..." to "c/..."
    if let Some((first, rest)) = path_str.split_once(':') {
        let drive = first.chars().next().unwrap().to_lowercase().to_string();
        path_str = format!("/mnt/{}{}", drive, rest);
    } else {
        path_str.insert_str(0, "/mnt/");
    }

    // Check for command-line flags
    let quote_if_needed = args.iter().any(|arg| arg == "--quote-if-needed" || arg == "-q");
    let quote_always = args.iter().any(|arg| arg == "--quote-always" || arg == "-Q");
    let no_quote = args.iter().any(|arg| arg == "--no-quote" || arg == "-n");

    // Apply quoting logic
    if !no_quote {
        if quote_always || (quote_if_needed && path_str.contains(' ')) {
            path_str = format!("\"{}\"", path_str);
        }
    }

    // Copy the resulting path to the clipboard
    Clipboard::new()?.set_text(path_str)?;

    Ok(())
}

// Function to return usage message
fn get_usage_message() -> String {
    "Usage: program [OPTIONS]

Copies the current working directory to the clipboard in WSL-compatible format.

Options:
  -q, --quote-if-needed   Wrap the path in quotes only if it contains spaces.
  -Q, --quote-always      Always wrap the path in quotes.
  -n, --no-quote          Never wrap the path in quotes (overrides other quote options).
  -h, --help              Show this help message and exit."
        .to_string()
}

// Function to show a message box for Windows GUI apps
#[cfg(target_os = "windows")]
fn show_message_box(title: &str, message: &str) {
    msgbox::create(title, message, msgbox::IconType::Info).ok();
}

// No-op for non-Windows platforms
#[cfg(not(target_os = "windows"))]
fn show_message_box(_title: &str, _message: &str) {}
