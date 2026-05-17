#![allow(dead_code)]
mod css_scoper;
mod scaffold;

use clap::{Parser, Subcommand};
use scaffold::ScaffoldFile;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "domius", about = "Domius project CLI")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Domius project
    New {
        #[command(subcommand)]
        what: NewCommands,
    },
    /// Add components/pages to an existing project
    Add {
        #[command(subcommand)]
        what: AddCommands,
    },
}

#[derive(Subcommand)]
enum NewCommands {
    /// Scaffold a new Domius project
    Project { name: String },
}

#[derive(Subcommand)]
enum AddCommands {
    /// Add a new component
    Component { name: String },
    /// Add a new page
    Page { name: String },
}

fn write_files(files: Vec<ScaffoldFile>) {
    for file in files {
        let path = Path::new(&file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Error creating directory {:?}: {}", parent, e);
            });
        }
        fs::write(&file.path, &file.content).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {}", file.path, e);
        });
        println!("  created {}", file.path);
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::New { what } => match what {
            NewCommands::Project { name } => {
                println!("Creating project '{}'...", name);
                write_files(scaffold::new_project(&name));
                println!("Done! cd {} && cargo build", name);
            }
        },
        Commands::Add { what } => match what {
            AddCommands::Component { name } => {
                println!("Adding component '{}'...", name);
                write_files(scaffold::new_component(&name));
            }
            AddCommands::Page { name } => {
                println!("Adding page '{}'...", name);
                write_files(scaffold::new_page(&name));
            }
        },
    }
}
