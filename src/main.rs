//! A parser for YAML files that generates
//!
//! 1. a documentation written in Markdown
//! 2. a `.env` file one can use inside containers to provide environment variables

mod arguments;
mod variables;

use ::anyhow::Context as _;
use ::clap::Parser as _;

fn main() -> ::anyhow::Result<()> {
    let arguments = arguments::Arguments::parse();

    let input = std::fs::read_to_string(&arguments.input_file_path).context(format!(
        "Could not read from file {:?}",
        arguments.input_file_path
    ))?;

    let categories: variables::Categories = serde_yaml::from_str(&input).context(format!(
        "Could not serialize contents in {:?}",
        arguments.input_file_path
    ))?;

    categories.perform_self_check()?;

    println!("Here is the parsed data:\n\n{categories:#?}");

	categories.write_as_markdown(arguments.output_file_path_markdown)?;
	categories.write_as_shell(arguments.output_file_path_shell)?;
    Ok(())
}
