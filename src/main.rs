use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "ansi-senor")]
#[command(about = "Run commands with ANSI color output captured to HTML", long_about = None)]
struct Args {
    /// Output HTML file path (default: /tmp/ansi-senor/command-name-hash.html)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Color theme for HTML output (light or dark)
    #[arg(short, long, default_value = "dark")]
    theme: Theme,

    /// Command to run
    #[arg(required = true, trailing_var_arg = true)]
    command: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    fn background_color(&self) -> &str {
        match self {
            Theme::Light => "#ffffff",
            Theme::Dark => "#1e1e1e",
        }
    }

    fn text_color(&self) -> &str {
        match self {
            Theme::Light => "#24292e",
            Theme::Dark => "#d4d4d4",
        }
    }
}

impl std::str::FromStr for Theme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Theme::Light),
            "dark" => Ok(Theme::Dark),
            _ => Err(format!("Invalid theme '{}'. Valid options: light, dark", s)),
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.command.is_empty() {
        anyhow::bail!("No command specified");
    }

    // Start timing
    let start_time = Instant::now();

    // Run the command and capture output
    let (output_text, exit_status) = run_command_with_capture(&args.command)?;

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Print separator and output with timing
    println!("\n---");
    print!("❯ {}", args.command.join(" "));
    println!("{}", format_duration(elapsed));
    print!("{}", output_text);
    if !output_text.ends_with('\n') {
        println!();
    }
    println!("---\n");

    // Generate output filename (hash based on output content)
    let output_path = generate_output_path(&args.command, &output_text, args.output)?;

    // Ensure output directory exists
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .context(format!("Failed to create directory: {}", parent.display()))?;
    }

    // Convert ANSI to HTML and save
    let html_content =
        ansi_to_html::convert(&output_text).context("Failed to convert ANSI to HTML")?;

    let full_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>
        body {{
            background-color: {};
            color: {};
            font-family: 'Consolas', 'Courier New', monospace;
            padding: 20px;
            margin: 0;
        }}
        pre {{
            white-space: pre-wrap;
            word-wrap: break-word;
        }}
    </style>
</head>
<body>
    <pre>{}</pre>
</body>
</html>"#,
        args.command.join(" "),
        args.theme.background_color(),
        args.theme.text_color(),
        html_content
    );

    fs::write(&output_path, full_html).context(format!(
        "Failed to write output file: {}",
        output_path.display()
    ))?;

    println!("Output saved to {}", output_path.display());

    // Exit with the same status as the command
    std::process::exit(exit_status);
}

fn run_command_with_capture(command: &[String]) -> Result<(String, i32)> {
    let program = &command[0];
    let args = &command[1..];

    let mut child = Command::new(program)
        .args(args)
        .env("CLICOLOR_FORCE", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context(format!("Failed to execute command: {}", program))?;

    let mut output_buffer = Vec::new();

    // Capture stdout
    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.split(b'\n') {
            let line = line.context("Failed to read stdout")?;
            // Print to console
            std::io::stdout().write_all(&line)?;
            if !line.is_empty() || output_buffer.last() != Some(&b'\n') {
                std::io::stdout().write_all(b"\n")?;
            }
            std::io::stdout().flush()?;
            // Save to buffer
            output_buffer.extend_from_slice(&line);
            output_buffer.push(b'\n');
        }
    }

    // Capture stderr
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.split(b'\n') {
            let line = line.context("Failed to read stderr")?;
            // Print to console
            std::io::stderr().write_all(&line)?;
            if !line.is_empty() || output_buffer.last() != Some(&b'\n') {
                std::io::stderr().write_all(b"\n")?;
            }
            std::io::stderr().flush()?;
            // Save to buffer
            output_buffer.extend_from_slice(&line);
            output_buffer.push(b'\n');
        }
    }

    let status = child.wait().context("Failed to wait for command")?;
    let exit_code = status.code().unwrap_or(1);

    let output_text = String::from_utf8_lossy(&output_buffer).to_string();

    Ok((output_text, exit_code))
}

fn generate_output_path(command: &[String], output_text: &str, custom_output: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(path) = custom_output {
        return Ok(path);
    }

    // Generate hash from output content
    let digest = md5::compute(output_text.as_bytes());
    let hash = format!("{:x}", digest);

    // Get the full command for the filename (replace spaces with dashes)
    let command_name = command.join(" ").replace(' ', "-");

    let filename = format!("{}-{}.html", command_name, &hash[..8]);

    // Use system temp directory with ansi-senor subdirectory
    let temp_dir = std::env::temp_dir().join("ansi-senor");
    Ok(temp_dir.join(filename))
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{}s", seconds));
    }

    if parts.is_empty() {
        " took < 1s".to_string()
    } else {
        format!(" took {}", parts.join(""))
    }
}
