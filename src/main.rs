use std::{env, io, process::Command, thread, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    // "opsec" mode (SAFE parody)
    if args.contains(&"--opsec".to_string()) {
        println!("🔒 OPSEC MODE ACTIVATED");
        println!("Simulating secure shutdown...");
        thread::sleep(Duration::from_secs(2));
        println!("Just kidding. Nothing happened. Stay paranoid 😎");
        return Ok(());
    }

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut progress = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(15),
                    Constraint::Length(5),
                    Constraint::Min(5),
                ])
                .split(size);

            // ASCII
            let ascii = Paragraph::new(get_ascii())
                .block(Block::default().borders(Borders::ALL).title("SLOPfetch"));
            f.render_widget(ascii, chunks[0]);

            // Fake training bar
            let gauge = Gauge::default()
                .block(Block::default().title("training on your data...").borders(Borders::ALL))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(progress);
            f.render_widget(gauge, chunks[1]);

            // Info
            let info = Paragraph::new(get_info())
                .block(Block::default().borders(Borders::ALL).title("system??"));
            f.render_widget(info, chunks[2]);
        })?;

        if progress < 100 {
            progress += rand::thread_rng().gen_range(1..5);
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn get_info() -> String {
    format!(
        "user@host: {}@{}\nkernel: {}\nuptime: {}\npublic_ip: {}\nGPU: {}\nRAM: {} MB",
        cmd("whoami"),
        cmd("hostname"),
        cmd("uname -r"),
        cmd("uptime -p"),
        get_ip(),
        fake_gpu(),
        fake_ram()
    )
}

fn cmd(c: &str) -> String {
    Command::new("sh")
        .arg("-c")
        .arg(c)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or("???".into())
}

fn get_ip() -> String {
    reqwest::blocking::get("https://api.ipify.org")
        .and_then(|r| r.text())
        .unwrap_or("???".into())
}

fn fake_gpu() -> String {
    let gpus = [
        "NVIDIA RADEON 30000 Ti ULTRA MAX",
        "INTEL Quantum QUADRO 128GB",
        "Intel HD Graphics",
        "ChatGPT Integrated Graphics",
    ];
    gpus[rand::thread_rng().gen_range(0..gpus.len())].into()
}

fn fake_ram() -> u32 {
    rand::thread_rng().gen_range(1000..64000)
}

fn get_ascii() -> &'static str {
    r#"
     S L O P F E T C H

        made in rust by chatgpt
         [:::::::::::::::]
         [::: 0 ::: 0 :::]
         [::::  ▽  :::::]
         [::: \___/ ::::]
         [:::::::::::::::]
"#
}
