use std::process::{exit, Command};
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CompileCommand {
    arguments: Vec<String>,
    directory: PathBuf,
    file: String,
    output: String,
}

#[derive(Debug, Deserialize)]
pub struct CompileCommands(Vec<CompileCommand>);

impl CompileCommands {
    /// exec takes the names of all the .cpp files in compile_commands.json
    /// and creates a g++ command to compile the whole thing, the returned
    /// string slice is the name of the output file
    pub fn exec(&self) -> &str {
        let mut files = Vec::with_capacity(self.0.len());
        
        // add files to vec
        self.0.iter().for_each(|c| files.push(&c.file));

        match Command::new(&self.0[0].arguments[0])
            .args(&files)
            .args(&self.0[0].arguments[2..5])
            .arg(&self.0[0].output)
            .spawn().unwrap()
            .wait() {
                Ok(_) => &self.0[0].output,
                Err(e) => {
                    eprintln!("{}\n\nBuild Failed", e.to_string());
                    exit(1);
                }
            }
    }
}
