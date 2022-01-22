mod args;
mod comp_cmd;

use comp_cmd::CompileCommands;
use args::{Args, SubCommand};

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::process::{Command, exit};


use gumdrop::Options;

fn main() { 
    let args = Args::parse_args_default_or_exit();
    let subcmd = match &args.command {
        Some(s) => s.to_owned(),
        None => {
            eprintln!("Subcommand required:\n");
            eprintln!("{}", args.self_command_list().unwrap());
            exit(1);
        }
    };

    let mut current_dir = match std::env::current_dir() {
        Ok(f) => f,
        Err(_) => exit_and_msg("Error looking in current working directory", 1),
    };

    // Looking for compile_commands.json
    current_dir.push("compile_commands.json");

    let mut json_file = match File::open(current_dir) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => exit_and_msg("\"compile_commands.json\" not found", 1),
            ErrorKind::PermissionDenied => exit_and_msg("Permission Denied" ,1),
            _ => exit_and_msg("Unexpected Error occurred while trying to open \"compile_commands.json\"", 1),
        }
    };

    // String to hold contents of json file
    let mut json_contents = String::with_capacity(256);

    // Read file into string
    match json_file.read_to_string(&mut json_contents) {
        Ok(_) => (),
        Err(e) => exit_and_msg(&e.to_string(), 1),
    };

    // Deserialize data into struct 
    let comp_cmds: CompileCommands = serde_json::from_str(&json_contents).unwrap(); 

    let output = comp_cmds.exec();

    match subcmd {
        SubCommand::Build(_) => {},
        SubCommand::Run(opts) => 
            match Command::new(output)
                .args(&opts.free[..])
                .spawn().unwrap()
                .wait() {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Execution Failed:\n{}", e.to_string());
                    exit(1);
                }
        }
    }
}

fn exit_and_msg(msg: &str, code: i32) -> ! {
    eprintln!("{}", msg);
    exit(code)
}
