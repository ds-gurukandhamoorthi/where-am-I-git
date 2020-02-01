extern crate ansi_term;

use std::process::Command;

use ansi_term::Colour;

fn main() {
    let output = Command::new("git").arg("status").output().expect("Failed to retrieve status from Git");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    let status_line = output.lines().next();
    match status_line {
        Some(status) => {
            let status = status.replace("On branch ","").replace("HEAD detached at ", ""); //replace only replaces if it finds any
            let color = if !status.contains("working directory clean") {
                Colour::Red
            }else if status.contains("Your branch is ahead of") {
                Colour::Yellow
            }else if status.contains("nothing to commit"){
                Colour::Green
            }else {
                Colour::Purple
            };
            let status = format!("({})", status);
            println!("{}", color.paint(status));
        },
        _ => println!(),
    }
}
