use clap::{App, Arg, SubCommand};
use std::process::Command;

fn main() {
    let matches = App::new("lim")
        .version("1.0")
        .author("Cheng Lim")
        .about("Cheng's command line toolkit")
        .subcommand(
            SubCommand::with_name("g")
                .about("Executes git commands")
                .arg(
                    Arg::with_name("message")
                        .short('m')
                        .long("message")
                        .value_name("MESSAGE")
                        .help("Sets the commit message")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("g") {
        if let Some(message) = matches.value_of("message") {
            // Execute Git commands
            let add_status = Command::new("git")
                .args(&["add", "."])
                .status()
                .expect("Failed to execute git add");

            if add_status.success() {
                let commit_status = Command::new("git")
                    .args(&["commit", "-m", message])
                    .status()
                    .expect("Failed to execute git commit");

                if commit_status.success() {
                    let push_status = Command::new("git")
                        .args(&["push"])
                        .status()
                        .expect("Failed to execute git push");

                    if push_status.success() {
                        println!("Successfully pushed to the repository.");
                    } else {
                        eprintln!("Failed to push to the repository.");
                    }
                } else {
                    eprintln!("Failed to commit changes.");
                }
            } else {
                eprintln!("Failed to add changes.");
            }
        }
    }
}
