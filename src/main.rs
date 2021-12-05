use std::process::Command;
use std::env;

const COLOR_RED : &str = "\u{001b}[0;31m";
const COLOR_YELLOW : &str = "\u{001b}[0;33m";
const COLOR_GREEN : &str = "\u{001b}[0;32m";
const COLOR_OCHRE : &str = "\u{001b}[0;37m";
const COLOR_RESET : &str = "\u{001b}[0m";

fn main() {
    let type_res = env::args().nth(1).unwrap();
    if type_res == "branch" {
        let output = Command::new("git").arg("rev-parse").arg("--abbrev-ref").arg("HEAD").output().expect("Failed to retrieve branch name from Git");
        let output = String::from_utf8_lossy(output.stdout.as_slice());
        let output = output.replace('\n', "");
        if !output.is_empty(){
            print!("({})", output); //branch-name or commit-id
        }
    } else {
        let output = Command::new("git").arg("status").output().expect("Failed to retrieve status from Git");
        let output = String::from_utf8_lossy(output.stdout.as_slice());
        let text = output.to_string();
        let color = if !text.contains("working tree clean") {
            COLOR_RED
        }else if text.contains("Your branch is ahead of") {
            COLOR_YELLOW
        }else if text.contains("nothing to commit"){
            COLOR_GREEN
        }else {
            COLOR_OCHRE
        };
        print!("{}", color); //branch-name or commit-id
    }
}
