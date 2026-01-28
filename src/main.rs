mod db;
mod scanner;
mod cli;
mod search;

use clap::Parser;
use cli::Cli;
use db::Database;
use std::path::Path;
use std::time::Instant;
use chrono::{DateTime, Local};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let db_path = "index_entries.db";
    let mut db = Database::new(db_path)?;

    if cli.clear {
        println!("\x1b[31;1mClearing index...\x1b[0m");
        db.clear()?;
        println!("Index cleared successfully.");
        return Ok(());
    }

    if let Some(path_to_index) = cli.index {
        println!("\x1b[34;1mIndexing\x1b[0m \x1b[33m{}\x1b[0m", path_to_index);
        let start = Instant::now();
        if cli.reindex {
            db.clear()?;
        }
        scanner::scan_directory(Path::new(&path_to_index), &mut db)?;
        println!("\x1b[32;1mSuccess!\x1b[0m Indexing completed in \x1b[36m{:?}\x1b[0m", start.elapsed());
        return Ok(());
    }

    if cli.reindex {
        println!("\x1b[31;1mError:\x1b[0m --reindex requires --index <path>");
        return Ok(());
    }

    // Search mode
    let start = Instant::now();
    let results = search::execute_search(&db, &cli)?;
    let duration = start.elapsed();

    display_results(&results, &cli);

    let total_files = results.iter().filter(|r| !r.is_dir).count();
    let total_dirs = results.iter().filter(|r| r.is_dir).count();
    let total_size: u64 = results.iter().map(|r| r.size).sum();

    println!("\x1b[2m────────────────────────────────────────────────────────────────────────────────\x1b[0m");
    println!(
        "\x1b[36;1mFound {} results\x1b[0m in \x1b[35m{:?}\x1b[0m", 
        results.len(), 
        duration
    );
    println!(
        "\x1b[32mFiles: {}\x1b[0m | \x1b[34mDirs: {}\x1b[0m | \x1b[33mTotal Size: {}\x1b[0m",
        total_files,
        total_dirs,
        format_size(total_size)
    );

    Ok(())
}

fn display_results(results: &[search::SearchResult], cli: &Cli) {
    let mut stdout = io::BufWriter::new(io::stdout());
    
    // Header
    writeln!(stdout, "\x1b[1m{:<12} {:<20} {:<30}\x1b[0m", "SIZE", "MODIFIED", "NAME").ok();
    writeln!(stdout, "\x1b[2m────────────────────────────────────────────────────────────────────────────────\x1b[0m").ok();

    for res in results {
        let type_tag = if res.is_dir { 
            "\x1b[44;37m DIR \x1b[0m" // White on Blue
        } else { 
            "\x1b[42;30m FILE \x1b[0m" // Black on Green
        };
        
        let name_color = if res.is_dir { "\x1b[34;1m" } else { "\x1b[37;1m" }; // Bold Blue for dir, Bold White for file
        let reset = "\x1b[0m";
        let dim = "\x1b[2m";
        let cyan = "\x1b[36m";

        let size_str = format_size(res.size);
        let date_str = format_date(res.last_modified);

        // Highlight match if name search was used
        let highlighted_name = if let Some(ref search_name) = cli.name {
            highlight_match(&res.name, search_name)
        } else {
            res.name.clone()
        };

        writeln!(
            stdout,
            "{} {}{:<10}{} {:<20} {}{}{}",
            type_tag, cyan, size_str, reset, date_str, name_color, highlighted_name, reset
        ).ok();
        
        writeln!(stdout, "   {}└─ {}{}", dim, res.path, reset).ok();
    }
    stdout.flush().ok();
}

fn format_size(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    } else if bytes >= 1024 * 1024 {
        format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

fn format_date(timestamp: i64) -> String {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
    let local_dt: DateTime<Local> = DateTime::from(dt);
    local_dt.format("%Y-%m-%d %H:%M").to_string()
}

fn highlight_match(name: &str, pattern: &str) -> String {
    let highlight = "\x1b[43;30m"; // Black text on Yellow background
    let reset = "\x1b[0m";
    
    // Simple case-insensitive replacement for highlighting
    let lower_name = name.to_lowercase();
    let lower_pattern = pattern.to_lowercase();
    
    if let Some(idx) = lower_name.find(&lower_pattern) {
        let mut result = String::new();
        result.push_str(&name[..idx]);
        result.push_str(highlight);
        result.push_str(&name[idx..idx + pattern.len()]);
        result.push_str(reset);
        result.push_str(&name[idx + pattern.len()..]);
        result
    } else {
        name.to_string()
    }
}
