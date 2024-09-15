use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn get_current_working_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => {
            let cwd = path.into_os_string().into_string().unwrap();
            let home_dir = "/Users/moshoodbello"; // Your home directory
            cwd.replace(home_dir, "~") // Replaces /Users/moshoodbello with ~
        }
        Err(_) => "FAILED".to_string(),
    }
}

fn main() {
    print!(
        "
██████╗  ██████╗ ███╗   ██╗██╗  ██╗██╗██╗██╗
██╔══██╗██╔═══██╗████╗  ██║██║ ██╔╝██║██║██║
██████╔╝██║   ██║██╔██╗ ██║█████╔╝ ██║██║██║
██╔══██╗██║   ██║██║╚██╗██║██╔═██╗ ╚═╝╚═╝╚═╝
██████╔╝╚██████╔╝██║ ╚████║██║  ██╗██╗██╗██╗
╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝╚═╝╚═╝
"
    );
    loop {
        let cwd = get_current_working_dir();
        print!("Bonksh {} -> ", cwd);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" > ").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
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
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            prev_command = Some(output);
                        }
                        Err(e) => {
                            prev_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }
        if let Some(mut final_command) = prev_command {
            final_command.wait().unwrap();
        }
    }
}

