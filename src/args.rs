use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Args {
    // Help message
    #[options(help = "print help message")]
    help: bool,

    // Subcommand
    #[options(command)]
    pub command: Option<SubCommand>,
}

#[derive(Debug, Options)]
pub enum SubCommand {
    // Build will just build the cpp program, takes no options
    #[options(help = "Build the program")]
    Build(BuildOpts),

    // Builds and runs the program with arguments passed through
    #[options(help = "Build and run the program with arguments passed through")]
    Run(RunOpts),
}

#[derive(Debug, Options)]
pub struct RunOpts {
    // Arguments passed through to the executable
    #[options(free)]
    pub free: Vec<String>,
}

#[derive(Debug, Options)]
pub struct BuildOpts {
    // Arguments passed through to the executable
    #[options(free)]
    free: Vec<String>,
}
