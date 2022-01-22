# cppb
Small program to build and run my c++ homework faster

> Warning! This is a work in progress, use at your own risk

cppb is a simple tool to build and or run my c++ homework from a compile_commands.json. 
I really like cargo an using "cargo run" to build and run my projects so I tried to emulate that as closely as possible.
There are two subcommands `build` and `run`. 
The `build` subcommand requires no extra inputs and will just ingnore any passed.

The `run` subcommand will accept arguments for the resulting executable created by the build process.
So if your executable takes an input and output file as arguments to use cppb you would run:

`$ cppb run -- input.txt output.txt`

and cppb will build your program and then run the executable passing those as arguments.

# Installing
Clone the repository, open the file, and run

`$ cargo build --release`

Then navigate to 

`$ cppb/target/release/`

There should be an executable there that you can move to your path

Enjoy!
