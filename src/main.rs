use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser, Debug)]
#[command(name = "nmclean", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Scan {
        #[arg(long, default_value = ".")]
        root: PathBuf,

        #[arg(long)]
        max_depth: Option<usize>,
    },
}

fn is_node_modules_dir(e: &DirEntry) -> bool {
    e.file_type().is_dir() && e.file_name() == "node_modules"
}

fn should_enter(e: &DirEntry) -> bool {
    if e.file_name() == "node_modules" {
        return true;
    }

    e.path()
        .components()
        .all(|c| c.as_os_str() != "node_modules")
}

fn scan_node_modules(root: &Path, max_depth: Option<usize>) -> Result<Vec<PathBuf>> {
    let mut wd = WalkDir::new(root).follow_links(false);

    if let Some(d) = max_depth {
        wd = wd.max_depth(d);
    }

    let mut found = Vec::new();

    for entry in wd.into_iter().filter_entry(should_enter) {
        let entry = entry?;

        if is_node_modules_dir(&entry) {
            found.push(entry.path().to_path_buf());
        }
    }

    // let mut it = wd.into_iter();

    // while let Some(entry) = it.next() {
    //     let entry = entry?;

    //     if !should_enter(&entry) {
    //         continue;
    //     }
    //     if is_node_modules_dir(&entry) {
    //         found.push(entry.path().to_path_buf());
    //     }

    //     it.skip_current_dir();
    // }

    found.sort();
    Ok(found)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { root, max_depth } => {
            let items = scan_node_modules(&root, max_depth)?;

            if items.is_empty() {
                println!("No node_modules found under {}", root.display());
                return Ok(());
            }

            for (i, p) in items.iter().enumerate() {
                println!("{:>3}: {}", i + 1, p.display())
            }
        }
    }

    Ok(())
}
