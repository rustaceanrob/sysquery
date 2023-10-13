use std::env;
use sysquery::utils::{self, FileInfo, network, processes};
use utils::{find_largest_files, digest};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[clap(version = "0.0.1", name = "sysquery", author = "Robert Netzke rob.netzke@gmail.com", about = "Gets basic information about the operating system.")]
#[command(propagate_version = true)]
pub struct SysArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Find the largest files in the current working directory.
    Largefiles(LargeFiles),
    /// Get a full system digest.
    Digest,
    /// Get a network I/O digest.
    Network,
    /// Get the current processes expending the most memory. 
    Process(Proccesses),
}

#[derive(Debug, Clone, Args)]
pub struct LargeFiles {
    /// The number of files to return.
    numfiles: Option<u8>,
}

#[derive(Debug, Clone, Args)]
pub struct Proccesses {
    /// The number of most expensive processes to return.
    numprocesses: Option<u8>,
}

fn print_files(files: Vec<FileInfo>) -> () {
    for file in files.iter() {
        let fp = &file.path;
        let sz = file.size as f64 / 1_048_576 as f64;
        let path = fp.to_string_lossy().bright_green();
        let size = format!("{:.2}", sz).bright_yellow();
        print!("The file located at {} is {} millibytes", path, size);
        println!("\n");
    }
}

fn main() {
    let args = SysArgs::parse();

    match args.command {
        Commands::Largefiles(LargeFiles { numfiles }) => {
            match env::current_dir() {
                Ok(dir) => {
                    match numfiles {
                        Some(numfiles) => {
                            println!("\nFinding your largest files...\n");
                            let files = find_largest_files(&dir, numfiles).expect("Some error occured sorting the files.");
                            print_files(files);
                        },
                        None => {
                            println!("\nFinding your largest files...\n");
                            let files = find_largest_files(&dir, 5).expect("Some error occured sorting the files.");
                            print_files(files);
                        },
                    }
                },
                Err(_) => {
                    let error = "Current working directory is unreachable.".bright_red();
                    println!("\n{}\n", error);
                },
            }
            
        },
        Commands::Digest => {
            digest();
        },
        Commands::Network => {
            network();
        },
        Commands::Process(Proccesses { numprocesses }) => {
            match numprocesses {
                Some(n) => {
                    processes(n);
                },
                None => {
                    processes(10)
                },
            }
        },
    }
}
