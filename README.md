# ⚡ Pop: Blazing-Fast File Search Engine 
Terminal-based and Better than Windows Explorer Search :)
Pop is a high-performance, low-memory, terminal-based file and folder search engine built from scratch in Rust. It utilizes pre-built indexing in a SQLite database to deliver instant search results, significantly outperforming live disk scanning.

## ⚓ Myself :)

- **TheCaax** - [TheCaax](https://github.com/TheCaax)

I built Pop to solve my own frustration with the slow and often unreliable search functionality found in standard file explorers. My goal was to create a lightweight, terminal-centric tool that offers instantaneous results through efficient indexing, providing a seamless experience for developers who live in the command line.



## 🚀 Key Features

- **Pre-indexing Model**: Index once, query instantly. No more waiting for slow disk scans.
- **Extreme Performance**: Parallel filesystem scanning with batch database insertions.
- **Advanced Filtering**: Search by name, regex, extension, size expressions, date ranges, and file type.
- **Professional UI**: Clean, colored terminal output with headers, type tags, and highlighted matches.
- **Detailed Summaries**: Get counts of matched files, directories, and total combined size.
- **Low Footprint**: Minimal RAM usage and fast startup times.

## 🛠️ Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed.

```bash
# Clone the repository
git clone <repository-url>
cd pop

# Build for release
cargo build --release

# The binary will be located at ./target/release/pop
```

### Or just donwload the **.exe** file from the releases page.

## 📖 Usage Guide

### Indexing

Before searching, you need to index a directory or a drive.

```bash
# Index the current directory
pop --index .

# Index a specific path
pop --index C:\Users\YourName\Documents

# Force a rebuild of the index
pop --index . --reindex

# Clear the entire index database
pop --clear
```

### Searching

Once indexed, searches are nearly instantaneous.

```bash
# Search by filename
pop --name "config"

# Regex-based search
pop --regex ".*\.rs$"

# Filter by extension
pop --ext rs

# Size-based filtering (supports <, >, and ranges)
pop --size ">10MB"
pop --size "<500KB"
pop --size "1MB-10MB"

# Filter by last modified date (YYYY-MM-DD)
pop --lmd "2023-01-01"

# Filter by type (file or dir)
pop --type file
pop --type dir

# Limit search to a specific subdirectory
pop --path "./src"
```

### Result Control

```bash
# Sort results (name, size, lmd, ext)
pop --name "main" --sort size

# Reverse sort order
pop --name "main" --sort size --reverse

# Limit the number of results
pop --ext log --limit 50

# Enable case-sensitive matching
pop --name "Main" --case_sensitive
```

## 🏗️ Architecture

- **Primary Language**: Rust
- **Database**: SQLite (Disk-backed, optimized for low memory)
- **Concurrency**: Parallel directory walking via `walkdir` and batch processing.
- **CLI Framework**: Clap (Command Line Argument Parser)
- **Persistence**: Index data is stored in `index_entries.db`.

## 🎨 Professional Terminal Look

Pop features a "premium" terminal aesthetics:
- **` DIR `**: White on Blue tag for directories.
- **` FILE `**: Black on Green tag for files.
- **Cyan**: File sizes.
- **Bold White/Blue**: Filenames.
- **Dim**: Paths and timestamps.
- **Highlighted**: Search terms are highlighted within filenames.

---

I try to make it as good as possible for me.
