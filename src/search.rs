use crate::cli::Cli;
use crate::db::Database;
use rusqlite::Result;
use chrono::{TimeZone, Utc};


#[allow(dead_code)]
pub struct SearchResult {
    pub path: String,
    pub name: String,
    pub extension: Option<String>,
    pub size: u64,
    pub last_modified: i64,
    pub is_dir: bool,
}

pub fn execute_search(db: &Database, cli: &Cli) -> Result<Vec<SearchResult>> {
    let mut query = "SELECT path, name, extension, size, last_modified, is_dir FROM files WHERE 1=1".to_string();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(ref name) = cli.name {
        if cli.case_sensitive {
            query.push_str(" AND name LIKE ?");
        } else {
            query.push_str(" AND name LIKE ? COLLATE NOCASE");
        }
        params.push(Box::new(format!("%{}%", name)));
    }

    if let Some(ref ext) = cli.ext {
        query.push_str(" AND extension = ?");
        params.push(Box::new(ext.to_lowercase()));
    }

    if let Some(ref path_limit) = cli.path {
        query.push_str(" AND path LIKE ?");
        params.push(Box::new(format!("{}%", path_limit)));
    }

    if let Some(ref r#type) = cli.r#type {
        query.push_str(" AND is_dir = ?");
        params.push(Box::new(r#type == "dir"));
    }

    if let Some(ref size_expr) = cli.size {
        if let Some((op, bytes)) = parse_size_expression(size_expr) {
            query.push_str(&format!(" AND size {} ?", op));
            params.push(Box::new(bytes));
        }
    }

    if let Some(ref lmd_expr) = cli.lmd {
        if let Some(timestamp) = parse_date_expression(lmd_expr) {
            query.push_str(" AND last_modified >= ?");
            params.push(Box::new(timestamp));
        }
    }

    // Regex is handled post-query because SQLite doesn't have native REGEXP by default
    // However, if we want to be "blazing fast", we should try to filter as much as possible in SQL.

    if let Some(ref sort_field) = cli.sort {
        let field = match sort_field.as_str() {
            "name" => "name",
            "size" => "size",
            "lmd" => "last_modified",
            "ext" => "extension",
            _ => "name",
        };
        query.push_str(&format!(" ORDER BY {}", field));
        if cli.reverse {
            query.push_str(" DESC");
        } else {
            query.push_str(" ASC");
        }
    }

    if let Some(limit) = cli.limit {
        query.push_str(&format!(" LIMIT {}", limit));
    } else {
        query.push_str(" LIMIT 1000"); // Default limit for safety
    }

    let conn = db.get_connection();
    let mut stmt = conn.prepare(&query)?;
    
    let mut results = Vec::new();
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    
    let rows = stmt.query_map(&param_refs[..], |row| {
        Ok(SearchResult {
            path: row.get(0)?,
            name: row.get(1)?,
            extension: row.get(2)?,
            size: row.get(3)?,
            last_modified: row.get(4)?,
            is_dir: row.get(5)?,
        })
    })?;

    for row in rows {
        let res = row?;
        
        // Final regex filtering if needed
        if let Some(ref pattern) = cli.regex {
            let re = match regex::RegexBuilder::new(pattern)
                .case_insensitive(!cli.case_sensitive)
                .build() {
                    Ok(re) => re,
                    Err(_) => continue,
                };
            if !re.is_match(&res.name) {
                continue;
            }
        }
        
        results.push(res);
    }

    Ok(results)
}

fn parse_size_expression(expr: &str) -> Option<(&'static str, i64)> {
    let expr = expr.trim();
    let (op, val_str) = if expr.starts_with('>') {
        (">", &expr[1..])
    } else if expr.starts_with('<') {
        ("<", &expr[1..])
    } else {
        ("=", expr)
    };

    let val_str = val_str.trim();
    let bytes = if val_str.to_uppercase().ends_with("GB") {
        val_str[..val_str.len()-2].parse::<f64>().ok()? * 1024.0 * 1024.0 * 1024.0
    } else if val_str.to_uppercase().ends_with("MB") {
        val_str[..val_str.len()-2].parse::<f64>().ok()? * 1024.0 * 1024.0
    } else if val_str.to_uppercase().ends_with("KB") {
        val_str[..val_str.len()-2].parse::<f64>().ok()? * 1024.0
    } else {
        val_str.parse::<f64>().ok()?
    };

    Some((op, bytes as i64))
}

fn parse_date_expression(expr: &str) -> Option<i64> {
    // Basic YYYY-MM-DD parsing
    let parts: Vec<&str> = expr.split('-').collect();
    if parts.len() == 3 {
        let year = parts[0].parse::<i32>().ok()?;
        let month = parts[1].parse::<u32>().ok()?;
        let day = parts[2].parse::<u32>().ok()?;
        
        let dt = Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).single()?;
        Some(dt.timestamp())
    } else {
        None
    }
}
