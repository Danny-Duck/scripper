use std::process::{Command, Output};

use clap::{command, Parser};
use regex::Regex;

#[derive(Debug)]
struct Cmd {
    cmd: String,
    args: String,
}

struct CmdErr {
    cmd: String,
    stderr: Vec<u8>,
}

fn handle_error(CmdErr { cmd, stderr }: CmdErr, errors: &mut Vec<CmdErr>) {
    println!("❌ {}", cmd);

    let new_err = CmdErr { cmd, stderr };

    errors.push(new_err);
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None )]
struct Args {
    #[arg(short, long)]
    commands: Vec<String>,
}

fn main() {
    let Args { commands } = Args::parse();
    println!("{:?}", commands);

    let mut errors: Vec<CmdErr> = vec![];

    fn parse_commands(args: Args) -> Vec<Cmd> {
        let mut cmds: Vec<Cmd> = vec![];

        let regex = Regex::new(r"(?P<command>\w*) (?P<args>.*)").unwrap();

        for cmd in args.commands {
            let res = regex.captures(&cmd).unwrap();

            let cmd = Cmd {
                cmd: res["command"].to_string(),
                args: res["args"].to_string(),
            };
            cmds.push(cmd)
        }

        println!("{:?}", cmds);
        cmds
    }

    for Cmd { cmd, args } in parse_commands(Args { commands }) {
        match Command::new(&cmd).arg(args).output() {
            Ok(Output { status, stderr, .. }) => {
                if status.code() > Some(0) {
                    handle_error(CmdErr { cmd, stderr }, &mut errors)
                } else {
                    println!("✅ {}", cmd);
                }
            }
            Err(e) => handle_error(
                CmdErr {
                    cmd,
                    stderr: e.to_string().into(),
                },
                &mut errors,
            ),
        };
    }

    if !errors.is_empty() {
        println!("\n-------------\n")
    }

    for CmdErr { cmd, stderr } in errors.iter() {
        println!("❌ {}", cmd);
        println!("{}", String::from_utf8_lossy(stderr));
        println!("\n");
    }
}

// what I want it to be
// run a single script to run all pre-push checks
// I want each script print its out put then clear it, and only if it fails capture and print the output
