use std::process::{Command, Output};

struct Cmd {
    cmd: String,
    args: Vec<String>,
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

fn main() {
    let commands = vec![
        Cmd {
            cmd: "ls".to_string(),
            args: vec!["-la".to_string()],
        },
        Cmd {
            cmd: "pwd".to_string(),
            args: vec![],
        },
        Cmd {
            cmd: "dalksjl".to_string(),
            args: vec![],
        },
    ];

    let mut errors: Vec<CmdErr> = vec![];

    for Cmd { cmd, args } in commands {
        match Command::new(&cmd).args(args).output() {
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

    println!("\n-------------\n");

    for CmdErr { cmd, stderr } in errors.iter() {
        println!("❌ {}", cmd);
        println!("{}", String::from_utf8_lossy(stderr));
        println!("\n");
    }
}

// what I want it to be
// run a single script to run all pre-push checks
// I want each script print its out put then clear it, and only if it fails capture and print the output
