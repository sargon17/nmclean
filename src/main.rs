use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Confirm, MultiSelect, theme::ColorfulTheme};
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

    Delete {
        #[arg(long, default_value = ".")]
        root: PathBuf,

        #[arg(long)]
        max_depth: Option<usize>,

        /// Skip selection UI and delete all found
        #[arg(long)]
        all: bool,

        /// Preview without deleting
        #[arg(long)]
        dry_run: bool,

        /// Do not ask for final confirmation
        #[arg(long)]
        yes: bool,
    },
}

fn select_paths(items: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let labels: Vec<String> = items.iter().map(|p| p.display().to_string()).collect();

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select node_modules to delete (space to toggle, enter to confirm)")
        .items(&labels)
        .interact()?;

    Ok(selections.into_iter().map(|i| items[i].clone()).collect())
}

fn delete_paths(selected: &[PathBuf]) -> Result<()> {
    for p in selected {
        let md = std::fs::symlink_metadata(p)
            .with_context(|| format!("failed to stat {}", p.display()))?;

        if md.file_type().is_symlink() {
            anyhow::bail!("Refusing to delete symlink: {}", p.display())
        }

        std::fs::remove_dir_all(p).with_context(|| format!("failed to remove {}", p.display()))?;
    }

    Ok(())
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

        Commands::Delete {
            root,
            max_depth,
            all,
            dry_run,
            yes,
        } => {
            let items = scan_node_modules(&root, max_depth)?;

            if items.is_empty() {
                println!("No node_modules found under {}", root.display());
                return Ok(());
            }

            let selected = if all {
                items
            } else {
                let picked = select_paths(&items)?;
                if picked.is_empty() {
                    println!("Nothing selected");
                    return Ok(());
                }
                picked
            };

            println!("Selected {} directories", selected.len());

            for p in &selected {
                println!(" {}", p.display());
            }

            if dry_run {
                println!("Dry run: nothing deleted.");
                return Ok(());
            }

            if !yes {
                let ok = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Proceed with deletion?")
                    .default(false)
                    .interact()?;

                if !ok {
                    println!("Cancelled.");
                    return Ok(());
                }
            }

            delete_paths(&selected)?;
            println!("Done.");
        }
    }

    Ok(())
}
