extern crate ansi_term;

use std::process::Command;

use ansi_term::Colour;

fn main() {
    let output = Command::new("git").arg("status").output().expect("Failed to retrieve status from Git");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    let text = output.to_string();
    let color = if !text.contains("working directory clean") {
        Colour::Red
    }else if text.contains("Your branch is ahead of") {
        Colour::Yellow
    }else if text.contains("nothing to commit"){
        Colour::Green
    }else {
        Colour::Purple
    };
    if let Some(branch_info) = text.lines().next(){
        let branch_info = branch_info.replace("On branch ","").replace("HEAD detached at ", ""); //replace only replaces if it finds any
        let branch_info = format!("({})", branch_info);
        println!("{}", color.paint(branch_info)); //branch-name or commit-id
    }
}
