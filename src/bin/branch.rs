use std::process::Command;


fn main() {
        let output = Command::new("git").arg("rev-parse").arg("--abbrev-ref").arg("HEAD").output().expect("Failed to retrieve branch name from Git");
        let output = String::from_utf8_lossy(output.stdout.as_slice());
        let output = output.replace('\n', "");
        if !output.is_empty(){
            print!("({})", output); //branch-name or commit-id
        }
}
