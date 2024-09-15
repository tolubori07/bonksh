use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
fn main() {
    loop {
        //we use the → characters as the prompt
        //need to explicitly flush this to ensure it reprints before readline
        print!(">>>>");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        //read line leaves a trailing new line character, the trim function can be used to remove
        //this character; this also should be made peekable for us to know when we're on the last
        //command, the "|" character is commonly used in piping, i have decided, for my own
        //understanding to change it to this: >
        let mut commands = input.trim().split(" > ").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {
            //everything after the first whitespace is interpreted as arguments to the command(e.g
            //say we input command "cd ./foo", the command is "cd" the argument is "./foo")
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    //default to '/' as new directory if one wasn't provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    prev_command = None;
                }

                "exit" => return,
                command => {
                    let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        //there is another command after this one so prepare to send the output to
                        //the next command
                        Stdio::piped()
                    } else {
                        //there are no more commands piped behind this one, send output to shell
                        //stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
                    match output{
                        Ok(output) =>{
                            prev_command = Some(output);
                        }
                        Err(e)=>{
                            prev_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }
        if let Some(mut final_command) = prev_command {
            //block until final command has finished
            final_command.wait().unwrap();
        }
    }
}
