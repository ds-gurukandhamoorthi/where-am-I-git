use std::process::Command;


fn main() {
    let output = Command::new("git").arg("status").output().expect("Failed to retrieve status from Git");
    let output = String::from_utf8_lossy(output.stdout.as_slice());
    let text = output.to_string();
    if let Some(branch_info) = text.lines().next(){
        let branch_info = branch_info.replace("On branch ","").replace("HEAD detached at ", ""); //replace only replaces if it finds any
        print!("({})", branch_info); //branch-name or commit-id
    }
}
