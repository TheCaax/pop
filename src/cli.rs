use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pop", about = "Blazing-fast file indexing and search tool")]
pub struct Cli {
    /// Index a directory or drive
    #[arg(long)]
    pub index: Option<String>,

    /// Force rebuild of index
    #[arg(long)]
    pub reindex: bool,

    /// Clear all entries from the index
    #[arg(long)]
    pub clear: bool,

    /// Partial or full filename match
    #[arg(long)]
    pub name: Option<String>,

    /// Regular-expression-based name search
    #[arg(long)]
    pub regex: Option<String>,

    /// File extension filter
    #[arg(long)]
    pub ext: Option<String>,

    /// File size filter (e.g., >10MB, <500KB, 1MB-10MB)
    #[arg(long)]
    pub size: Option<String>,

    /// Last modified date filter (YYYY-MM-DD or range)
    #[arg(long)]
    pub lmd: Option<String>,

    /// Filter files or directories
    #[arg(long, value_parser = ["file", "dir"])]
    pub r#type: Option<String>,

    /// Limit search to a subdirectory
    #[arg(long)]
    pub path: Option<String>,

    /// Sort results (name|size|lmd|ext)
    #[arg(long, value_parser = ["name", "size", "lmd", "ext"])]
    pub sort: Option<String>,

    /// Reverse sort order
    #[arg(long)]
    pub reverse: bool,

    /// Limit number of results
    #[arg(long)]
    pub limit: Option<usize>,

    /// Disable case-insensitive matching
    #[arg(long)]
    pub case_sensitive: bool,
}
