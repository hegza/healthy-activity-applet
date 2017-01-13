use std::process::Command;

fn main() {
    println!("Running command: npm install");
    Command::new("npm").arg("install").status().unwrap();
    println!("Running command: webpack");
    Command::new("webpack").status().unwrap();
}
