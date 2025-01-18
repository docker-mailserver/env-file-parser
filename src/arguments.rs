//! Contains all structures and functions associated with the arguments the binary
//! received. The heavy lifting is done by [`clap`].

use clap::Parser;

/// Arguments parsed by [`clap`].
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// The path to the input YAML file.
    pub input_file_path: std::path::PathBuf,
    /// The path to the output Markdown file (i.e. the documentation).
    pub output_file_path_markdown: std::path::PathBuf,
    /// The path to the output `*.env` file.
    pub output_file_path_shell: std::path::PathBuf,
}
