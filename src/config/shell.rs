use std::env;
use std::io::{self, Write};
use std::process::{Command, Output};

pub fn configure() {
  println!("Configuring Shells");

  match env::consts::OS {
    "linux" => {
      configure_linux();
    }

    _ => (),
  }
}

fn run(shell: &str, command: &str) -> Output {
  let output = Command::new(shell)
    .arg("-c")
    .arg(command)
    .output()
    .expect("Failed to execute command");

  io::stdout().write_all(&output.stdout).unwrap();

  return output;
}

fn configure_linux() {
  run("sh", "sudo apt install build-essential");

  run("sh", "sudo apt-add-repository ppa:fish-shell/release-3");
  run("sh", "sudo apt update");
  run("sh", "sudo apt install fish");
  // run("sh", "echo /usr/bin/fish | sudo tee -a /etc/shells");
  // run("sh", "chsh -s /usr/bin/fish");

  // run("sh", "curl -sS https://starship.rs/install.sh | sh");
  // run("sh", "starship init fish | source");

  // run(
  //   "sh",
  //   "curl -sL https://git.io/fisher | source && fisher install jorgebucaran/fisher",
  // );

  println!("linux 2");
}
