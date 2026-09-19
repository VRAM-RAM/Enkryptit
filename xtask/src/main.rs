use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let task = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let release = args.iter().any(|a| a == "--release");

    match task {
        "build" => {
            if release {
                run_cargo(&["build", "--release"]);
            } else {
                run_cargo(&["build"]);

            }
        }

        "install" => {
            run_cargo(&["install", "--path", "./eck"]);
        }

        "test" => {
            run_cargo(&["test", "--test=eck_tests"]);
        }

        "fmt" => {
            run_cargo(&["fmt"]);
        }

        "help" | "--help" | "-h" => {
            print_help();
        }

        "doc" | "rustdoc" | "rsdoc" => {
            run_cargo(&["doc", "--open" , "--document-private-items"]);
        } 
        _ => {
            eprintln!("Unknown task: {task}");
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    eprintln!(
        "\
cargo xtask — dev helpers for Enkryptit!

USAGE:
  cargo xtask <COMMAND> [--release]

BUILD COMMANDS:
  build                  Build eck (Enkryptit!'s binary)
  install                Install eck

TEST:
  test                   Run workspace tests
                                                                
OTHER:
  fmt                    Format code                                                           
                                                        "
    );
}

pub fn run_cargo(args: &[&str]) {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .expect("Unable to run cargo");
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}

